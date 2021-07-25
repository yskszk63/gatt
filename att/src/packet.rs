//! ATT Protocol Packet
use std::convert::TryFrom;
use std::fmt;
use std::io;

use derive_new::new as New;
use getset::Getters;

//use crate::pack::{Error as UnpackError, Pack, Unpack};
use crate::size::Size;
use crate::uuid::Uuid16;
use crate::{Handle, Uuid};
use pack::{Error as PackError, Pack, Result as PackResult, Unpack};

#[macro_use]
pub(crate) mod pack;
mod impls;

/// ATT Error Response - Error Code
///
/// see BLUETOOTH CORE SPECIFICATION Version 5.1 |Vol 3, Part F
///     Table 3.3: Error Codes
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    /// Invalid Handle
    InvalidHandle,
    /// Read Not Permitted
    ReadNotPermitted,
    /// Write Not Permitted
    WriteNotPermitted,
    /// Invalid PDU
    InvalidPDU,
    /// Insufficient Authentication
    InsufficientAuthentication,
    /// Request Not Supported
    RequestNotSupported,
    /// Invalid Offset
    InvalidOffset,
    /// Insufficient Authorization
    InsufficientAuthorization,
    /// Prepare Queue Full
    PrepareQueueFull,
    /// Attribute Not Found
    AttributeNotFound,
    /// Attribute Not Long
    AttributeNotLong,
    /// Insufficient Encryption Key Size
    InsufficientEncryptionKeySize,
    /// Invalid Attribute Value Length
    InvalidAttributeValueLength,
    /// Unlikely Error
    UnlikelyError,
    /// Insufficient Encryption
    InsufficientEncryption,
    /// Unsupported Group Type
    UnsupportedGroupType,
    /// Insufficient Resources
    InsufficientResources,
    /// Database Out Of Sync
    DatabaseOutOfSync,
    /// Value Not Allowed
    ValueNotAllowed,
    /// Application Error
    ApplicationError(u8),
    /// Common Profile And Service Error Codes
    CommonProfileAndServiceErrorCodes(u8),
    /// Reserved for Future Use
    ReservedForFutureUse(u8),
}

impl Pack for ErrorCode {
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        let v = match self {
            Self::InvalidHandle => 0x01,
            Self::ReadNotPermitted => 0x02,
            Self::WriteNotPermitted => 0x03,
            Self::InvalidPDU => 0x04,
            Self::InsufficientAuthentication => 0x05,
            Self::RequestNotSupported => 0x06,
            Self::InvalidOffset => 0x07,
            Self::InsufficientAuthorization => 0x08,
            Self::PrepareQueueFull => 0x09,
            Self::AttributeNotFound => 0x0A,
            Self::AttributeNotLong => 0x0B,
            Self::InsufficientEncryptionKeySize => 0x0C,
            Self::InvalidAttributeValueLength => 0x0D,
            Self::UnlikelyError => 0x0E,
            Self::InsufficientEncryption => 0x0F,
            Self::UnsupportedGroupType => 0x10,
            Self::InsufficientResources => 0x11,
            Self::DatabaseOutOfSync => 0x12,
            Self::ValueNotAllowed => 0x13,
            Self::ApplicationError(v)
            | Self::CommonProfileAndServiceErrorCodes(v)
            | Self::ReservedForFutureUse(v) => v,
        };
        u8::pack(v, write)
    }
}

impl Unpack for ErrorCode {
    fn unpack<R>(read: &mut R) -> PackResult<Self>
    where
        R: io::Read,
    {
        Ok(match u8::unpack(read)? {
            0x01 => Self::InvalidHandle,
            0x02 => Self::ReadNotPermitted,
            0x03 => Self::WriteNotPermitted,
            0x04 => Self::InvalidPDU,
            0x05 => Self::InsufficientAuthentication,
            0x06 => Self::RequestNotSupported,
            0x07 => Self::InvalidOffset,
            0x08 => Self::InsufficientAuthorization,
            0x09 => Self::PrepareQueueFull,
            0x0A => Self::AttributeNotFound,
            0x0B => Self::AttributeNotLong,
            0x0C => Self::InsufficientEncryptionKeySize,
            0x0D => Self::InvalidAttributeValueLength,
            0x0E => Self::UnlikelyError,
            0x0F => Self::InsufficientEncryption,
            0x10 => Self::UnsupportedGroupType,
            0x11 => Self::InsufficientResources,
            0x12 => Self::DatabaseOutOfSync,
            0x13 => Self::ValueNotAllowed,
            v if (0x80..=0x9F).contains(&v) => Self::ApplicationError(v),
            v if (0xE0..=0xFF).contains(&v) => Self::CommonProfileAndServiceErrorCodes(v),
            v => Self::ReservedForFutureUse(v),
        })
    }
}

