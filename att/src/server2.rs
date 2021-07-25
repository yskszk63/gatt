use std::future::Future;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll, Waker};

use futures::channel::oneshot;
use futures::lock::{Mutex, MutexGuard};
use futures::{FutureExt, Sink, SinkExt, Stream, StreamExt, TryStreamExt};
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

use crate::packet as pkt;
use crate::sock::{AttListener, AttStream};
use crate::Handle;
pub use crate::{ErrorResponse, Handler};
use pkt::pack::{self, Unpack};

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

    fn set_txmtu(&mut self, mtu: usize) {
        let mut buf = vec![0; mtu];
        let len = mtu.min(self.txbuf.len());
        (&mut buf[..len]).copy_from_slice(&self.txbuf[..len]);
        self.txbuf = buf.into();
    }

    fn set_rxmtu(&mut self, mtu: usize) {
        let mut buf = vec![0; mtu];
        let len = mtu.min(self.rxbuf.len());
        (&mut buf[..len]).copy_from_slice(&self.rxbuf[..len]);
        self.rxbuf = buf.into();
    }
}

impl<R> Stream for PacketStream<R>
where
    R: AsyncRead + Unpin,
{
    type Item = Result<pkt::DeviceRecv>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let Self { inner, rxbuf, .. } = self.get_mut();

        let mut buf = ReadBuf::new(rxbuf);
        match Pin::new(inner).poll_read(cx, &mut buf)? {
            Poll::Ready(()) => {}
            Poll::Pending => return Poll::Pending,
        }
        let mut filled = buf.filled();
        if filled.is_empty() {
            Poll::Ready(None)
        } else {
            Poll::Ready(Some(Ok(Unpack::unpack(&mut filled)?)))
        }
    }
}

impl<W, S> Sink<S> for PacketStream<W>
where
    W: AsyncWrite + Unpin,
    S: pkt::DeviceSend,
{
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
        let Self {
            inner,
            txlen,
            txbuf,
            ..
        } = self.get_mut();

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

struct Inner<IO> {
    stream: PacketStream<IO>,
    await_confirmation: Option<oneshot::Sender<()>>,
    // used notification / indication handles
}

impl<IO> Inner<IO> {
    fn new(io: IO) -> Self {
        Self {
            stream: PacketStream::new(io),
            await_confirmation: Default::default(),
        }
    }
}

enum NotificationState {
    Write,
    NeedFlush(usize),
}

struct NotificationInner<IO> {
    handle: Handle,
    inner: Arc<Mutex<Inner<IO>>>,
    state: NotificationState,
}

impl<IO> AsyncWrite for NotificationInner<IO>
where
    IO: AsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let Self {
            handle,
            inner,
            state,
            ..
        } = self.get_mut();
        let mut guard = match inner.lock().poll_unpin(cx) {
            Poll::Ready(guard) => guard,
            Poll::Pending => return Poll::Pending,
        };

        loop {
            match &state {
                NotificationState::Write => {
                    match Sink::<pkt::HandleValueNotificationBorrow>::poll_ready(
                        Pin::new(&mut guard.stream),
                        cx,
                    ) {
                        Poll::Ready(Ok(())) => {}
                        Poll::Ready(Err(err)) => {
                            return Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err)))
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                    let item = pkt::HandleValueNotificationBorrow::new(handle.clone(), buf);
                    match Pin::new(&mut guard.stream).start_send(item) {
                        Ok(()) => *state = NotificationState::NeedFlush(buf.len()),
                        Err(err) => {
                            return Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err)))
                        }
                    }
                }

                NotificationState::NeedFlush(len) => {
                    match Sink::<pkt::HandleValueNotificationBorrow>::poll_flush(
                        Pin::new(&mut guard.stream),
                        cx,
                    ) {
                        Poll::Ready(Ok(())) => {
                            let len = *len;
                            *state = NotificationState::Write;
                            return Poll::Ready(Ok(len));
                        }
                        Poll::Ready(Err(err)) => {
                            return Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err)))
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let Self { inner, .. } = self.get_mut();
        let mut guard = match inner.lock().poll_unpin(cx) {
            Poll::Ready(guard) => guard,
            Poll::Pending => return Poll::Pending,
        };
        match Sink::<pkt::HandleValueNotificationBorrow>::poll_close(
            Pin::new(&mut guard.stream),
            cx,
        ) {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(err)) => Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err))),
            Poll::Pending => Poll::Pending,
        }
    }
}

enum IndicationState {
    Write,
    NeedFlush(usize),
    AwaitConfirmation(usize, oneshot::Receiver<()>),
}

struct IndicationInner<IO> {
    handle: Handle,
    inner: Arc<Mutex<Inner<IO>>>,
    state: IndicationState,
}

