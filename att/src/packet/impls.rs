use std::iter::{Extend, FromIterator};

use super::*;

impl FromIterator<(Handle, Uuid)> for FindInformationResponse {
    fn from_iter<T: IntoIterator<Item = (Handle, Uuid)>>(iter: T) -> Self {
        Self {
            values: AttributeDataList(iter.into_iter().collect()),
        }
    }
}

impl Extend<(Handle, Uuid)> for FindInformationResponse {
    fn extend<T: IntoIterator<Item = (Handle, Uuid)>>(&mut self, iter: T) {
        self.values.0.extend(iter)
    }
}

impl FromIterator<(Handle, Handle)> for FindByTypeValueResponse {
    fn from_iter<T: IntoIterator<Item = (Handle, Handle)>>(iter: T) -> Self {
        Self {
            values: HandlesInformationList(iter.into_iter().collect()),
        }
    }
}

impl Extend<(Handle, Handle)> for FindByTypeValueResponse {
    fn extend<T: IntoIterator<Item = (Handle, Handle)>>(&mut self, iter: T) {
        self.values.0.extend(iter)
    }
}

impl FromIterator<(Handle, Box<[u8]>)> for ReadByTypeResponse {
    fn from_iter<T: IntoIterator<Item = (Handle, Box<[u8]>)>>(iter: T) -> Self {
        Self {
            values: AttributeDataList(iter.into_iter().collect()),
        }
    }
}

impl Extend<(Handle, Box<[u8]>)> for ReadByTypeResponse {
    fn extend<T: IntoIterator<Item = (Handle, Box<[u8]>)>>(&mut self, iter: T) {
        self.values.0.extend(iter)
    }
}

impl FromIterator<(Handle, Handle, Box<[u8]>)> for ReadByGroupTypeResponse {
    fn from_iter<T: IntoIterator<Item = (Handle, Handle, Box<[u8]>)>>(iter: T) -> Self {
        Self {
            values: AttributeDataList(iter.into_iter().collect()),
        }
    }
}

impl Extend<(Handle, Handle, Box<[u8]>)> for ReadByGroupTypeResponse {
    fn extend<T: IntoIterator<Item = (Handle, Handle, Box<[u8]>)>>(&mut self, iter: T) {
        self.values.0.extend(iter)
    }
}

impl IntoIterator for ReadMultipleRequest {
    type Item = Handle;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.set_of_handles.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a ReadMultipleRequest {
    type Item = &'a Handle;
    type IntoIter = std::slice::Iter<'a, Handle>;
    fn into_iter(self) -> Self::IntoIter {
        self.set_of_handles.into_iter()
    }
}
