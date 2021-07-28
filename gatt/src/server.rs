//! GATT Protocol Server
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use att::packet as pkt;
use att::server::{
    Connection as AttConnection, Error as AttError, ErrorResponse, Handler, Indication,
    Notification, Server as AttServer,
};
use att::Handle;
use tokio::sync::mpsc;

use crate::database::Database;
use crate::Registration;

#[derive(Debug)]
struct GattHandler<T> {
    db: Database,
    write_tokens: HashMap<Handle, T>,
    events_txs: Vec<mpsc::UnboundedSender<Event<T>>>,
    authenticated: Arc<AtomicBool>,
}

impl<T> GattHandler<T> {
    fn new(
        db: Database,
        write_tokens: HashMap<Handle, T>,
        events_txs: Vec<mpsc::UnboundedSender<Event<T>>>,
        authenticated: Arc<AtomicBool>,
    ) -> Self {
        Self {
            db,
            write_tokens,
            events_txs,
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
        let r = match self
            .db
            .read(item.attribute_handle(), false, self.authenticated())
        {
            Ok(v) => v,
            Err((h, e)) => return Err(ErrorResponse::new(h, e)),
        };
        let offset = *item.attribute_offset() as usize;
        Ok(pkt::ReadBlobResponse::new(r[offset..].into()))
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
            for tx in &self.events_txs {
                tx.send(Event::Write(token.clone(), value.to_vec().into()))
                    .ok();
            }
        }

        match self.db.write(item.attribute_handle(), value, false, false) {
            Ok(_) => Ok(pkt::WriteResponse::new()),
            Err((h, e)) => Err(ErrorResponse::new(h, e)),
        }
    }

    fn handle_write_command(&mut self, item: &pkt::WriteCommand) {
        let value = item.attribute_value();
        if let Some(token) = self.write_tokens.get(item.attribute_handle()) {
            for tx in &self.events_txs {
                tx.send(Event::Write(token.clone(), value.to_vec().into()))
                    .ok();
            }
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
            for tx in &self.events_txs {
                tx.send(Event::Write(token.clone(), value.to_vec().into()))
                    .ok();
            }
        }

        if let Err(err) =
            self.db
                .write(item.attribute_handle(), item.attribute_value(), false, true)
        {
            log::warn!("{:?}", err);
        };
    }
}

/// Error for [`Control::notify`] | [`Control::indicate`]
#[derive(Debug, thiserror::Error)]
#[error("channel error")]
pub struct ChannelError;

/// GATT Server control.
#[derive(Debug)]
pub struct Authenticator {
    authenticated: Arc<AtomicBool>,
}

impl Authenticator {
    pub fn mark_authenticated(&self) {
        self.authenticated.store(true, Ordering::SeqCst);
    }
}

/// GATT Event
#[derive(Debug)]
pub enum Event<T> {
    Write(T, Box<[u8]>),
}

/// GATT Event Stream
#[derive(Debug)]
pub struct Events<T>(mpsc::UnboundedReceiver<Event<T>>);

impl<T> Events<T> {
    pub async fn next(&mut self) -> Option<Event<T>> {
        self.0.recv().await
    }
}

#[derive(Debug, thiserror::Error)]
#[error("handle not found.")]
pub struct HandleNotFound;

/// Run [`Connection::run`]
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
pub struct RunError(#[from] AttError);

/// GATT Connection
pub struct Connection<T> {
    inner: AttConnection,
    event_txs: Vec<mpsc::UnboundedSender<Event<T>>>,
    db: Database,
    write_tokens: HashMap<Handle, T>,
    notify_or_indicate_handles: HashMap<T, Handle>,
    authenticated: Arc<AtomicBool>, // TODO
}

impl<T> Connection<T>
where
    T: Eq + Hash + Clone,
{
    fn new(inner: AttConnection, registration: Registration<T>) -> Self {
        let (db, write_tokens, notify_or_indicate_handles) = registration.build();

        Self {
            inner,
            event_txs: vec![],
            db,
            write_tokens,
            notify_or_indicate_handles,
            authenticated: Arc::new(AtomicBool::from(false)),
        }
    }

    pub fn authenticator(&self) -> Authenticator {
        Authenticator {
            authenticated: self.authenticated.clone(),
        }
    }

    pub fn events(&mut self) -> Events<T> {
        let (tx, rx) = mpsc::unbounded_channel();
        self.event_txs.push(tx);
        Events(rx)
    }

    pub fn notification(&self, token: &T) -> Result<Notification, HandleNotFound> {
        if let Some(handle) = self.notify_or_indicate_handles.get(token) {
            let notification = self.inner.notification(handle.clone());
            Ok(notification)
        } else {
            Err(HandleNotFound)
        }
    }

    pub fn indication(&self, token: &T) -> Result<Indication, HandleNotFound> {
        if let Some(handle) = self.notify_or_indicate_handles.get(token) {
            let indication = self.inner.indication(handle.clone());
            Ok(indication)
        } else {
            Err(HandleNotFound)
        }
    }

    pub fn address(&self) -> &att::Address {
        &self.inner.address()
    }

    pub async fn run(self) -> Result<(), RunError> {
        let Self {
            db,
            write_tokens,
            event_txs,
            authenticated,
            ..
        } = self;
        self.inner
            .run(GattHandler::<T>::new(
                db,
                write_tokens,
                event_txs,
                authenticated,
            ))
            .await?;
        Ok(())
    }
}

/// GATT Protocol Server
pub struct Server {
    inner: AttServer,
}

impl Server {
    pub fn bind() -> io::Result<Self> {
        let server = AttServer::new()?;
        Ok(Self { inner: server })
    }

    /// Accept [`Connection`]
    pub async fn accept<T>(
        &mut self,
        registration: Registration<T>,
    ) -> io::Result<Option<Connection<T>>>
    where
        T: Eq + Hash + Clone,
    {
        if let Some((connection, _)) = self.inner.accept().await? {
            Ok(Some(Connection::new(connection, registration)))
        } else {
            Ok(None)
        }
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
