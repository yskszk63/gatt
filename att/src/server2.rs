use std::pin::Pin;
use std::task::{Context, Poll, Waker};
use std::io;
use std::sync::Arc;
use std::future::Future;

use futures::{Stream, Sink, TryStreamExt, SinkExt, FutureExt, StreamExt};
use futures::lock::{Mutex, MutexGuard};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::sock::{AttListener, AttStream};
use crate::packet as pkt;
use pkt::pack::{self, Unpack};
pub use crate::{Handler, ErrorResponse};

const DEFAULT_MTU: usize = 23;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Pack(#[from] pack::Error),
}

type Result<R> = std::result::Result<R, Error>;

struct PacketStream<R> {
    inner: R,
    rxbuf: Box<[u8]>,
    txbuf: Box<[u8]>,
    txlen: usize,
    txwaker: Vec<Waker>,
}

impl<R> PacketStream<R> {
    fn new(inner: R) -> Self {
        Self {
            inner,
            rxbuf: [0; DEFAULT_MTU].into(),
            txbuf: [0; DEFAULT_MTU].into(),
            txlen: 0,
            txwaker: vec![],
        }
    }

    fn txmtu(&self) -> usize {
        self.txbuf.len()
    }
}

impl<R> Stream for PacketStream<R> where R: AsyncRead + Unpin {
    type Item = Result<pkt::DeviceRecv>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self { inner, rxbuf, .. } = self.get_mut();

        let mut buf = ReadBuf::new(rxbuf);
        match Pin::new(inner).poll_read(cx, &mut buf)? {
            Poll::Ready(()) => {}
            Poll::Pending => return Poll::Pending,
        }
        Poll::Ready(Some(Ok(Unpack::unpack(&mut buf.filled())?)))
    }
}

impl<W, S> Sink<S> for PacketStream<W> where W: AsyncWrite + Unpin, S: pkt::DeviceSend {
    type Error = Error;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let Self { txlen, txwaker, .. } = self.get_mut();
        if *txlen != 0 {
            txwaker.push(cx.waker().clone());
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }
    fn start_send(self: Pin<&mut Self>, item: S) -> Result<()> {
        let Self { txlen, txbuf, .. } = self.get_mut();

        let mut write = txbuf.as_mut();
        let len = write.len();
        item.pack_with_code(&mut write)?;
        *txlen = len - write.len();
        Ok(())
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let Self { inner, txlen, txbuf, .. } = self.get_mut();

        while *txlen != 0 {
            match Pin::new(&mut *inner).poll_write(cx, &txbuf[..*txlen])? {
                Poll::Ready(n) => *txlen -= n,
                Poll::Pending => return Poll::Pending,
            }
        }
        match Pin::new(&mut *inner).poll_flush(cx)? {
            Poll::Ready(()) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending,
        }
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();
        match Sink::<S>::poll_flush(Pin::new(this), cx)? {
            Poll::Ready(()) => {}
            Poll::Pending => return Poll::Pending,
        }
        match Pin::new(&mut this.inner).poll_shutdown(cx)? {
            Poll::Ready(()) => Poll::Ready(Ok(())),
            Poll::Pending => Poll::Pending,
        }
    }
}

struct TryLockNext<'a, IO> {
    inner: &'a Mutex<PacketStream<IO>>,
}

