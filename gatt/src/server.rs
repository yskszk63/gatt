use std::collections::HashMap;
use std::future::Future;
use std::hash::Hash;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use att::packet as pkt;
use att::server::{
    Connection as AttConnection, ErrorResponse, Handler, Outbound, RunError as AttRunError,
    Server as AttServer,
};
use att::Handle;
use bytes::Bytes;
use tokio::sync::mpsc;

use crate::database::Database;
use crate::Registration;

#[derive(Debug)]
struct GattHandler<T> {
    db: Database,
    write_tokens: HashMap<Handle, T>,
    events_tx: mpsc::UnboundedSender<Event<T>>,
    authenticated: Arc<AtomicBool>,
}

impl<T> GattHandler<T> {
    fn new(
        db: Database,
        write_tokens: HashMap<Handle, T>,
        events_tx: mpsc::UnboundedSender<Event<T>>,
        authenticated: Arc<AtomicBool>,
    ) -> Self {
        Self {
            db,
            write_tokens,
            events_tx,
            authenticated,
        }
    }

    fn authenticated(&self) -> bool {
        self.authenticated.load(Ordering::SeqCst)
    }
}

impl<T> Handler for GattHandler<T>
where
    T: Clone,
{
    fn handle_exchange_mtu_request(
        &mut self,
        item: &pkt::ExchangeMtuRequest,
    ) -> Result<pkt::ExchangeMtuResponse, ErrorResponse> {
        Ok(pkt::ExchangeMtuResponse::new(*item.client_rx_mtu()))
    }

    fn handle_find_information_request(
        &mut self,
        item: &pkt::FindInformationRequest,
    ) -> Result<pkt::FindInformationResponse, ErrorResponse> {
        let r = match self
            .db
            .find_information(item.starting_handle().clone()..=item.ending_handle().clone())
        {
            Ok(v) => v,
            Err((h, e)) => return Err(ErrorResponse::new(h, e)),
        };
        Ok(r.into_iter().map(Into::into).collect())
    }

    fn handle_find_by_type_value_request(
        &mut self,
        item: &pkt::FindByTypeValueRequest,
    ) -> Result<pkt::FindByTypeValueResponse, ErrorResponse> {
        let r = match self.db.find_by_type_value(
            item.starting_handle().clone()..=item.ending_handle().clone(),
            item.attribute_type(),
            item.attribute_value(),
            false,
            self.authenticated(),
        ) {
            Ok(v) => v,
            Err((h, e)) => return Err(ErrorResponse::new(h, e)),
        };
        Ok(r.into_iter().map(Into::into).collect())
    }

    fn handle_read_by_type_request(
        &mut self,
        item: &pkt::ReadByTypeRequest,
    ) -> Result<pkt::ReadByTypeResponse, ErrorResponse> {
        let r = match self.db.read_by_type(
            item.starting_handle().clone()..=item.ending_handle().clone(),
            item.attribute_type(),
            false,
            self.authenticated(),
        ) {
            Ok(v) => v,
            Err((h, e)) => return Err(ErrorResponse::new(h, e)),
        };
        Ok(r.into_iter().map(Into::into).collect())
    }

    fn handle_read_request(
        &mut self,
        item: &pkt::ReadRequest,
    ) -> Result<pkt::ReadResponse, ErrorResponse> {
        let r = match self
            .db
            .read(item.attribute_handle(), false, self.authenticated())
        {
            Ok(v) => v,
            Err((h, e)) => return Err(ErrorResponse::new(h, e)),
        };
        Ok(pkt::ReadResponse::new(r))
    }

    fn handle_read_blob_request(
        &mut self,
        item: &pkt::ReadBlobRequest,
    ) -> Result<pkt::ReadBlobResponse, ErrorResponse> {
        let mut r = match self
            .db
            .read(item.attribute_handle(), false, self.authenticated())
        {
            Ok(v) => v,
            Err((h, e)) => return Err(ErrorResponse::new(h, e)),
        };
        let offset = *item.attribute_offset() as usize;
        Ok(pkt::ReadBlobResponse::new(r.split_off(offset)))
    }

    fn handle_read_by_group_type_request(
        &mut self,
        item: &pkt::ReadByGroupTypeRequest,
    ) -> Result<pkt::ReadByGroupTypeResponse, ErrorResponse> {
        let r = match self.db.read_by_group_type(
            item.starting_handle().clone()..=item.ending_handle().clone(),
            item.attribute_group_type(),
            false,
            self.authenticated(),
        ) {
            Ok(v) => v,
            Err((h, e)) => return Err(ErrorResponse::new(h, e)),
        };
        Ok(r.into_iter().map(Into::into).collect())
    }

    fn handle_write_request(
        &mut self,
        item: &pkt::WriteRequest,
    ) -> Result<pkt::WriteResponse, ErrorResponse> {
        let value = item.attribute_value();
        if let Some(token) = self.write_tokens.get(item.attribute_handle()) {
            self.events_tx
                .send(Event::Write(token.clone(), value.to_vec().into()))
                .ok();
        }

        match self.db.write(item.attribute_handle(), value, false, false) {
            Ok(_) => Ok(pkt::WriteResponse::new()),
            Err((h, e)) => Err(ErrorResponse::new(h, e)),
        }
    }

    fn handle_write_command(&mut self, item: &pkt::WriteCommand) {
        let value = item.attribute_value();
        if let Some(token) = self.write_tokens.get(item.attribute_handle()) {
            self.events_tx
                .send(Event::Write(token.clone(), value.to_vec().into()))
                .ok();
        }

        if let Err(err) = self.db.write(
            item.attribute_handle(),
            item.attribute_value(),
            false,
            false,
        ) {
            log::warn!("{:?}", err);
        };
    }

    fn handle_signed_write_command(&mut self, item: &pkt::SignedWriteCommand) {
        let value = item.attribute_value();
        if let Some(token) = self.write_tokens.get(item.attribute_handle()) {
            self.events_tx
                .send(Event::Write(token.clone(), value.to_vec().into()))
                .ok();
        }

        if let Err(err) =
            self.db
                .write(item.attribute_handle(), item.attribute_value(), false, true)
        {
            log::warn!("{:?}", err);
        };
    }
}