impl<IO> AsyncWrite for IndicationInner<IO>
where
    IO: AsyncWrite + Unpin,
{
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let Self {
            state,
            handle,
            inner,
            ..
        } = self.get_mut();
        let mut guard = match inner.lock().poll_unpin(cx) {
            Poll::Ready(guard) => guard,
            Poll::Pending => return Poll::Pending,
        };

        loop {
            match state {
                IndicationState::Write => {
                    match Sink::<pkt::HandleValueIndicationBorrow>::poll_ready(
                        Pin::new(&mut guard.stream),
                        cx,
                    ) {
                        Poll::Ready(Ok(())) => {}
                        Poll::Ready(Err(err)) => {
                            return Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err)))
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                    let item = pkt::HandleValueIndicationBorrow::new(handle.clone(), buf);
                    match Pin::new(&mut guard.stream).start_send(item) {
                        Ok(()) => *state = IndicationState::NeedFlush(buf.len()),
                        Err(err) => {
                            return Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err)))
                        }
                    }
                }

                IndicationState::NeedFlush(len) => {
                    match Sink::<pkt::HandleValueIndicationBorrow>::poll_flush(
                        Pin::new(&mut guard.stream),
                        cx,
                    ) {
                        Poll::Ready(Ok(())) => {
                            let (tx, rx) = oneshot::channel();
                            guard.await_confirmation = Some(tx); // TODO check existence
                            *state = IndicationState::AwaitConfirmation(*len, rx);
                        }
                        Poll::Ready(Err(err)) => {
                            return Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err)))
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }

                IndicationState::AwaitConfirmation(len, rx) => match rx.poll_unpin(cx) {
                    Poll::Ready(Ok(())) => {
                        let len = *len;
                        *state = IndicationState::Write;
                        return Poll::Ready(Ok(len));
                    }
                    Poll::Ready(Err(err)) => {
                        return Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err)))
                    }
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let Self { inner, .. } = self.get_mut();
        let mut guard = match inner.lock().poll_unpin(cx) {
            Poll::Ready(guard) => guard,
            Poll::Pending => return Poll::Pending,
        };
        match Sink::<pkt::HandleValueIndicationBorrow>::poll_close(Pin::new(&mut guard.stream), cx)
        {
            Poll::Ready(Ok(())) => Poll::Ready(Ok(())),
            Poll::Ready(Err(err)) => Poll::Ready(Err(io::Error::new(io::ErrorKind::Other, err))),
            Poll::Pending => Poll::Pending,
        }
    }
}

struct TryLockNext<'a, IO> {
    inner: &'a Mutex<Inner<IO>>,
}

impl<'a, IO> Future for TryLockNext<'a, IO>
where
    IO: AsyncRead + Unpin,
{
    type Output = (MutexGuard<'a, Inner<IO>>, Option<Result<pkt::DeviceRecv>>);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { inner } = self.get_mut();

        let mut guard = match inner.lock().poll_unpin(cx) {
            Poll::Ready(guard) => guard,
            Poll::Pending => return Poll::Pending,
        };

        match guard.stream.poll_next_unpin(cx) {
            Poll::Ready(item) => Poll::Ready((guard, item)),
            Poll::Pending => Poll::Pending,
        }
    }
}

