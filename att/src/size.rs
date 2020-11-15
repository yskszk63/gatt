use bytes::Bytes;

use crate::{Handle, Uuid};

pub(crate) trait Size {
    fn size(&self) -> usize;
}

impl Size for Handle {
    fn size(&self) -> usize {
        2
    }
}

impl Size for Uuid {
    fn size(&self) -> usize {
        match self {
            Self::Uuid16(_) => 2,
            Self::Uuid128(_) => 16,
        }
    }
}

impl Size for Bytes {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<E1, E2> Size for (E1, E2)
where
    E1: Size,
    E2: Size,
{
    fn size(&self) -> usize {
        self.0.size() + self.1.size()
    }
}

impl<E1, E2, E3> Size for (E1, E2, E3)
where
    E1: Size,
    E2: Size,
    E3: Size,
{
    fn size(&self) -> usize {
        self.0.size() + self.1.size() + self.2.size()
    }
}