#[derive(Debug)]
struct HandlesInformationList(Vec<(Handle, Handle)>);

impl Pack for HandlesInformationList {
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        pack::RemainingVec(self.0).pack(write)
    }
}

impl Unpack for HandlesInformationList {
    fn unpack<R>(read: &mut R) -> PackResult<Self>
    where
        R: io::Read,
    {
        let v = pack::RemainingVec::<(Handle, Handle)>::unpack(read)?;
        Ok(Self(v.0))
    }
}

#[derive(Debug)]
struct SetOfHandles(Vec<Handle>);

impl<'a> IntoIterator for &'a SetOfHandles {
    type Item = &'a Handle;
    type IntoIter = std::slice::Iter<'a, Handle>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Pack for SetOfHandles {
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        pack::RemainingVec(self.0).pack(write)
    }
}

impl Unpack for SetOfHandles {
    fn unpack<R>(read: &mut R) -> PackResult<Self>
    where
        R: io::Read,
    {
        let v = pack::RemainingVec::<Handle>::unpack(read)?;
        Ok(Self(v.0))
    }
}

#[derive(Debug)]
struct AttributeDataList<T>(Vec<T>);

impl Pack for AttributeDataList<(Handle, Uuid)> {
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        let mut iter = self.0.into_iter();
        let head = if let Some(head) = iter.next() {
            head
        } else {
            return 0u8.pack(write);
        };
        let format = match &head.1 {
            Uuid::Uuid16(_) => 0x01u8,
            Uuid::Uuid128(_) => 0x02u8,
        };

        let pack_data = |(handle, uuid): (Handle, Uuid), write: &mut W| -> PackResult<()> {
            handle.pack(write)?;
            match &uuid {
                Uuid::Uuid16(_) if format != 0x01 => panic!(),
                Uuid::Uuid128(_) if format != 0x02 => panic!(),
                _ => {}
            };
            uuid.pack(write)?;
            Ok(())
        };

        (format).pack(write)?;
        pack_data(head, write)?;
        for data in iter {
            pack_data(data, write)?;
        }
        Ok(())
    }
}

impl Unpack for AttributeDataList<(Handle, Uuid)> {
    fn unpack<R>(read: &mut R) -> PackResult<Self>
    where
        R: io::Read,
    {
        let format = u8::unpack(read)?;

        let mut v = vec![];
        loop {
            let handle = match Handle::unpack(read) {
                Ok(handle) => handle,
                Err(PackError::NoDataAvailable) => break,
                Err(err) => return Err(err),
            };
            let uuid = if format == 0x01 {
                Uuid::Uuid16(Unpack::unpack(read)?)
            } else {
                Uuid::Uuid128(Unpack::unpack(read)?)
            };
            v.push((handle, uuid))
        }
        Ok(Self(v))
    }
}

impl Pack for AttributeDataList<(Handle, Box<[u8]>)> {
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        let mut iter = self.0.into_iter();
        let head = if let Some(head) = iter.next() {
            head
        } else {
            return 0u8.pack(write);
        };
        let len = head.1.len();

        let pack_data = |(handle, data): (Handle, Box<[u8]>), write: &mut W| -> PackResult<()> {
            if data.len() != len {
                panic!()
            }
            handle.pack(write)?;
            data.pack(write)?;
            Ok(())
        };

