use att::packet as pkt;
use att::server::*;

/// GAP / GATT Service only (with no Characteristics)
#[derive(Debug)]
struct MyHandler;

impl Handler for MyHandler {
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
                    (0x0001.into(), 0x0001.into(), vec![0x00, 0x18].into()), // Generic Access
                    (0x0002.into(), 0x0002.into(), vec![0x01, 0x18].into()), // Generic Attribute
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
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let server = Server::new()?;
    let connection = server.accept().await?;
    connection.run(MyHandler).await?;
    Ok(())
}
