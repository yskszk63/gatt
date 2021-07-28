use std::collections::BTreeMap;
use std::iter::FromIterator;
use std::ops::RangeInclusive;

use att::packet::ErrorCode;
use att::uuid::Uuid16;
use att::{Handle, Uuid};

use crate::attribute::{Attribute, Error as AttrError};

type Result<T> = std::result::Result<T, (Handle, ErrorCode)>;

#[derive(Debug)]
pub(crate) struct Database {
    attrs: BTreeMap<Handle, Attribute>,
}

impl Database {
    #[allow(clippy::type_complexity)]
    pub(crate) fn read_by_group_type(
        &self,
        range: RangeInclusive<Handle>,
        uuid: &Uuid,
        authorized: bool,
        authenticated: bool,
    ) -> Result<Vec<(Handle, Handle, Box<[u8]>)>> {
        let start = range.start().clone();

        if range.start() == &Handle::from(0x0000) || range.start() > range.end() {
            return Err((start, ErrorCode::InvalidHandle));
        }

        let mut result = vec![];
        let mut current = None as Option<(&Handle, Box<[u8]>)>;
        let mut last = &Handle::from(0x0000);
        let mut val_len = None;

        for (key, val) in self.attrs.range(range) {
            if val.attr_type() == uuid {
                if let Some((start, val)) = current {
                    val_len = Some(val.len());
                    result.push((start.clone(), last.clone(), val))
                }

                let b = match val.get(authorized, authenticated) {
                    Ok(b) => b,
                    Err(AttrError::PermissionDenied) => {
                        return Err((key.clone(), ErrorCode::ReadNotPermitted))
                    }
                    Err(AttrError::AuthorizationRequired) => {
                        return Err((key.clone(), ErrorCode::InsufficientAuthorization))
                    }
                    Err(AttrError::AuthenticationRequired) => {
                        return Err((key.clone(), ErrorCode::InsufficientAuthentication))
                    }
                    _ => unreachable!(),
                };
                if let Some(len) = val_len {
                    if len != b.len() {
                        return Ok(result);
                    }
                };
                current = Some((key, b));
            }

            last = key;
        }

        if let Some((start, val)) = current {
            result.push((start.clone(), last.clone(), val));
            Ok(result)
        } else {
            Err((start, ErrorCode::AttributeNotFound))
        }
    }

