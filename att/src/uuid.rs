//! ATT Protocol UUIDs.
use std::fmt;
use std::io;

pub use uuid::Uuid as Uuid128;

use crate::packet::pack::{Error as PackError, Pack, Result as PackResult, Unpack};

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
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        self.to_u128_le().pack(write)
    }
}

impl Unpack for Uuid128 {
    fn unpack<R>(read: &mut R) -> PackResult<Self>
    where
        R: io::Read,
    {
        Ok(Self::from_u128_le(Unpack::unpack(read)?))
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
    fn pack<W>(self, write: &mut W) -> PackResult<()>
    where
        W: io::Write,
    {
        match self {
            Self::Uuid16(uuid) => uuid.pack(write),
            Self::Uuid128(uuid) => uuid.pack(write),
        }
    }
}

impl Unpack for Uuid {
    fn unpack<R>(read: &mut R) -> PackResult<Self>
    where
        R: io::Read,
    {
        let buf = Box::<[u8]>::unpack(read)?;
        Ok(match buf.len() {
            2 => Self::Uuid16(Unpack::unpack(read)?),
            16 => Self::Uuid128(Unpack::unpack(read)?),
            unknown => return Err(PackError::Unexpected(format!("uuid length {}", unknown))),
        })
    }
}
