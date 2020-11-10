use std::io::stdin;

use tokio::task::spawn_blocking;

use att::packet as pkt;
use att::server::*;

struct H;

#[allow(unused_variables)]
impl Handler for H {
    fn handle_exchange_mtu_request(
        &mut self,
        item: &pkt::ExchangeMtuRequest,
    ) -> RequestResult<pkt::ExchangeMtuResponse> {
        Err(anyhow::anyhow!("OK"))?;
        Ok(pkt::ExchangeMtuResponse::new(item.client_rx_mtu().clone()))
    }

    fn handle_find_information_request(
        &mut self,
        item: &pkt::FindInformationRequest,
    ) -> RequestResult<pkt::FindInformationResponse> {
        match (
            item.starting_handle().clone().into(),
            item.ending_handle().clone().into(),
        ) {
            (0x000F, 0x000F) => {
                Ok(vec![
                    (0x000F.into(), pkt::Uuid16::from(0x2902).into()), // Client Characteristic Configuration
                ]
                .into_iter()
                .collect())
            }
            (0x0026, 0x0027) => {
                Ok(vec![
                    (0x0026.into(), pkt::Uuid16::from(0x2902).into()), // Client Characteristic Configuration
                ]
                .into_iter()
                .collect())
            }
            (0x0027, 0x0027) => {
                Ok(vec![
                    (0x0027.into(), pkt::Uuid16::from(0x2904).into()), // Characteristic Presentation Format Descriptor
                ]
                .into_iter()
                .collect())
            }
            (x, _) => Err((x.clone().into(), pkt::ErrorCode::AttributeNotFound).into()),
        }
    }

    fn handle_find_by_type_value_request(
        &mut self,
        item: &pkt::FindByTypeValueRequest,
    ) -> RequestResult<pkt::FindByTypeValueResponse> {
        todo!()
    }

    fn handle_read_by_type_request(
        &mut self,
        item: &pkt::ReadByTypeRequest,
    ) -> RequestResult<pkt::ReadByTypeResponse> {
        match (
            item.starting_handle().clone().into(),
            item.ending_handle().clone().into(),
            item.attribute_type(),
        ) {
            (0x0001, 0x000B, pkt::Uuid::Uuid16(uuid)) if u16::from(uuid.clone()) == 0x2803 => {
                Ok(vec![
                    (0x0002.into(), vec![0x08, 0x04, 0x00, 0x00, 0x2A].into()), // Generic Access / Device Name
                    (0x0003.into(), vec![0x02, 0x05, 0x00, 0x01, 0x2A].into()), // Generic Access / Appearance
                ]
                .into_iter()
                .collect())
            }
            (0x000C, 0x000F, pkt::Uuid::Uuid16(uuid)) if u16::from(uuid.clone()) == 0x2803 => {
                Ok(vec![
                    (0x000D.into(), vec![0x20, 0x0E, 0x00, 0x05, 0x2A].into()), // Generic Attribute / ServiceChanged
                ]
                .into_iter()
                .collect())
            }
            (0x0010, 0x0022, pkt::Uuid::Uuid16(uuid)) if u16::from(uuid.clone()) == 0x2803 => {
                Ok(vec![
                    (0x0011.into(), vec![0x02, 0x12, 0x00, 0x29, 0x2A].into()), // Device Information / Manufacture Name String
                    (0x0013.into(), vec![0x02, 0x14, 0x00, 0x24, 0x2A].into()), // Device Information / Model Number String
                    (0x0015.into(), vec![0x02, 0x16, 0x00, 0x25, 0x2A].into()), // Device Information / Serial Number String
                ]
                .into_iter()
                .collect())
            }
            (0x0023, 0x0027, pkt::Uuid::Uuid16(uuid)) if u16::from(uuid.clone()) == 0x2803 => {
                Ok(vec![
                    (0x0024.into(), vec![0x12, 0x25, 0x00, 0x19, 0x2A].into()), // Battery Service / Battery Level
                ]
                .into_iter()
                .collect())
            }
            (x, _, _) => Err((x.into(), pkt::ErrorCode::AttributeNotFound).into()),
        }
    }

    fn handle_read_request(&mut self, item: &pkt::ReadRequest) -> RequestResult<pkt::ReadResponse> {
        match item.attribute_handle().clone().into() {
            0x0005 => Ok(pkt::ReadResponse::new(vec![0x00].into())),
            _ => Ok(pkt::ReadResponse::new(vec![0x00].into())),
            //x => Err((x.into(), pkt::ErrorCode::AttributeNotFound).into())
        }
    }

    fn handle_read_blob_request(
        &mut self,
        item: &pkt::ReadBlobRequest,
    ) -> RequestResult<pkt::ReadBlobResponse> {
        todo!()
    }

    fn handle_read_multiple_request(
        &mut self,
        item: &pkt::ReadMultipleRequest,
    ) -> RequestResult<pkt::ReadMultipleResponse> {
        todo!()
    }

    fn handle_read_by_group_type_request(
        &mut self,
        item: &pkt::ReadByGroupTypeRequest,
    ) -> RequestResult<pkt::ReadByGroupTypeResponse> {
        match (
            item.starting_handle().clone().into(),
            item.ending_handle().clone().into(),
        ) {
            (0x0001, 0xFFFF) => {
                Ok(vec![
                    (0x0001.into(), 0x000B.into(), vec![0x00, 0x18].into()), // Generic Access
                    (0x000C.into(), 0x000F.into(), vec![0x01, 0x18].into()), // Generic Attribute
                    (0x0010.into(), 0x0022.into(), vec![0x0A, 0x18].into()), // Device Information
                ]
                .into_iter()
                .collect())
            }
            (0x0023, 0xFFFF) => {
                Ok(vec![
                    (0x0023.into(), 0x0027.into(), vec![0x0F, 0x18].into()), // Battery Service
                ]
                .into_iter()
                .collect())
            }
            (x, _) => Err((x.into(), pkt::ErrorCode::AttributeNotFound).into()),
        }
    }

    fn handle_write_request(
        &mut self,
        item: &pkt::WriteRequest,
    ) -> RequestResult<pkt::WriteResponse> {
        Ok(pkt::WriteResponse::default())
    }

    fn handle_write_command(&mut self, item: &pkt::WriteCommand) -> CommandResult<()> {
        todo!()
    }

    fn handle_prepare_write_request(
        &mut self,
        item: &pkt::PrepareWriteRequest,
    ) -> RequestResult<pkt::PrepareWriteResponse> {
        todo!()
    }

    fn handle_execute_write_request(
        &mut self,
        item: &pkt::ExecuteWriteRequest,
    ) -> RequestResult<pkt::ExecuteWriteResponse> {
        todo!()
    }

    fn handle_signed_write_command(&mut self, item: &pkt::SignedWriteCommand) -> CommandResult<()> {
        todo!()
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let server = Server::new()?;
    let connection = server.accept().await?;
    let outbound = connection.outbound();

    let mut task = tokio::spawn(connection.run(H));

    let mut n = 0;
    loop {
        tokio::select! {
            result = std::pin::Pin::new(&mut task) => {
                result??;
            }

            maybe_line = spawn_blocking(|| stdin().read_line(&mut String::new())) => {
                maybe_line??;
                outbound.notify(0x0025.into(), vec![n].into())?;
                outbound.indicate(0x000E.into(), vec![0x0C, 0x00, 0x0F, 0x00].into()).await?; // GATT / Service Changed
                n += 1;
            }
        }
    }
}