    pub(crate) fn find_by_type_value(
        &self,
        range: RangeInclusive<Handle>,
        uuid: &Uuid16,
        value: &[u8],
        authorized: bool,
        authenticated: bool,
    ) -> Result<Vec<(Handle, Handle)>> {
        let start = range.start().clone();

        let result = self
            .read_by_group_type(range, &uuid.clone().into(), authorized, authenticated)?
            .into_iter()
            .filter_map(|(handle, end, v)| {
                if &*v == value {
                    Some((handle, end))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        if result.is_empty() {
            return Err((start, ErrorCode::AttributeNotFound));
        }
        Ok(result)
    }

    pub(crate) fn read_by_type(
        &self,
        range: RangeInclusive<Handle>,
        uuid: &Uuid,
        authorized: bool,
        authenticated: bool,
    ) -> Result<Vec<(Handle, Box<[u8]>)>> {
        let start = range.start().clone();

        if range.start() == &Handle::from(0x0000) || range.start() > range.end() {
            return Err((start, ErrorCode::InvalidHandle));
        }

        let result = self
            .attrs
            .range(range)
            .filter_map(|(k, v)| {
                if v.attr_type() == uuid {
                    match v.get(authorized, authenticated) {
                        Ok(b) => Some(Ok((k.clone(), b))),
                        Err(AttrError::PermissionDenied) => {
                            Some(Err((k.clone(), ErrorCode::ReadNotPermitted)))
                        }
                        Err(AttrError::AuthorizationRequired) => {
                            Some(Err((k.clone(), ErrorCode::InsufficientAuthorization)))
                        }
                        Err(AttrError::AuthenticationRequired) => {
                            Some(Err((k.clone(), ErrorCode::InsufficientAuthentication)))
                        }
                        _ => unreachable!(),
                    }
                } else {
                    None
                }
            })
            .collect::<Result<Vec<_>>>()?;

        if result.is_empty() {
            Err((start, ErrorCode::AttributeNotFound))
        } else {
            Ok(result)
        }
    }

    pub(crate) fn find_information(
        &self,
        range: RangeInclusive<Handle>,
    ) -> Result<Vec<(Handle, Uuid)>> {
        let start = range.start().clone();

        if range.start() == &Handle::from(0x0000) || range.start() > range.end() {
            return Err((start, ErrorCode::InvalidHandle));
        }

        let mut result = self
            .attrs
            .range(range)
            .map(|(_, v)| (v.handle(), v.attr_type()));
        match result.next() {
            Some(v @ (_, Uuid::Uuid16(_))) => {
                let result = result.take_while(|(_, b)| matches!(b, Uuid::Uuid16(_)));
                Ok(std::iter::once(v)
                    .chain(result)
                    .map(|(a, b)| (a.clone(), b.clone()))
                    .collect())
            }
            Some(v @ (_, Uuid::Uuid128(_))) => {
                let result = result.take_while(|(_, b)| matches!(b, Uuid::Uuid128(_)));
                Ok(std::iter::once(v)
                    .chain(result)
                    .map(|(a, b)| (a.clone(), b.clone()))
                    .collect())
            }
            None => Err((start, ErrorCode::AttributeNotFound)),
        }
    }

    pub(crate) fn read(
        &self,
        handle: &Handle,
        authorized: bool,
        authenticated: bool,
    ) -> Result<Box<[u8]>> {
        if handle == &0x0000.into() {
            return Err((handle.clone(), ErrorCode::InvalidHandle));
        }

        if let Some(v) = self.attrs.get(handle) {
            match v.get(authorized, authenticated) {
                Ok(v) => Ok(v),
                Err(AttrError::PermissionDenied) => {
                    Err((handle.clone(), ErrorCode::ReadNotPermitted))
                }
                Err(AttrError::AuthorizationRequired) => {
                    Err((handle.clone(), ErrorCode::InsufficientAuthorization))
                }
                Err(AttrError::AuthenticationRequired) => {
                    Err((handle.clone(), ErrorCode::InsufficientAuthentication))
                }
                _ => unreachable!(),
            }
        } else {
            Err((handle.clone(), ErrorCode::AttributeNotFound))
        }
    }

    pub(crate) fn write(
        &mut self,
        handle: &Handle,
        val: &[u8],
        authorized: bool,
        authenticated: bool,
    ) -> Result<()> {
        if handle == &0x0000.into() {
            return Err((handle.clone(), ErrorCode::InvalidHandle));
        }

        if let Some(v) = self.attrs.get_mut(handle) {
            match v.set(val, authorized, authenticated) {
                Ok(_) => Ok(()),
                Err(AttrError::PermissionDenied) => {
                    Err((handle.clone(), ErrorCode::WriteNotPermitted))
                }
                Err(AttrError::AuthorizationRequired) => {
                    Err((handle.clone(), ErrorCode::InsufficientAuthorization))
                }
                Err(AttrError::AuthenticationRequired) => {
                    Err((handle.clone(), ErrorCode::InsufficientAuthentication))
                }
                Err(AttrError::InvalidDataLength) => {
                    Err((handle.clone(), ErrorCode::InvalidAttributeValueLength))
                }
            }
        } else {
            Err((handle.clone(), ErrorCode::AttributeNotFound))
        }
    }
}

impl FromIterator<Attribute> for Database {
    fn from_iter<T: IntoIterator<Item = Attribute>>(iter: T) -> Self {
        Self {
            attrs: iter.into_iter().map(|a| (a.handle().clone(), a)).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::attribute::{
        CharacteristicProperties, ClientCharacteristicConfiguration, Permission,
    };

    #[test]
    fn test_read_by_group_type() {
        let db = example_db();

        let result = db
            .read_by_group_type(
                0x0001.into()..=0xFFFF.into(),
                &Uuid::new_uuid16(0x2800),
                false,
                false,
            )
            .unwrap();
        assert_eq!(
            &result,
            &[
                (0x0001.into(), 0x0005.into(), vec![0x00, 0x18].into()),
                (0x000C.into(), 0x000F.into(), vec![0x01, 0x18].into()),
                (0x0010.into(), 0x0016.into(), vec![0x0A, 0x18].into()),
            ]
        );

        let result = db
            .read_by_group_type(
                0x0017.into()..=0xFFFF.into(),
                &Uuid::new_uuid16(0x2800),
                false,
                false,
            )
            .unwrap();
        assert_eq!(
            &result,
            &[(
                0x0020.into(),
                0x0020.into(),
                vec![
                    0x34, 0x12, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00
                ]
                .into()
            ),]
        );

        let result = db
            .read_by_group_type(
                0x0021.into()..=0xFFFF.into(),
                &Uuid::new_uuid16(0x2800),
                false,
                false,
            )
            .unwrap();
        assert_eq!(
            &result,
            &[(0x0023.into(), 0x0027.into(), vec![0x0F, 0x18].into()),]
        );

        let result = db
            .read_by_group_type(
                0x0028.into()..=0xFFFF.into(),
                &Uuid::new_uuid16(0x2800),
                false,
                false,
            )
            .unwrap_err();
        assert_eq!(result, (0x0028.into(), ErrorCode::AttributeNotFound));

        let result = db
            .read_by_group_type(
                0x0002.into()..=0x0001.into(),
                &Uuid::new_uuid16(0x2800),
                false,
                false,
            )
            .unwrap_err();
        assert_eq!(result, (0x0002.into(), ErrorCode::InvalidHandle));

        let result = db
            .read_by_group_type(
                0x0000.into()..=0x0001.into(),
                &Uuid::new_uuid16(0x2800),
                false,
                false,
            )
            .unwrap_err();
        assert_eq!(result, (0x0000.into(), ErrorCode::InvalidHandle));
    }

    #[test]
    fn test_find_by_type_value() {
        let db = example_db();

        let result = db
            .find_by_type_value(
                0x0001.into()..=0xFFFF.into(),
                &Uuid16::new(0x2800),
                &vec![0x01, 0x18],
                false,
                false,
            )
            .unwrap();
        assert_eq!(&result, &[(0x000C.into(), 0x000F.into())]);
        let result = db
            .find_by_type_value(
                0x0010.into()..=0xFFFF.into(),
                &Uuid16::new(0x2800),
                &vec![0x01, 0x18],
                false,
                false,
            )
            .unwrap_err();
        assert_eq!(result, (0x0010.into(), ErrorCode::AttributeNotFound));
    }

    #[test]
    fn test_read_by_type() {
        let db = example_db();

        let result = db
            .read_by_type(
                0x0001.into()..=0x000b.into(),
                &Uuid::new_uuid16(0x2802),
                false,
                false,
            )
            .unwrap_err();
        assert_eq!(result, (0x0001.into(), ErrorCode::AttributeNotFound));

        let result = db
            .read_by_type(
                0x0001.into()..=0x000b.into(),
                &Uuid::new_uuid16(0x2803),
                false,
                false,
            )
            .unwrap();
        assert_eq!(
            result,
            &[
                (0x0002.into(), vec![0x08, 0x03, 0x00, 0x00, 0x2A].into()),
                (0x0004.into(), vec![0x02, 0x05, 0x00, 0x01, 0x2A].into()),
            ]
        );

        let result = db
            .read_by_type(
                0x0005.into()..=0x000b.into(),
                &Uuid::new_uuid16(0x2803),
                false,
                false,
            )
            .unwrap_err();
        assert_eq!(result, (0x0005.into(), ErrorCode::AttributeNotFound));

        let result = db
            .read_by_type(
                0x0002.into()..=0x0001.into(),
                &Uuid::new_uuid16(0x2802),
                false,
                false,
            )
            .unwrap_err();
        assert_eq!(result, (0x0002.into(), ErrorCode::InvalidHandle));

        let result = db
            .read_by_type(
                0x0000.into()..=0x0001.into(),
                &Uuid::new_uuid16(0x2802),
                false,
                false,
            )
            .unwrap_err();
        assert_eq!(result, (0x0000.into(), ErrorCode::InvalidHandle));
    }

    #[test]
    fn test_find_information() {
        let db = example_db();

        let result = db
            .find_information(0x0006.into()..=0x000B.into())
            .unwrap_err();
        assert_eq!(result, (0x0006.into(), ErrorCode::AttributeNotFound));

        let result = db.find_information(0x000F.into()..=0x000F.into()).unwrap();
        assert_eq!(result, &[(0x000F.into(), Uuid::new_uuid16(0x2902)),]);

        let result = db.find_information(0x0026.into()..=0x0027.into()).unwrap();
        assert_eq!(
            result,
            &[
                (0x0026.into(), Uuid::new_uuid16(0x2902)),
                (0x0027.into(), Uuid::new_uuid16(0x2904)),
            ]
        );

        let result = db
            .find_information(0x0002.into()..=0x0001.into())
            .unwrap_err();
        assert_eq!(result, (0x0002.into(), ErrorCode::InvalidHandle));

        let result = db
            .find_information(0x0000.into()..=0x0001.into())
            .unwrap_err();
        assert_eq!(result, (0x0000.into(), ErrorCode::InvalidHandle));
    }

    #[test]
    fn test_read() {
        let db = example_db();

        let result = db.read(&0x0005.into(), false, false).unwrap();
        assert_eq!(&*result, &b"abc"[..]);

        let result = db.read(&0x0000.into(), false, false).unwrap_err();
        assert_eq!(result, (0x0000.into(), ErrorCode::InvalidHandle));
    }

    #[test]
    fn test_write() {
        let mut db = example_db();

        let _result = db
            .write(&0x000F.into(), &vec![0x00, 0x00], false, false)
            .unwrap();

        let result = db.write(&0x0000.into(), &vec![], false, false).unwrap_err();
        assert_eq!(result, (0x0000.into(), ErrorCode::InvalidHandle));
    }

    fn example_db() -> Database {
        vec![
            Attribute::new_primary_service(0x0001.into(), Uuid::new_uuid16(0x1800)),
            Attribute::new_characteristic(
                0x0002.into(),
                CharacteristicProperties::WRITE,
                0x0003.into(),
                Uuid::new_uuid16(0x2A00),
            ),
            Attribute::new_characteristic_value(
                0x0003.into(),
                Uuid::new_uuid16(0x2A00),
                [].into(),
                Permission::WRITEABLE,
            ),
            Attribute::new_characteristic(
                0x0004.into(),
                CharacteristicProperties::READ,
                0x0005.into(),
                Uuid::new_uuid16(0x2A01),
            ),
            Attribute::new_characteristic_value(
                0x0005.into(),
                Uuid::new_uuid16(0x2A01),
                b"abc".as_ref().into(),
                Permission::READABLE,
            ),
            Attribute::new_primary_service(0x000C.into(), Uuid::new_uuid16(0x1801)),
            Attribute::new_characteristic(
                0x000D.into(),
                CharacteristicProperties::INDICATE,
                0x000E.into(),
                Uuid::new_uuid16(0x2A05),
            ),
            Attribute::new_characteristic_value(
                0x000E.into(),
                Uuid::new_uuid16(0x2A05),
                [].into(),
                Permission::READABLE,
            ),
            Attribute::new_client_characteristic_configuration(
                0x000F.into(),
                ClientCharacteristicConfiguration::empty(),
                Permission::READABLE | Permission::WRITEABLE,
            ),
            Attribute::new_primary_service(0x0010.into(), Uuid::new_uuid16(0x180A)),
            Attribute::new_characteristic(
                0x0011.into(),
                CharacteristicProperties::READ,
                0x0012.into(),
                Uuid::new_uuid16(0x2A29),
            ),
            Attribute::new_characteristic_value(
                0x0012.into(),
                Uuid::new_uuid16(0x2A29),
                [].into(),
                Permission::READABLE,
            ),
            Attribute::new_characteristic(
                0x0013.into(),
                CharacteristicProperties::READ,
                0x0014.into(),
                Uuid::new_uuid16(0x2A24),
            ),
            Attribute::new_characteristic_value(
                0x0014.into(),
                Uuid::new_uuid16(0x2A24),
                [].into(),
                Permission::READABLE,
            ),
            Attribute::new_characteristic(
                0x0015.into(),
                CharacteristicProperties::READ,
                0x0016.into(),
                Uuid::new_uuid16(0x2A25),
            ),
            Attribute::new_characteristic_value(
                0x0016.into(),
                Uuid::new_uuid16(0x2A25),
                [].into(),
                Permission::READABLE,
            ),
            Attribute::new_primary_service(0x0020.into(), Uuid::new_uuid128(0x1234)),
            Attribute::new_primary_service(0x0023.into(), Uuid::new_uuid16(0x180F)),
            Attribute::new_characteristic(
                0x0024.into(),
                CharacteristicProperties::NOTIFY | CharacteristicProperties::READ,
                0x0025.into(),
                Uuid::new_uuid16(0x2A19),
            ),
            Attribute::new_characteristic_value(
                0x0025.into(),
                Uuid::new_uuid16(0x2A19),
                [].into(),
                Permission::READABLE,
            ),
            Attribute::new_client_characteristic_configuration(
                0x0026.into(),
                ClientCharacteristicConfiguration::empty(),
                Permission::READABLE | Permission::WRITEABLE,
            ),
            Attribute::new_characteristic_presentation_format(0x0027.into(), 0, 0, 0, 0, 0),
        ]
        .into_iter()
        .collect::<Database>()
    }
}