        ((len + 2) as u8).pack(write)?;
        pack_data(head, write)?;
        for data in iter {
            pack_data(data, write)?;
        }
        Ok(())
    }
}

impl Unpack for AttributeDataList<(Handle, Box<[u8]>)> {
    fn unpack<R>(read: &mut R) -> PackResult<Self>
    where
        R: io::Read,
    {
        let len = u8::unpack(read)? as usize;
        let mut v = vec![];
        loop {
            let handle = match Handle::unpack(read) {
                Ok(handle) => handle,
                Err(PackError::NoDataAvailable) => break,
                Err(err) => return Err(err),
            };
            let mut data = vec![0; len - 2];
            read.read_exact(&mut data)?;
            v.push((handle, data.into()));
        }
        Ok(Self(v))
    }
}

impl Pack for AttributeDataList<(Handle, Handle, Box<[u8]>)> {
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        let mut iter = self.0.into_iter();
        let head = if let Some(head) = iter.next() {
            head
        } else {
            return 0u8.pack(write);
        };
        let len = head.2.len();

        let pack_data = |(handle1, handle2, data): (Handle, Handle, Box<[u8]>),
                         write: &mut W|
         -> PackResult<()> {
            if data.len() != len {
                panic!()
            }
            handle1.pack(write)?;
            handle2.pack(write)?;
            data.pack(write)?;
            Ok(())
        };

        ((len + 4) as u8).pack(write)?;
        pack_data(head, write)?;
        for data in iter {
            pack_data(data, write)?;
        }
        Ok(())
    }
}

impl Unpack for AttributeDataList<(Handle, Handle, Box<[u8]>)> {
    fn unpack<R>(read: &mut R) -> PackResult<Self>
    where
        R: io::Read,
    {
        let len = u8::unpack(read)? as usize;
        let mut v = vec![];
        loop {
            let handle1 = match Handle::unpack(read) {
                Ok(handle) => handle,
                Err(PackError::NoDataAvailable) => break,
                Err(err) => return Err(err),
            };
            let handle2 = Handle::unpack(read)?;
            let mut data = vec![0; len - 4];
            read.read_exact(&mut data)?;
            v.push((handle1, handle2, data.into()));
        }
        Ok(Self(v))
    }
}