impl<'a, IO> Future for TryLockNext<'a, IO> where IO: AsyncRead + Unpin {
    type Output = (MutexGuard<'a, PacketStream<IO>>, Option<Result<pkt::DeviceRecv>>);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { inner } = self.get_mut();

        let mut guard = match inner.lock().poll_unpin(cx) {
            Poll::Ready(guard) => guard,
            Poll::Pending => return Poll::Pending,
        };

        match (*guard).poll_next_unpin(cx) {
            Poll::Ready(item) => Poll::Ready((guard, item)),
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn respond<IO, R>(stream: &mut PacketStream<IO>, r: std::result::Result<R::Response, crate::handler::ErrorResponse>) -> Result<()> where IO: AsyncWrite + Unpin, R: pkt::Request {
    let mtu = stream.txmtu();
    match r {
        Ok(mut r) => {
            pkt::Response::truncate(&mut r, mtu);
            stream.send(r).await?;
        }
        Err(crate::ErrorResponse(handle, code)) => {
            let err = pkt::ErrorResponse::new(R::opcode(), handle, code);
            stream.send(err).await?;
        }
    }
    Ok(())
}

async fn handle<IO, H>(stream: &mut PacketStream<IO>, handler: &mut H, request: pkt::DeviceRecv) -> Result<()> where IO: AsyncWrite + Unpin, H: crate::Handler {
    match request {
        pkt::DeviceRecv::ExchangeMtuRequest(item) => {
            let response = handler.handle_exchange_mtu_request(&item);
            respond::<_, pkt::ExchangeMtuRequest>(stream, response).await?;
            // TODO grow mtu
        }

        pkt::DeviceRecv::FindInformationRequest(item) => {
            let response = handler.handle_find_information_request(&item);
            respond::<_, pkt::FindInformationRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::FindByTypeValueRequest(item) => {
            let response = handler.handle_find_by_type_value_request(&item);
            respond::<_, pkt::FindByTypeValueRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::ReadByTypeRequest(item) => {
            let response = handler.handle_read_by_type_request(&item);
            respond::<_, pkt::ReadByTypeRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::ReadRequest(item) => {
            let response = handler.handle_read_request(&item);
            respond::<_, pkt::ReadRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::ReadBlobRequest(item) => {
            let response = handler.handle_read_blob_request(&item);
            respond::<_, pkt::ReadBlobRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::ReadMultipleRequest(item) => {
            let response = handler.handle_read_multiple_request(&item);
            respond::<_, pkt::ReadMultipleRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::ReadByGroupTypeRequest(item) => {
            let response = handler.handle_read_by_group_type_request(&item);
            respond::<_, pkt::ReadByGroupTypeRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::WriteRequest(item) => {
            let response = handler.handle_write_request(&item);
            respond::<_, pkt::WriteRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::WriteCommand(item) => {
            handler.handle_write_command(&item);
        }

        pkt::DeviceRecv::PrepareWriteRequest(item) => {
            let response = handler.handle_prepare_write_request(&item);
            respond::<_, pkt::PrepareWriteRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::ExecuteWriteRequest(item) => {
            let response = handler.handle_execute_write_request(&item);
            respond::<_, pkt::ExecuteWriteRequest>(stream, response).await?;
        }

        pkt::DeviceRecv::SignedWriteCommand(item) => {
            handler.handle_signed_write_command(&item);
        }

        pkt::DeviceRecv::HandleValueConfirmation(_item) => {
            todo!()
        }
    }
    Ok(())
}

struct ConnectionInner<IO> {
    inner: Arc<Mutex<PacketStream<IO>>>,
}

impl<IO> ConnectionInner<IO> where IO: AsyncRead + AsyncWrite + Unpin {
    async fn run<H>(&self, mut handler: H) -> Result<()> where H: crate::Handler {
        loop {
            let (mut guard, request) = TryLockNext { inner: &self.inner }.await;
            let request = if let Some(request) = request {
                request?
            } else {
                return Ok(());
            };

            handle(&mut *guard, &mut handler, request).await?;
        }
    }
}

struct ServerInner<L> {
    inner: L,
}

impl<L, IO> ServerInner<L> where L: Stream<Item = io::Result<IO>> + Unpin, IO: AsyncRead + AsyncWrite + Unpin {
    async fn accept(&mut self) -> io::Result<Option<ConnectionInner<IO>>> {
        if let Some(sock) = self.inner.try_next().await? {
            return Ok(Some(ConnectionInner {
                inner: Arc::new(Mutex::new(PacketStream::new(sock))),
            }));
        }
        Ok(None)
    }
}

pub struct Connection {
    inner: ConnectionInner<AttStream>,
}

impl Connection {
    pub async fn run<H>(&self, handler: H) -> Result<()> where H: crate::Handler {
        self.inner.run(handler).await?;
        Ok(())
    }
}

pub struct Server {
    inner: ServerInner<AttListener>,
}

impl Server {
    /// Constract Instance.
    pub fn new() -> io::Result<Self> {
        let sock = AttListener::new()?;
        Ok(Self {
            inner: ServerInner {
                inner: sock,
            }
        })
    }

    pub fn needs_bond(&self) -> io::Result<()> {
        self.inner.inner
            .set_sockopt_bt_security(crate::sock::BT_SECURITY_MEDIUM, 0)
    }

    pub fn needs_bond_mitm(&self) -> io::Result<()> {
        self.inner.inner
            .set_sockopt_bt_security(crate::sock::BT_SECURITY_HIGH, 0)
    }

    pub async fn accept(&mut self) -> io::Result<Option<Connection>> {
        if let Some(connection) = self.inner.accept().await? {
            Ok(Some(Connection {
                inner: connection,
            }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test::io::Builder;
    use futures::{TryStreamExt, SinkExt};
    use std::convert::TryFrom;

    #[tokio::test]
    async fn test_stream() {
        let stream = Builder::new()
            .read(&[0x02, 0x17, 0x00])
            .write(&[0x03, 0x18, 0x00])
            .build();
        let mut stream = PacketStream::new(stream);
        let packet = stream.try_next().await.unwrap().unwrap();
        let packet = pkt::ExchangeMtuRequest::try_from(packet).unwrap();
        assert_eq!(*packet.client_rx_mtu(), 23);

        let packet = pkt::ExchangeMtuResponse::new(0x0018);
        stream.send(packet).await.unwrap();
    }
}