async fn respond<IO, R>(
    stream: &mut PacketStream<IO>,
    r: std::result::Result<R::Response, crate::handler::ErrorResponse>,
) -> Result<()>
where
    IO: AsyncWrite + Unpin,
    R: pkt::Request,
{
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

async fn handle<IO, H>(
    inner: &mut Inner<IO>,
    handler: &mut H,
    request: pkt::DeviceRecv,
) -> Result<()>
where
    IO: AsyncWrite + Unpin,
    H: crate::Handler,
{
    match request {
        pkt::DeviceRecv::ExchangeMtuRequest(item) => {
            let response = handler.handle_exchange_mtu_request(&item);
            if let Ok(response) = &response {
                let client_rx_mtu = *item.client_rx_mtu() as usize;
                let server_rx_mtu = *response.server_rx_mtu() as usize;
                inner.stream.set_txmtu(client_rx_mtu);
                inner.stream.set_rxmtu(server_rx_mtu);
            }
            respond::<_, pkt::ExchangeMtuRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::FindInformationRequest(item) => {
            let response = handler.handle_find_information_request(&item);
            respond::<_, pkt::FindInformationRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::FindByTypeValueRequest(item) => {
            let response = handler.handle_find_by_type_value_request(&item);
            respond::<_, pkt::FindByTypeValueRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::ReadByTypeRequest(item) => {
            let response = handler.handle_read_by_type_request(&item);
            respond::<_, pkt::ReadByTypeRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::ReadRequest(item) => {
            let response = handler.handle_read_request(&item);
            respond::<_, pkt::ReadRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::ReadBlobRequest(item) => {
            let response = handler.handle_read_blob_request(&item);
            respond::<_, pkt::ReadBlobRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::ReadMultipleRequest(item) => {
            let response = handler.handle_read_multiple_request(&item);
            respond::<_, pkt::ReadMultipleRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::ReadByGroupTypeRequest(item) => {
            let response = handler.handle_read_by_group_type_request(&item);
            respond::<_, pkt::ReadByGroupTypeRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::WriteRequest(item) => {
            let response = handler.handle_write_request(&item);
            respond::<_, pkt::WriteRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::WriteCommand(item) => {
            handler.handle_write_command(&item);
        }

        pkt::DeviceRecv::PrepareWriteRequest(item) => {
            let response = handler.handle_prepare_write_request(&item);
            respond::<_, pkt::PrepareWriteRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::ExecuteWriteRequest(item) => {
            let response = handler.handle_execute_write_request(&item);
            respond::<_, pkt::ExecuteWriteRequest>(&mut inner.stream, response).await?;
        }

        pkt::DeviceRecv::SignedWriteCommand(item) => {
            handler.handle_signed_write_command(&item);
        }

        pkt::DeviceRecv::HandleValueConfirmation(..) => {
            if let Some(channel) = inner.await_confirmation.take() {
                channel.send(()).ok();
            }
        }
    }
    Ok(())
}

struct ConnectionInner<IO> {
    inner: Arc<Mutex<Inner<IO>>>,
}

impl<IO> ConnectionInner<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    fn notification(&self, handle: Handle) -> NotificationInner<IO> {
        NotificationInner {
            handle,
            inner: self.inner.clone(),
            state: NotificationState::Write,
        }
    }

    fn indication(&self, handle: Handle) -> IndicationInner<IO> {
        IndicationInner {
            handle,
            inner: self.inner.clone(),
            state: IndicationState::Write,
        }
    }

    async fn run<H>(self, mut handler: H) -> Result<()>
    where
        H: crate::Handler,
    {
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

pub struct Notification {
    inner: NotificationInner<AttStream>,
}

impl AsyncWrite for Notification {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_shutdown(cx)
    }
}

pub struct Indication {
    inner: IndicationInner<AttStream>,
}

impl AsyncWrite for Indication {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.get_mut().inner).poll_write(cx, buf)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_flush(cx)
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.get_mut().inner).poll_shutdown(cx)
    }
}

struct ServerInner<L> {
    inner: L,
}

impl<L, IO> ServerInner<L>
where
    L: Stream<Item = io::Result<IO>> + Unpin,
    IO: AsyncRead + AsyncWrite + Unpin,
{
    async fn accept(&mut self) -> io::Result<Option<ConnectionInner<IO>>> {
        if let Some(sock) = self.inner.try_next().await? {
            return Ok(Some(ConnectionInner {
                inner: Arc::new(Mutex::new(Inner::new(sock))),
            }));
        }
        Ok(None)
    }
}

pub struct Connection {
    inner: ConnectionInner<AttStream>,
}

impl Connection {
    pub fn notification(&self, handle: Handle) -> Notification {
        Notification {
            inner: self.inner.notification(handle),
        }
    }

    pub fn indication(&self, handle: Handle) -> Indication {
        Indication {
            inner: self.inner.indication(handle),
        }
    }

    pub async fn run<H>(self, handler: H) -> Result<()>
    where
        H: crate::Handler,
    {
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
            inner: ServerInner { inner: sock },
        })
    }

    pub fn needs_bond(&self) -> io::Result<()> {
        self.inner
            .inner
            .set_sockopt_bt_security(crate::sock::BT_SECURITY_MEDIUM, 0)
    }

    pub fn needs_bond_mitm(&self) -> io::Result<()> {
        self.inner
            .inner
            .set_sockopt_bt_security(crate::sock::BT_SECURITY_HIGH, 0)
    }

    pub async fn accept(&mut self) -> io::Result<Option<Connection>> {
        if let Some(connection) = self.inner.accept().await? {
            Ok(Some(Connection { inner: connection }))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::{SinkExt, TryStreamExt};
    use std::convert::TryFrom;
    use tokio::io::AsyncWriteExt;
    use tokio_test::io::Builder;

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

    #[tokio::test]
    async fn test_connection() {
        struct H;
        impl Handler for H {}

        let stream = Builder::new()
            .write(&[0x1B, 0x01, 0x00, 0x6F, 0x6B])
            .read(&[0x02, 0x17, 0x00])
            .write(&[0x03, 0x17, 0x00])
            .build();
        let connection = ConnectionInner {
            inner: Arc::new(Mutex::new(Inner::new(stream))),
        };

        let mut notification = connection.notification(Handle::new(1));
        notification.write_all(b"ok").await.unwrap();
        connection.run(H).await.unwrap();
    }

    #[tokio::test]
    async fn test_indication() {
        struct H;
        impl Handler for H {}

        let stream = Builder::new()
            .write(&[0x1D, 0x01, 0x00, 0x6F, 0x6B])
            .read(&[0x1E, 0x17, 0x00])
            .build();
        let connection = ConnectionInner {
            inner: Arc::new(Mutex::new(Inner::new(stream))),
        };

        let mut indication = connection.indication(Handle::new(1));
        let task = tokio::spawn(connection.run(H));

        indication.write_all(b"ok").await.unwrap();

        task.await.unwrap().unwrap();
    }
}
