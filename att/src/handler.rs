use crate::packet as pkt;
use crate::Handle;

/// ATT Protocol Error Response
#[derive(Debug, thiserror::Error)]
#[error("error response {0:?} {1:?}")]
pub struct ErrorResponse(pub(crate) Handle, pub(crate) pkt::ErrorCode);

impl ErrorResponse {
    /// Constract Instance
    pub fn new(handle: Handle, code: pkt::ErrorCode) -> Self {
        Self(handle, code)
    }
}

/// ATT Protocol Handler
pub trait Handler {
    /// handle `exchange mtu request`
    fn handle_exchange_mtu_request(
        &mut self,
        item: &pkt::ExchangeMtuRequest,
    ) -> Result<pkt::ExchangeMtuResponse, ErrorResponse> {
        Ok(pkt::ExchangeMtuResponse::new(*item.client_rx_mtu()))
    }

    /// handle `find information request`
    fn handle_find_information_request(
        &mut self,
        item: &pkt::FindInformationRequest,
    ) -> Result<pkt::FindInformationResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.starting_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    /// handle `find by type value request`
    fn handle_find_by_type_value_request(
        &mut self,
        item: &pkt::FindByTypeValueRequest,
    ) -> Result<pkt::FindByTypeValueResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.starting_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    /// handle `read by type request`
    fn handle_read_by_type_request(
        &mut self,
        item: &pkt::ReadByTypeRequest,
    ) -> Result<pkt::ReadByTypeResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.starting_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    /// handle `read request`
    fn handle_read_request(
        &mut self,
        item: &pkt::ReadRequest,
    ) -> Result<pkt::ReadResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.attribute_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    /// handle `read blob request`
    fn handle_read_blob_request(
        &mut self,
        item: &pkt::ReadBlobRequest,
    ) -> Result<pkt::ReadBlobResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.attribute_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    /// handle `read multiple request`
    fn handle_read_multiple_request(
        &mut self,
        item: &pkt::ReadMultipleRequest,
    ) -> Result<pkt::ReadMultipleResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.into_iter().next().unwrap().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    /// handle `read by group type request`
    fn handle_read_by_group_type_request(
        &mut self,
        item: &pkt::ReadByGroupTypeRequest,
    ) -> Result<pkt::ReadByGroupTypeResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.starting_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    /// handle `write request`
    fn handle_write_request(
        &mut self,
        item: &pkt::WriteRequest,
    ) -> Result<pkt::WriteResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.attribute_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    /// handle `write command`
    #[allow(unused_variables)]
    fn handle_write_command(&mut self, item: &pkt::WriteCommand) {
        // nop
    }

    /// handle `prepare write request`
    fn handle_prepare_write_request(
        &mut self,
        item: &pkt::PrepareWriteRequest,
    ) -> Result<pkt::PrepareWriteResponse, ErrorResponse> {
        Err(ErrorResponse::new(
            item.attribute_handle().clone(),
            pkt::ErrorCode::RequestNotSupported,
        ))
    }

    /// handle `execute write request`
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

    /// handle `signed write command`
    #[allow(unused_variables)]
    fn handle_signed_write_command(&mut self, item: &pkt::SignedWriteCommand) {
        // nop
    }
}


