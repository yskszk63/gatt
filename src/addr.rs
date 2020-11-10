use std::fmt;

#[derive(Debug)]
pub struct Address([u8; 6]);

impl Address {
    pub(crate) fn new(b: [u8; 6]) -> Self {
        Self(b)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], // TODO reverse?
        )
    }
}