macro_rules! packet {
    (
        $(
            $(#[$attrs:meta])*
            $vis:vis struct $name:ident : $op:literal {
                $(
                    $(#[$fattrs:meta])*
                    $fvis:vis $fname:ident : $fty:ty,
                )*
            }
        )*
    ) => {
        $(
            packable_struct! {
                $(#[$attrs])*
                $vis struct $name {
                    $(
                        $(#[$fattrs])*
                        $fvis $fname : $fty,
                    )*
                }
            }

            impl Packet for $name {
                const OPCODE: OpCode = OpCode::$name;
            }
        )*

        packable_enum! {
            /// ATT Op Codes
            #[derive(Debug, PartialEq, Eq, Hash)]
            pub enum OpCode: u8 {
                $($name = $op,)*
            }
        }

        /// ATT Packet
        pub trait Packet: fmt::Debug {
            const OPCODE: OpCode;

            fn opcode() -> OpCode {
                Self::OPCODE
            }
        }

    }
}

packet! {
    /// Error Response
    #[derive(Debug, New, Getters)]
    pub struct ErrorResponse: 0x01 {
        request_opcode_in_error: OpCode,
        attribute_handle_in_error: Handle,
        error_code: ErrorCode,
    }

    /// Exchange MTU Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ExchangeMtuRequest: 0x02 {
        client_rx_mtu: u16,
    }

    /// Exchange MTU Response
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ExchangeMtuResponse: 0x03 {
        server_rx_mtu: u16,
    }

    /// Find Information Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct FindInformationRequest: 0x04 {
        starting_handle: Handle,
        ending_handle: Handle,
    }

    /// Find Information Response
    #[derive(Debug)]
    pub struct FindInformationResponse: 0x05 {
        values: AttributeDataList<(Handle, Uuid)>, // FIXME
    }

    /// Find By Type Value Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct FindByTypeValueRequest: 0x06 {
        starting_handle: Handle,
        ending_handle: Handle,
        attribute_type: Uuid16,
        attribute_value: Box<[u8]>,
    }

    /// Find By Type Value Response
    #[derive(Debug)]
    pub struct FindByTypeValueResponse: 0x07 {
        values: HandlesInformationList,
    }

    /// Read By Type Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadByTypeRequest: 0x08 {
        starting_handle: Handle,
        ending_handle: Handle,
        attribute_type: Uuid,
    }

    /// Read By Type Response
    #[derive(Debug)]
    pub struct ReadByTypeResponse: 0x09 {
        values: AttributeDataList<(Handle, Box<[u8]>)>,
    }

    /// Read Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadRequest: 0x0A {
        attribute_handle: Handle,
    }

    /// Read Response
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadResponse: 0x0B {
        attribute_value: Box<[u8]>,
    }

    /// Read Blob Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadBlobRequest: 0x0C {
        attribute_handle: Handle,
        attribute_offset: u16,
    }

    /// Read Blob Response
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadBlobResponse: 0x0D {
        attribute_value: Box<[u8]>,
    }

    /// Read Multiple Request
    #[derive(Debug)]
    pub struct ReadMultipleRequest: 0x0E {
        set_of_handles: SetOfHandles,
    }

    /// Read Multiple Response
    #[derive(Debug, New)]
    pub struct ReadMultipleResponse: 0x0F {
        set_of_values: Box<[u8]>, // FIXME
    }

    /// Read By Group Type Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadByGroupTypeRequest: 0x10 {
        starting_handle: Handle,
        ending_handle: Handle,
        attribute_group_type: Uuid,
    }

    /// Read By Group Type Response
    #[derive(Debug)]
    pub struct ReadByGroupTypeResponse: 0x11 {
        values: AttributeDataList<(Handle, Handle, Box<[u8]>)>,
    }

    /// Write Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct WriteRequest: 0x12 {
        attribute_handle: Handle,
        attribute_value: Box<[u8]>,
    }

    /// Write Response
    #[derive(Debug, New, Default)]
    pub struct WriteResponse: 0x13 {
    }

    /// Write Command
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct WriteCommand: 0x52 {
        attribute_handle: Handle,
        attribute_value: Box<[u8]>,
    }

    /// Signed Write Command
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct SignedWriteCommand: 0xD2 {
        attribute_handle: Handle,
        attribute_value: Box<[u8]>,
        authentication_signature: Box<[u8]>, // FIXME
    }

    /// Prepare Write Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct PrepareWriteRequest: 0x16 {
        attribute_handle: Handle,
        value_offset: u16,
        part_attribute_value: Box<[u8]>,
    }

    /// Prepare Write Response
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct PrepareWriteResponse: 0x17 {
        attribute_handle: Handle,
        value_offset: u16,
        part_attribute_value: Box<[u8]>,
    }

    /// Execute Write Request
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ExecuteWriteRequest: 0x18 {
        flags: bool,
    }

    /// Execute Write Response
    #[derive(Debug, New, Default)]
    pub struct ExecuteWriteResponse: 0x19 {
    }

    /// Handle Value Notification
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct HandleValueNotification: 0x1B {
        attribute_handle: Handle,
        attribute_value: Box<[u8]>,
    }

    /// Handle Value Indication
    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct HandleValueIndication: 0x1D {
        attribute_handle: Handle,
        attribute_value: Box<[u8]>,
    }

    /// Handle Value Confirmation
    #[derive(Debug, New, Default)]
    pub struct HandleValueConfirmation: 0x1E {
    }

}

