use std::fmt;

packable_newtype! {
    /// Attribute Handle.
    #[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Handle(u16);
}

impl Handle {
    /// Construct new Handle instance.
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