#[derive(Debug, thiserror::Error)]
#[error("channel error")]
pub struct ChannelError;

#[derive(Debug)]
pub struct Control<T> {
    inner: Outbound,
    token_map: HashMap<T, Handle>,
    authenticated: Arc<AtomicBool>,
}

impl<T> Control<T> {
    pub fn mark_authenticated(&self) {
        self.authenticated.store(true, Ordering::SeqCst);
    }
}

impl<T> Control<T>
where
    T: Eq + Hash,
{
    pub fn notify<B>(&self, token: &T, val: B) -> Result<(), ChannelError>
    where
        B: Into<Bytes>,
    {
        let handle = self.token_map.get(token).unwrap();
        self.inner
            .notify(handle.clone(), val.into())
            .map_err(|_| ChannelError)?;
        Ok(())
    }

    pub async fn indicate<B>(&self, token: &T, val: B) -> Result<(), ChannelError>
    where
        B: Into<Bytes>,
    {
        let handle = self.token_map.get(token).unwrap();
        self.inner
            .indicate(handle.clone(), val.into())
            .await
            .map_err(|_| ChannelError)?;
        Ok(())
    }
}

#[derive(Debug)]
pub enum Event<T> {
    Write(T, Bytes),
}

#[derive(Debug)]
pub struct Events<T>(mpsc::UnboundedReceiver<Event<T>>);

impl<T> Events<T> {
    pub async fn next(&mut self) -> Option<Event<T>> {
        self.0.recv().await
    }
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct RunError(#[from] AttRunError);

#[derive(Debug)]
pub struct Connection {
    inner: AttConnection,
}

impl Connection {
    pub fn address(&self) -> &att::Address {
        &self.inner.address()
    }

    pub fn run<T>(
        self,
        authenticated: bool,
        registration: Registration<T>,
    ) -> (
        att::Address,
        impl Future<Output = Result<(), RunError>>,
        Control<T>,
        Events<T>,
    )
    where
        T: Hash + Eq + Clone,
    {
        let (db, write_tokens, notify_or_indicate_handles) = registration.build();
        let outgoing = self.inner.outbound();
        let address = self.inner.address().clone();

        let (tx, rx) = mpsc::unbounded_channel();
        let events = Events(rx);

        let authenticated = Arc::new(AtomicBool::new(authenticated));
        let task = self.inner.run(GattHandler::<T>::new(
            db,
            write_tokens,
            tx,
            authenticated.clone(),
        ));
        let task = async move {
            if let Err(e) = task.await {
                Err(e.into())
            } else {
                Ok(())
            }
        };

        (
            address,
            task,
            Control {
                inner: outgoing,
                token_map: notify_or_indicate_handles,
                authenticated,
            },
            events,
        )
    }
}

#[derive(Debug)]
pub struct Server {
    inner: AttServer,
}

impl Server {
    pub fn bind() -> io::Result<Self> {
        let server = AttServer::new()?;
        Ok(Self { inner: server })
    }

    pub async fn accept(&self) -> io::Result<Connection> {
        let connection = self.inner.accept().await?;
        Ok(Connection { inner: connection })
    }

    pub fn needs_bond(&self) -> io::Result<()> {
        self.inner.needs_bond()?;
        Ok(())
    }

    pub fn needs_bond_mitm(&self) -> io::Result<()> {
        self.inner.needs_bond_mitm()?;
        Ok(())
    }
}
