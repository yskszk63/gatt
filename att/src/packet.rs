use std::convert::TryFrom;
use std::fmt;

use bytes::{Buf, Bytes, BytesMut};
use derive_new::new as New;
use getset::Getters;
pub use uuid::Uuid as Uuid128;

use crate::pack::{Error as UnpackError, Pack, Unpack};
use crate::size::Size;

mod impl_from_iter;

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    InvalidHandle,
    ReadNotPermitted,
    WriteNotPermitted,
    InvalidPDU,
    InsufficientAuthentication,
    RequestNotSupported,
    InvalidOffset,
    InsufficientAuthorization,
    PrepareQueueFull,
    AttributeNotFound,
    AttributeNotLong,
    InsufficientEncryptionKeySize,
    InvalidAttributeValueLength,
    UnlikelyError,
    InsufficientEncryption,
    UnsupportedGroupType,
    InsufficientResources,
    DatabaseOutOfSync,
    ValueNotAllowed,
    ApplicationError(u8),
    CommonProfileAndServiceErrorCodes(u8),
    ReservedForFutureUse(u8),
}

impl Pack for ErrorCode {
    fn pack(self, buf: &mut BytesMut) {
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
        u8::pack(v, buf);
    }
}

impl Unpack for ErrorCode {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        Ok(match u8::unpack(buf)? {
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

packable_newtype! {
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Handle(u16);
}

impl Handle {
    pub const fn new(v: u16) -> Self {
        Self(v)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

impl From<u16> for Handle {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl From<Handle> for u16 {
    fn from(v: Handle) -> Self {
        v.0
    }
}

packable_newtype! {
    #[derive(Clone, PartialEq, Eq)]
    pub struct Uuid16(u16);
}

impl Uuid16 {
    pub const fn new(v: u16) -> Self {
        Self(v)
    }

    pub fn as_u16(&self) -> u16 {
        self.0
    }
}

impl From<u16> for Uuid16 {
    fn from(v: u16) -> Self {
        Self(v)
    }
}

impl From<Uuid16> for u16 {
    fn from(v: Uuid16) -> Self {
        v.0
    }
}

impl fmt::Debug for Uuid16 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:04X}", self.0)
    }
}

impl From<Uuid16> for Uuid {
    fn from(v: Uuid16) -> Self {
        Self::Uuid16(v)
    }
}

impl Pack for Uuid128 {
    fn pack(self, buf: &mut BytesMut) {
        self.to_u128_le().pack(buf);
    }
}

impl Unpack for Uuid128 {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        Ok(Self::from_u128_le(Unpack::unpack(buf)?))
    }
}

impl From<Uuid128> for Uuid {
    fn from(v: Uuid128) -> Self {
        Self::Uuid128(v)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Uuid {
    Uuid16(Uuid16),
    Uuid128(Uuid128),
}

impl Uuid {
    pub const fn new_uuid16(v: u16) -> Self {
        Self::Uuid16(Uuid16::new(v))
    }

    pub const fn new_uuid128(v: u128) -> Self {
        Self::Uuid128(Uuid128::from_u128(v))
    }
}

impl Pack for Uuid {
    fn pack(self, buf: &mut BytesMut) {
        match self {
            Self::Uuid16(uuid) => uuid.pack(buf),
            Self::Uuid128(uuid) => uuid.pack(buf),
        }
    }
}

impl Unpack for Uuid {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        match buf.remaining() {
            2 => Ok(Self::Uuid16(Unpack::unpack(buf)?)),
            16 => Ok(Self::Uuid128(Unpack::unpack(buf)?)),
            v => Err(UnpackError::Unexpected(format!("unexpected length {}", v))),
        }
    }
}

#[derive(Debug)]
pub struct HandlesInformationList(Vec<(Handle, Handle)>);

impl Pack for HandlesInformationList {
    fn pack(self, buf: &mut BytesMut) {
        for item in self.0 {
            item.0.pack(buf);
            item.1.pack(buf);
        }
    }
}

impl Unpack for HandlesInformationList {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let mut v = vec![];
        while buf.has_remaining() {
            v.push((Unpack::unpack(buf)?, Unpack::unpack(buf)?))
        }
        Ok(Self(v))
    }
}

#[derive(Debug)]
pub struct SetOfHandles(Vec<Handle>);

impl<'a> IntoIterator for &'a SetOfHandles {
    type Item = &'a Handle;
    type IntoIter = std::slice::Iter<'a, Handle>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Pack for SetOfHandles {
    fn pack(self, buf: &mut BytesMut) {
        for item in self.0 {
            item.pack(buf);
        }
    }
}

impl Unpack for SetOfHandles {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let mut v = vec![];
        while buf.has_remaining() {
            v.push(Unpack::unpack(buf)?)
        }
        Ok(Self(v))
    }
}

#[derive(Debug)]
pub struct AttributeDataList<T>(Vec<T>);

impl Pack for AttributeDataList<(Handle, Uuid)> {
    fn pack(self, buf: &mut BytesMut) {
        let mut iter = self.0.into_iter();
        let head = if let Some(head) = iter.next() {
            head
        } else {
            (0 as u8).pack(buf);
            return;
        };
        let format = match &head.1 {
            Uuid::Uuid16(_) => 0x01u8,
            Uuid::Uuid128(_) => 0x02u8,
        };

        let pack_data = |data: (Handle, Uuid), buf: &mut BytesMut| {
            data.0.pack(buf);
            match &data.1 {
                Uuid::Uuid16(_) if format != 0x01 => panic!(),
                Uuid::Uuid128(_) if format != 0x02 => panic!(),
                _ => {}
            };
            data.1.pack(buf);
        };

        (format).pack(buf);
        pack_data(head, buf);
        for data in iter {
            pack_data(data, buf);
        }
    }
}

impl Unpack for AttributeDataList<(Handle, Uuid)> {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let format = u8::unpack(buf)?;
        let len = match format {
            0x01 => 4,
            0x02 => 24,
            v => return Err(UnpackError::Unexpected(format!("unexpected format {}", v))),
        };
        Ok(Self(
            (0..buf.remaining() / len)
                .map(|_| {
                    Ok((
                        Unpack::unpack(buf)?,
                        match format {
                            0x01 => Uuid::Uuid16(Unpack::unpack(buf)?),
                            0x02 => Uuid::Uuid128(Unpack::unpack(buf)?),
                            x => unreachable!(x),
                        },
                    ))
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Pack for AttributeDataList<(Handle, Bytes)> {
    fn pack(self, buf: &mut BytesMut) {
        let mut iter = self.0.into_iter();
        let head = if let Some(head) = iter.next() {
            head
        } else {
            (0 as u8).pack(buf);
            return;
        };
        let len = head.1.len();

        let pack_data = |data: (Handle, Bytes), buf: &mut BytesMut| {
            data.0.pack(buf);
            if data.1.len() != len {
                panic!()
            }
            buf.extend_from_slice(&data.1);
        };

        ((len + 2) as u8).pack(buf);
        pack_data(head, buf);
        for data in iter {
            pack_data(data, buf);
        }
    }
}

impl Unpack for AttributeDataList<(Handle, Bytes)> {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let len = u8::unpack(buf)? as usize;
        Ok(Self(
            (0..buf.remaining() / len)
                .map(|_| Ok((Unpack::unpack(buf)?, buf.copy_to_bytes(len - 2))))
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Pack for AttributeDataList<(Handle, Handle, Bytes)> {
    fn pack(self, buf: &mut BytesMut) {
        let mut iter = self.0.into_iter();
        let head = if let Some(head) = iter.next() {
            head
        } else {
            (0 as u8).pack(buf);
            return;
        };
        let len = head.2.len();

        let pack_data = |data: (Handle, Handle, Bytes), buf: &mut BytesMut| {
            data.0.pack(buf);
            data.1.pack(buf);
            if data.2.len() != len {
                panic!()
            }
            buf.extend_from_slice(&data.2);
        };

        ((len + 4) as u8).pack(buf);
        pack_data(head, buf);
        for data in iter {
            pack_data(data, buf);
        }
    }
}

impl Unpack for AttributeDataList<(Handle, Handle, Bytes)> {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
        let len = u8::unpack(buf)? as usize;
        Ok(Self(
            (0..buf.remaining() / len)
                .map(|_| {
                    Ok((
                        Unpack::unpack(buf)?,
                        Unpack::unpack(buf)?,
                        buf.copy_to_bytes(len - 4),
                    ))
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("failed to convert from packet")]
pub struct TryFromPacketError;

mod seal {
    pub trait Sealed {}
}

pub trait HasOpCode {
    fn opcode() -> OpCode;
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

            impl $name {
                const OPCODE: OpCode = OpCode::$name;
            }

            impl HasOpCode for $name {
                fn opcode() -> OpCode {
                    Self::OPCODE
                }
            }

            impl seal::Sealed for $name {}

            impl From<$name> for Packet {
                fn from(v: $name) -> Self {
                    Self::$name(v)
                }
            }

            impl TryFrom<Packet> for $name {
                type Error = TryFromPacketError;
                fn try_from(v: Packet) -> Result<Self, Self::Error> {
                    match v {
                        Packet::$name(v) => Ok(v),
                        _ => Err(TryFromPacketError),
                    }
                }
            }
        )*

        packable_enum! {
            #[derive(Debug, PartialEq, Eq, Hash)]
            pub enum OpCode: u8 {
                $($name => $op,)*
            }
        }

        #[derive(Debug)]
        pub enum Packet {
            $($name($name),)*
        }

        impl Pack for Packet {
            fn pack(self, buf: &mut BytesMut) {
                match self {
                    $(
                        Self::$name(v) => {
                            $name::OPCODE.pack(buf);
                            v.pack(buf);
                        }
                    )*
                }
            }
        }

        impl Unpack for Packet {
            fn unpack<B: Buf>(buf: &mut B) -> Result<Self, UnpackError> {
                let opcode = OpCode::unpack(buf)?;
                match opcode {
                    $(
                        OpCode::$name => Ok($name::unpack(buf)?.into()),
                    )*
                }
            }
        }

    }
}

packet! {
    #[derive(Debug, New, Getters)]
    pub struct ErrorResponse: 0x01 {
        request_opcode_in_error: OpCode,
        attribute_handle_in_error: Handle,
        error_code: ErrorCode,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ExchangeMtuRequest: 0x02 {
        client_rx_mtu: u16,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ExchangeMtuResponse: 0x03 {
        server_rx_mtu: u16,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct FindInformationRequest: 0x04 {
        starting_handle: Handle,
        ending_handle: Handle,
    }

    #[derive(Debug)]
    pub struct FindInformationResponse: 0x05 {
        values: AttributeDataList<(Handle, Uuid)>, // FIXME
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct FindByTypeValueRequest: 0x06 {
        starting_handle: Handle,
        ending_handle: Handle,
        attribute_type: Uuid16,
        attribute_value: Bytes,
    }

    #[derive(Debug)]
    pub struct FindByTypeValueResponse: 0x07 {
        values: HandlesInformationList,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadByTypeRequest: 0x08 {
        starting_handle: Handle,
        ending_handle: Handle,
        attribute_type: Uuid,
    }

    #[derive(Debug)]
    pub struct ReadByTypeResponse: 0x09 {
        values: AttributeDataList<(Handle, Bytes)>,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadRequest: 0x0A {
        attribute_handle: Handle,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadResponse: 0x0B {
        attribute_value: Bytes,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadBlobRequest: 0x0C {
        attribute_handle: Handle,
        attribute_offset: u16,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadBlobResponse: 0x0D {
        attribute_value: Bytes,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadMultipleRequest: 0x0E {
        set_of_handles: SetOfHandles,
    }

    #[derive(Debug, New)]
    pub struct ReadMultipleResponse: 0x0F {
        set_of_values: Bytes, // FIXME
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ReadByGroupTypeRequest: 0x10 {
        starting_handle: Handle,
        ending_handle: Handle,
        attribute_group_type: Uuid,
    }

    #[derive(Debug)]
    pub struct ReadByGroupTypeResponse: 0x11 {
        values: AttributeDataList<(Handle, Handle, Bytes)>,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct WriteRequest: 0x12 {
        attribute_handle: Handle,
        attribute_value: Bytes,
    }

    #[derive(Debug, New, Default)]
    pub struct WriteResponse: 0x13 {
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct WriteCommand: 0x52 {
        attribute_handle: Handle,
        attribute_value: Bytes,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct SignedWriteCommand: 0xD2 {
        attribute_handle: Handle,
        attribute_value: Bytes,
        authentication_signature: Bytes, // FIXME
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct PrepareWriteRequest: 0x16 {
        attribute_handle: Handle,
        value_offset: u16,
        part_attribute_value: Bytes,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct PrepareWriteResponse: 0x17 {
        attribute_handle: Handle,
        value_offset: u16,
        part_attribute_value: Bytes,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct ExecuteWriteRequest: 0x18 {
        flags: bool,
    }

    #[derive(Debug, New, Default)]
    pub struct ExecuteWriteResponse: 0x19 {
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct HandleValueNotification: 0x1B {
        attribute_handle: Handle,
        attribute_value: Bytes,
    }

    #[derive(Debug, New, Getters)]
    #[get = "pub"]
    pub struct HandleValueIndication: 0x1D {
        attribute_handle: Handle,
        attribute_value: Bytes,
    }

    #[derive(Debug, New, Default)]
    pub struct HandleValueConfirmation: 0x1E {
    }

}

pub trait Request: seal::Sealed {
    type Response: Response;
}

pub trait Response: seal::Sealed {
    fn truncate(&mut self, mtu: usize);
}

pub trait Command: seal::Sealed {}

pub trait Notificaion: seal::Sealed {}

pub trait Indication: seal::Sealed {
    type Confirmation: Confirmation;
}

pub trait Confirmation: seal::Sealed {}

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
        self.attribute_value.truncate(mtu - 1);
    }
}

impl Request for ReadBlobRequest {
    type Response = ReadBlobResponse;
}
impl Response for ReadBlobResponse {
    fn truncate(&mut self, mtu: usize) {
        self.attribute_value.truncate(mtu - 1);
    }
}

impl Request for ReadMultipleRequest {
    type Response = ReadMultipleResponse;
}
impl Response for ReadMultipleResponse {
    fn truncate(&mut self, mtu: usize) {
        self.set_of_values.truncate(mtu - 1);
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
        self.part_attribute_value.truncate(mtu);
    }
}

impl Request for ExecuteWriteRequest {
    type Response = ExecuteWriteResponse;
}
impl Response for ExecuteWriteResponse {
    fn truncate(&mut self, _: usize) {}
}

impl Notificaion for HandleValueNotification {}

impl Indication for HandleValueIndication {
    type Confirmation = HandleValueConfirmation;
}
impl Confirmation for HandleValueConfirmation {}

impl Command for SignedWriteCommand {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_safety() {
        #[allow(dead_code)]
        fn foo(_: &dyn Request<Response = dyn Response>) {}
    }
}
