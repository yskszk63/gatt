use std::iter::{ Extend, FromIterator};

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

impl FromIterator<(Handle, Bytes)> for ReadByTypeResponse {
    fn from_iter<T: IntoIterator<Item = (Handle, Bytes)>>(iter: T) -> Self {
        Self {
            values: AttributeDataList(iter.into_iter().collect()),
        }
    }
}

impl Extend<(Handle, Bytes)> for ReadByTypeResponse {
    fn extend<T: IntoIterator<Item = (Handle, Bytes)>>(&mut self, iter: T) {
        self.values.0.extend(iter)
    }
}

impl FromIterator<(Handle, Handle, Bytes)> for ReadByGroupTypeResponse {
    fn from_iter<T: IntoIterator<Item = (Handle, Handle, Bytes)>>(iter: T) -> Self {
        Self {
            values: AttributeDataList(iter.into_iter().collect()),
        }
    }
}

impl Extend<(Handle, Handle, Bytes)> for ReadByGroupTypeResponse {
    fn extend<T: IntoIterator<Item = (Handle, Handle, Bytes)>>(&mut self, iter: T) {
        self.values.0.extend(iter)
    }
}
