//! ATT Protocol UUIDs.
use std::fmt;

use bytes::{Buf, BytesMut};
pub use uuid::Uuid as Uuid128;

use crate::pack::{Error as UnpackError, Pack, Unpack};

packable_newtype! {
    /// 16bit UUID
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

/// 16bit or 128bit UUID
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Uuid {
    /// 16bit UUID
    Uuid16(Uuid16),
    /// 128bit UUID
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