macro_rules! device_recv {
    (
        $( $ident:ident, )*
    ) => {
        #[derive(Debug)]
        pub enum DeviceRecv {
            $( $ident($ident), )*
        }

        trait AssertUnpack: Packet + Unpack + Sized {}

        $(
            impl AssertUnpack for $ident {}

            impl From<$ident> for DeviceRecv {
                fn from(v: $ident) -> Self {
                    Self::$ident(v)
                }
            }

            impl TryFrom<DeviceRecv> for $ident {
                type Error = DeviceRecv;
                fn try_from(v: DeviceRecv) -> std::result::Result<Self, Self::Error> {
                    match v {
                        DeviceRecv::$ident(v) => Ok(v),
                        v => Err(v),
                    }
                }
            }
        )*

        impl Unpack for DeviceRecv {
            fn unpack<R>(read: &mut R) -> PackResult<Self> where R: io::Read {
                Ok(match OpCode::unpack(read)? {
                    $( OpCode::$ident => $ident::unpack(read)?.into(), )*
                    unknown => return Err(PackError::Unexpected(format!("{:?}", unknown))),
                })
            }
        }
    }
}

macro_rules! device_send {
    ( $( $ident:ident, )* ) => {
        $(
            impl DeviceSend for $ident { }
        )*
    }
}

device_recv![
    ExchangeMtuRequest,
    FindInformationRequest,
    FindByTypeValueRequest,
    ReadByTypeRequest,
    ReadRequest,
    ReadBlobRequest,
    ReadMultipleRequest,
    ReadByGroupTypeRequest,
    WriteRequest,
    PrepareWriteRequest,
    ExecuteWriteRequest,
    WriteCommand,
    SignedWriteCommand,
    HandleValueConfirmation,
];

device_send![
    ErrorResponse,
    ExchangeMtuResponse,
    FindInformationResponse,
    FindByTypeValueResponse,
    ReadByTypeResponse,
    ReadResponse,
    ReadBlobResponse,
    ReadMultipleResponse,
    ReadByGroupTypeResponse,
    WriteResponse,
    PrepareWriteResponse,
    ExecuteWriteResponse,
    HandleValueNotification,
    HandleValueIndication,
];

pub trait DeviceSend: Packet + Pack + Sized {
    fn pack_with_code<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        Self::OPCODE.pack(write)?;
        self.pack(write)?;
        Ok(())
    }
}

/// ATT Request
pub trait Request: Packet + TryFrom<DeviceRecv> {
    type Response: Response;
}

/// ATT Response
pub trait Response: Packet + DeviceSend {
    // truncate by mtu
    fn truncate(&mut self, mtu: usize);
}

/// ATT Command
pub trait Command: Packet + TryFrom<DeviceRecv> {}

/// ATT Notification
pub trait Notification: Packet + DeviceSend {}

/// ATT Indication
pub trait Indication: Packet + DeviceSend {
    type Confirmation: Confirmation;
}

/// ATT Confirmation
pub trait Confirmation: Packet + TryFrom<DeviceRecv> {}

impl Response for ErrorResponse {
    fn truncate(&mut self, _: usize) {}
}

impl Request for ExchangeMtuRequest {
    type Response = ExchangeMtuResponse;
}
impl Response for ExchangeMtuResponse {
    fn truncate(&mut self, _: usize) {}
}

impl Request for FindInformationRequest {
    type Response = FindInformationResponse;
}
impl Response for FindInformationResponse {
    fn truncate(&mut self, mtu: usize) {
        let mut remaining = mtu - 2;
        let mut len = 0;
        for item in &self.values.0 {
            let item_len = item.size();
            if item_len > remaining {
                break;
            }
            remaining -= item_len;
            len += 1;
        }
        self.values.0.truncate(len);
    }
}

impl Request for FindByTypeValueRequest {
    type Response = FindByTypeValueResponse;
}
impl Response for FindByTypeValueResponse {
    fn truncate(&mut self, mtu: usize) {
        let mut remaining = mtu - 1;
        let mut len = 0;
        for item in &self.values.0 {
            let item_len = item.size();
            if item_len > remaining {
                break;
            }
            remaining -= item_len;
            len += 1;
        }
        self.values.0.truncate(len);
    }
}

impl Request for ReadByTypeRequest {
    type Response = ReadByTypeResponse;
}
impl Response for ReadByTypeResponse {
    fn truncate(&mut self, mtu: usize) {
        let mut remaining = mtu - 2;
        let mut len = 0;
        for item in &self.values.0 {
            let item_len = item.size();
            if item_len > remaining {
                break;
            }
            remaining -= item_len;
            len += 1;
        }
        self.values.0.truncate(len);
    }
}

