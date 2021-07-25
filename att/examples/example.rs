use std::io::stdin;

use tokio::io::AsyncWriteExt;
use tokio::task::spawn_blocking;

use att::packet as pkt;
use att::server::*;
use att::Uuid;

struct H;

/*
 * 0x0001 Uuid16(0x1800) Generic Access
 * 0x0002 Write, 0x0004, Uuid16(0x2A00) Device Name
 * 0x0003 Read, 0x0005, Uuid16(0x2A01) Appearance
 * 0x0004 ...
 * 0x0005 ...
 * 0x000C Uuid16(0x1801) Generic Attribute
 * 0x000D Indication, 0x000E, Uuid16(0x2A05) ServiceChanged
 * 0x000E ...
 * 0x000F Uuid16(0x2902) Client Characteristic Configuration
 * 0x0010 Uuid16(0x180A) Device Information
 * 0x0011 Read, 0x0012, Uuid16(0x2A29) Manufacture Name String
 * 0x0012 ...
 * 0x0013 Read, 0x0014, Uuid16(0x2A24) Model Number String
 * 0x0014 ...
 * 0x0015 Read, 0x0016, Uuid16(0x2A25) Serial Number String
 * 0x0016 ...
 * 0x0023 Uuid16(0x180F) Battery Service
 * 0x0024 Read | Notification, 0x0025, Uuid16(0x2A19) Battery Level
 * 0x0025 ...
 * 0x0026 Uuid16(0x2902) Client Characteristic Configuration
 * 0x0027 Uuid16(0x2904) Client Presentation Format Descriptor
 */

impl Handler for H {
    fn handle_find_information_request(
        &mut self,
        item: &pkt::FindInformationRequest,
    ) -> Result<pkt::FindInformationResponse, ErrorResponse> {
        match (
            item.starting_handle().clone().into(),
            item.ending_handle().clone().into(),
        ) {
            (0x000F, 0x000F) => {
                Ok(vec![
                    (0x000F.into(), Uuid::new_uuid16(0x2902).into()), // Client Characteristic Configuration
                ]
                .into_iter()
                .collect())
            }
            (0x0026, 0x0027) => {
                Ok(vec![
                    (0x0026.into(), Uuid::new_uuid16(0x2902).into()), // Client Characteristic Configuration
                ]
                .into_iter()
                .collect())
            }
            (0x0027, 0x0027) => {
                Ok(vec![
                    (0x0027.into(), Uuid::new_uuid16(0x2904).into()), // Characteristic Presentation Format Descriptor
                ]
                .into_iter()
                .collect())
            }
            (x, _) => Err(ErrorResponse::new(
                x.clone().into(),
                pkt::ErrorCode::AttributeNotFound,
            )),
        }
    }

    fn handle_read_by_type_request(
        &mut self,
        item: &pkt::ReadByTypeRequest,
    ) -> Result<pkt::ReadByTypeResponse, ErrorResponse> {
        match (
            item.starting_handle().clone().into(),
            item.ending_handle().clone().into(),
            item.attribute_type(),
        ) {
            (0x0001, 0x000B, Uuid::Uuid16(uuid)) if u16::from(uuid.clone()) == 0x2803 => {
                Ok(vec![
                    (0x0002.into(), vec![0x08, 0x04, 0x00, 0x00, 0x2A].into()), // Generic Access / Device Name
                    (0x0003.into(), vec![0x02, 0x05, 0x00, 0x01, 0x2A].into()), // Generic Access / Appearance
                ]
                .into_iter()
                .collect())
            }
            (0x000C, 0x000F, Uuid::Uuid16(uuid)) if u16::from(uuid.clone()) == 0x2803 => {
                Ok(vec![
                    (0x000D.into(), vec![0x20, 0x0E, 0x00, 0x05, 0x2A].into()), // Generic Attribute / ServiceChanged
                ]
                .into_iter()
                .collect())
            }
            (0x0010, 0x0022, Uuid::Uuid16(uuid)) if u16::from(uuid.clone()) == 0x2803 => {
                Ok(vec![
                    (0x0011.into(), vec![0x02, 0x12, 0x00, 0x29, 0x2A].into()), // Device Information / Manufacture Name String
                    (0x0013.into(), vec![0x02, 0x14, 0x00, 0x24, 0x2A].into()), // Device Information / Model Number String
                    (0x0015.into(), vec![0x02, 0x16, 0x00, 0x25, 0x2A].into()), // Device Information / Serial Number String
                ]
                .into_iter()
                .collect())
            }
            (0x0023, 0x0027, Uuid::Uuid16(uuid)) if u16::from(uuid.clone()) == 0x2803 => {
                Ok(vec![
                    (0x0024.into(), vec![0x12, 0x25, 0x00, 0x19, 0x2A].into()), // Battery Service / Battery Level
                ]
                .into_iter()
                .collect())
            }
            (x, _, _) => Err(ErrorResponse::new(
                x.clone().into(),
                pkt::ErrorCode::AttributeNotFound,
            )),
        }
    }

    fn handle_read_request(
        &mut self,
        item: &pkt::ReadRequest,
    ) -> Result<pkt::ReadResponse, ErrorResponse> {
        match item.attribute_handle().clone().into() {
            0x0005 => Ok(pkt::ReadResponse::new(vec![0x00].into())),
            _ => Ok(pkt::ReadResponse::new(vec![0x00].into())),
            //x => Err((x.into(), pkt::ErrorCode::AttributeNotFound).into())
        }
    }

    fn handle_read_by_group_type_request(
        &mut self,
        item: &pkt::ReadByGroupTypeRequest,
    ) -> Result<pkt::ReadByGroupTypeResponse, ErrorResponse> {
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
            (x, _) => Err(ErrorResponse::new(
                x.clone().into(),
                pkt::ErrorCode::AttributeNotFound,
            )),
        }
    }

    fn handle_write_request(
        &mut self,
        _item: &pkt::WriteRequest,
    ) -> Result<pkt::WriteResponse, ErrorResponse> {
        Ok(pkt::WriteResponse::default())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    // 1. ensure bluez stop
    // 2. start advertising
    //   - btmgmt power on
    //   - btmgmt connectable yes
    //   - btmgmt discov yes
    //   - btmgmt advertising on
    // 3. run this
    //   - cargo run --example example

    pretty_env_logger::init();

    let mut server = Server::new()?;
    //server.needs_bond_mitm()?;
    let connection = server.accept().await?.unwrap();
    let mut notification = connection.notification(0x0025.into());
    let mut indication = connection.indication(0x000E.into());

    let mut task = tokio::spawn(connection.run(H));

    let mut n = 0;
    loop {
        tokio::select! {
            result = std::pin::Pin::new(&mut task) => {
                result??;
            }

            maybe_line = spawn_blocking(|| stdin().read_line(&mut String::new())) => {
                maybe_line??;
                notification.write_all(&[n]).await?;
                indication.write_all(&[0x0C, 0x00, 0x0F, 0x00]).await?;
                n += 1;
            }
        }
    }
}
