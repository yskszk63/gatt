use std::collections::VecDeque;
use std::io;

use bytes::{Bytes, BytesMut};
use tokio::select;
use tokio::sync::{mpsc, oneshot};

use crate::pack::{Error as UnpackError, Pack, Unpack};
use crate::packet as pkt;
use crate::packet::Response;
use crate::sock::{AttListener, AttStream};
use crate::Handle;

#[derive(Debug, thiserror::Error)]
#[error("error response {0:?} {1:?}")]
pub struct ErrorResponse(Handle, pkt::ErrorCode);

impl ErrorResponse {
    pub fn new(handle: Handle, code: pkt::ErrorCode) -> Self {
        Self(handle, code)
    }
}

pub trait Handler {
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
        Err(ErrorResponse::new(
            item.starting_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    fn handle_find_by_type_value_request(
        &mut self,
        item: &pkt::FindByTypeValueRequest,
    ) -> Result<pkt::FindByTypeValueResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.starting_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    fn handle_read_by_type_request(
        &mut self,
        item: &pkt::ReadByTypeRequest,
    ) -> Result<pkt::ReadByTypeResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.starting_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    fn handle_read_request(
        &mut self,
        item: &pkt::ReadRequest,
    ) -> Result<pkt::ReadResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.attribute_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    fn handle_read_blob_request(
        &mut self,
        item: &pkt::ReadBlobRequest,
    ) -> Result<pkt::ReadBlobResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.attribute_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    fn handle_read_multiple_request(
        &mut self,
        item: &pkt::ReadMultipleRequest,
    ) -> Result<pkt::ReadMultipleResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.into_iter().next().unwrap().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    fn handle_read_by_group_type_request(
        &mut self,
        item: &pkt::ReadByGroupTypeRequest,
    ) -> Result<pkt::ReadByGroupTypeResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.starting_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    fn handle_write_request(
        &mut self,
        item: &pkt::WriteRequest,
    ) -> Result<pkt::WriteResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.attribute_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    #[allow(unused_variables)]
    fn handle_write_command(&mut self, item: &pkt::WriteCommand) {
        // nop
    }

    fn handle_prepare_write_request(
        &mut self,
        item: &pkt::PrepareWriteRequest,
    ) -> Result<pkt::PrepareWriteResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.attribute_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    #[allow(unused_variables)]
    fn handle_execute_write_request(
        &mut self,
        item: &pkt::ExecuteWriteRequest,
    ) -> Result<pkt::ExecuteWriteResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            0x0000.into(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    #[allow(unused_variables)]
    fn handle_signed_write_command(&mut self, item: &pkt::SignedWriteCommand) {
        // nop
    }
}

#[derive(Debug)]
enum OutgoingMessage {
    Notification(pkt::HandleValueNotification),
    Indication(pkt::HandleValueIndication, oneshot::Sender<()>),
}

#[derive(Debug, thiserror::Error)]
pub enum NotifyError {
    #[error("task maybe dropped.")]
    Closed,
}

#[derive(Debug, thiserror::Error)]
pub enum IndicateError {
    #[error("task maybe dropped.")]
    Closed,
}

#[derive(Debug, Clone)]
pub struct Outbound {
    tx: mpsc::UnboundedSender<OutgoingMessage>,
}

impl Outbound {
    pub fn notify(&self, handle: Handle, value: Bytes) -> Result<(), NotifyError> {
        self.tx
            .send(OutgoingMessage::Notification(
                pkt::HandleValueNotification::new(handle, value),
            ))
            .map_err(|_| NotifyError::Closed)?;
        Ok(())
    }

    pub async fn indicate(&self, handle: Handle, value: Bytes) -> Result<(), IndicateError> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(OutgoingMessage::Indication(
                pkt::HandleValueIndication::new(handle, value),
                tx,
            ))
            .map_err(|_| IndicateError::Closed)?;
        rx.await.map_err(|_| IndicateError::Closed)?;
        Ok(())
    }
}

const DEFAULT_MTU: usize = 23;

#[derive(Debug, thiserror::Error)]
pub enum RunError {
    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    Unpack(#[from] UnpackError),
}

#[derive(Debug)]
pub struct Connection {
    tx: mpsc::UnboundedSender<OutgoingMessage>,
    rx: mpsc::UnboundedReceiver<OutgoingMessage>,
    sock: AttStream,
    addr: crate::Address,
}

async fn recv(sock: &AttStream, buf: &mut Vec<u8>) -> Result<pkt::Packet, RunError> {
    let len = sock.recv(buf).await?;
    let packet = pkt::Packet::unpack(&mut &buf[..len])?;
    log::debug!("<< {:?}", packet);
    Ok(packet)
}

async fn send(sock: &AttStream, buf: &mut BytesMut, packet: pkt::Packet) -> Result<(), RunError> {
    log::debug!(">> {:?}", packet);
    packet.pack(buf);
    sock.send(buf).await?;
    buf.clear();
    Ok(())
}

fn response<R>(r: Result<R::Response, ErrorResponse>, mtu: usize) -> pkt::Packet
where
    R: pkt::Request + pkt::HasOpCode,
    R::Response: Into<pkt::Packet>,
{
    match r {
        Ok(mut r) => {
            r.truncate(mtu);
            r.into()
        }
        Err(ErrorResponse(handle, code)) => {
            pkt::ErrorResponse::new(R::opcode(), handle, code).into()
        }
    }
}

impl Connection {
    pub fn address(&self) -> &crate::Address {
        &self.addr
    }

    pub fn outbound(&self) -> Outbound {
        Outbound {
            tx: self.tx.clone(),
        }
    }

    pub async fn run<H>(self, mut handler: H) -> Result<(), RunError>
    where
        H: Handler,
    {
        let Self { sock, mut rx, .. } = self;
        let mut mtu = DEFAULT_MTU;
        let mut rbuf = vec![0; DEFAULT_MTU];
        let mut wbuf = BytesMut::with_capacity(DEFAULT_MTU);
        let mut await_confirmations = VecDeque::new();

        loop {
            select! {
                message = rx.recv() => {
                    match message {
                        Some(OutgoingMessage::Notification(packet)) => {
                            send(&sock, &mut wbuf, packet.into()).await?;
                        }
                        Some(OutgoingMessage::Indication(packet, reply)) => {
                            await_confirmations.push_back(reply);
                            send(&sock, &mut wbuf, packet.into()).await?;
                        }
                        None => {},
                    }
                }

                maybe_packet = recv(&sock, &mut rbuf) => {
                    match maybe_packet? {
                        pkt::Packet::ExchangeMtuRequest(item) => {
                            let response = response::<pkt::ExchangeMtuRequest>(
                                handler.handle_exchange_mtu_request(&item), mtu);
                            if let pkt::Packet::ExchangeMtuResponse(response) = &response {
                                let client_rx_mtu = *item.client_rx_mtu() as usize;
                                let server_rx_mtu = *response.server_rx_mtu() as usize;
                                mtu = server_rx_mtu;
                                rbuf = vec![0; server_rx_mtu];
                                wbuf = BytesMut::with_capacity(client_rx_mtu);
                            };
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::FindInformationRequest(item) => {
                            let response = response::<pkt::FindInformationRequest>(
                                handler.handle_find_information_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::FindByTypeValueRequest(item) => {
                            let response = response::<pkt::FindByTypeValueRequest>(
                                handler.handle_find_by_type_value_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::ReadByTypeRequest(item) => {
                            let response = response::<pkt::ReadByTypeRequest>(
                                handler.handle_read_by_type_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::ReadRequest(item) => {
                            let response = response::<pkt::ReadRequest>(
                                handler.handle_read_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::ReadBlobRequest(item) => {
                            let response = response::<pkt::ReadBlobRequest>(
                                handler.handle_read_blob_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::ReadMultipleRequest(item) => {
                            let response = response::<pkt::ReadMultipleRequest>(
                                handler.handle_read_multiple_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::ReadByGroupTypeRequest(item) => {
                            let response = response::<pkt::ReadByGroupTypeRequest>(
                                handler.handle_read_by_group_type_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::WriteRequest(item) => {
                            let response = response::<pkt::WriteRequest>(
                                handler.handle_write_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::WriteCommand(item) => handler.handle_write_command(&item),

                        pkt::Packet::PrepareWriteRequest(item) => {
                            let response = response::<pkt::PrepareWriteRequest>(
                                handler.handle_prepare_write_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::ExecuteWriteRequest(item) => {
                            let response = response::<pkt::ExecuteWriteRequest>(
                                handler.handle_execute_write_request(&item), mtu);
                            send(&sock, &mut wbuf, response).await?;
                        }

                        pkt::Packet::HandleValueConfirmation(..) => {
                            if let Some(reply) = await_confirmations.pop_front() {
                                reply.send(()).ok();
                            }
                        }

                        pkt::Packet::SignedWriteCommand(item) => handler.handle_signed_write_command(&item),

                        e => unreachable!("{:?}", e),
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Server {
    sock: AttListener,
}

impl Server {
    pub fn new() -> io::Result<Self> {
        let sock = AttListener::new()?;
        Ok(Self { sock })
    }

    pub fn needs_bond(&self) -> io::Result<()> {
        self.sock
            .set_sockopt_bt_security(crate::sock::BT_SECURITY_MEDIUM, 0)
    }

    pub fn needs_bond_mitm(&self) -> io::Result<()> {
        self.sock
            .set_sockopt_bt_security(crate::sock::BT_SECURITY_HIGH, 0)
    }

    pub async fn accept(&self) -> io::Result<Connection> {
        let (sock, addr) = self.sock.accept().await?;
        let (tx, rx) = mpsc::unbounded_channel();
        Ok(Connection { sock, addr, tx, rx })
    }
}