impl Request for ReadRequest {
    type Response = ReadResponse;
}
impl Response for ReadResponse {
    fn truncate(&mut self, mtu: usize) {
        if self.attribute_value.len() > mtu - 1 {
            self.attribute_value = (&self.attribute_value[..mtu - 1]).into();
        }
    }
}

impl Request for ReadBlobRequest {
    type Response = ReadBlobResponse;
}
impl Response for ReadBlobResponse {
    fn truncate(&mut self, mtu: usize) {
        if self.attribute_value.len() > mtu - 1 {
            self.attribute_value = (&self.attribute_value[..mtu - 1]).into();
        }
    }
}

impl Request for ReadMultipleRequest {
    type Response = ReadMultipleResponse;
}
impl Response for ReadMultipleResponse {
    fn truncate(&mut self, mtu: usize) {
        if self.set_of_values.len() > mtu - 1 {
            self.set_of_values = (&self.set_of_values[..mtu - 1]).into();
        }
    }
}

impl Request for ReadByGroupTypeRequest {
    type Response = ReadByGroupTypeResponse;
}
impl Response for ReadByGroupTypeResponse {
    fn truncate(&mut self, mtu: usize) {
        let mut remaining = mtu - 2;
        let mut len = 0;
        for item in &self.values.0 {
            let item_len = item.size();
            if item_len > remaining {
                break;
            }
            remaining -= item_len;
            len += 1;
        }
        self.values.0.truncate(len);
    }
}

impl Request for WriteRequest {
    type Response = WriteResponse;
}
impl Response for WriteResponse {
    fn truncate(&mut self, _: usize) {}
}

impl Command for WriteCommand {}

impl Request for PrepareWriteRequest {
    type Response = PrepareWriteResponse;
}
impl Response for PrepareWriteResponse {
    fn truncate(&mut self, mtu: usize) {
        if self.part_attribute_value.len() > mtu - 1 {
            self.part_attribute_value = (&self.part_attribute_value[..mtu - 1]).into();
        }
    }
}

impl Request for ExecuteWriteRequest {
    type Response = ExecuteWriteResponse;
}
impl Response for ExecuteWriteResponse {
    fn truncate(&mut self, _: usize) {}
}

impl Notification for HandleValueNotification {}

impl Indication for HandleValueIndication {
    type Confirmation = HandleValueConfirmation;
}
impl Confirmation for HandleValueConfirmation {}

impl Command for SignedWriteCommand {}

/// Handle Value Notification
#[derive(Debug, New)]
pub struct HandleValueNotificationBorrow<'a>(Handle, &'a [u8]);

impl<'a> Packet for HandleValueNotificationBorrow<'a> {
    const OPCODE: OpCode = HandleValueNotification::OPCODE;
}

impl<'a> Pack for HandleValueNotificationBorrow<'a> {
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        self.0.pack(write)?;
        write.write_all(&self.1)?;
        Ok(())
    }
}

impl<'a> DeviceSend for HandleValueNotificationBorrow<'a> {}

impl<'a> Notification for HandleValueNotificationBorrow<'a> {}

/// Handle Value Indication
#[derive(Debug, New)]
pub struct HandleValueIndicationBorrow<'a>(Handle, &'a [u8]);

impl<'a> Packet for HandleValueIndicationBorrow<'a> {
    const OPCODE: OpCode = HandleValueIndication::OPCODE;
}

impl<'a> Pack for HandleValueIndicationBorrow<'a> {
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        self.0.pack(write)?;
        write.write_all(&self.1)?;
        Ok(())
    }
}

impl<'a> DeviceSend for HandleValueIndicationBorrow<'a> {}

impl<'a> Indication for HandleValueNotificationBorrow<'a> {
    type Confirmation = HandleValueConfirmation;
}

#[cfg(test)]
mod tests {
    //use super::*;

    #[test]
    fn test_object_safety() {
        // #[allow(dead_code)]
        // /fn foo(_: &dyn Request<Response = dyn Response>) {}
    }
}
