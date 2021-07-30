#![doc(html_root_url = "https://docs.rs/att/0.2.0")]
//! Bluetooth Low Energy Attribute Protocol Library.
//!
//! ref BLUETOOTH CORE SPECIFICATION Version 5.1 | Vol 3, Part F
//!     Attribute Protocol (ATT)
//!
//! ## Example
//!
//! ```
//! use att::packet as pkt;
//! use att::server::*;
//!
//! /// GAP / GATT Service only (with no Characteristics)
//! #[derive(Debug)]
//! struct MyHandler;
//!
//! impl Handler for MyHandler {
//!     fn handle_read_by_group_type_request(
//!         &mut self,
//!         item: &pkt::ReadByGroupTypeRequest,
//!     ) -> Result<pkt::ReadByGroupTypeResponse, ErrorResponse> {
//!         match (
//!             item.starting_handle().clone().into(),
//!             item.ending_handle().clone().into(),
//!         ) {
//!             (0x0001, 0xFFFF) => {
//!                 Ok(vec![
//!                     (0x0001.into(), 0x000B.into(), vec![0x00, 0x18].into()), // Generic Access
//!                     (0x000C.into(), 0x000F.into(), vec![0x01, 0x18].into()), // Generic Attribute
//!                 ]
//!                 .into_iter()
//!                 .collect())
//!             }
//!             (x, _) => Err(ErrorResponse::new(
//!                 x.clone().into(),
//!                 pkt::ErrorCode::AttributeNotFound,
//!             )),
//!         }
//!     }
//! }
//!
//! #[tokio::main(flavor = "current_thread")]
//! async fn main() -> anyhow::Result<()> {
//!     let server = Server::new()?;
//!     // let connection = server.accept().await?;
//!     // connection.run(MyHandler).await?;
//!     Ok(())
//! }
//! ```
//!
//! # Supported target
//!
//! - x86_64-unknown-linux-gnu
//!
//! Tested on Linux 5.9 (Arch Linux)
//!
//! ## License
//!
//! Licensed under either of
//! * Apache License, Version 2.0
//!   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
//! * MIT license
//!   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted
//! for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
//! dual licensed as above, without any additional terms or conditions.!

pub use crate::uuid::Uuid;
pub use bdaddr::Address;
pub use handle::Handle;
pub use handler::{ErrorResponse, Handler};
pub use server::Server;

#[macro_use]
mod macros;

mod handle;
mod handler;
pub mod packet;
pub mod server;
mod size;
mod sock;
pub mod uuid;
