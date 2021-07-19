use bytes::{Buf, Bytes, BytesMut};

use att::uuid::Uuid16;
use att::{Handle, Uuid};

#[derive(Debug, thiserror::Error)]
pub(crate) enum Error {
    #[error("permission denied")]
    PermissionDenied,

    #[error("authorization required")]
    AuthorizationRequired,

    #[error("authentication required")]
    AuthenticationRequired,

    #[error("invalid data length")]
    InvalidDataLength,
}

bitflags::bitflags! {
    pub(crate) struct Permission: u8 {
        const READABLE = 0b0000_0001;
        const WRITEABLE = 0b0000_0010;
        const AUTHORIZATION_REQUIRED = 0b0000_0100;
        const AUTHENTICATION_REQUIRED = 0b0000_1000;
    }
}

bitflags::bitflags! {
    pub(crate) struct CharacteristicProperties: u8 {
        const BROADCAST = 0x01;
        const READ = 0x02;
        const WRITE_WITHOUT_RESPONSE = 0x04;
        const WRITE = 0x08;
        const NOTIFY = 0x10;
        const INDICATE = 0x20;
        const AUTHENTICATED_SIGNED_WRITES = 0x40;
        const EXTENDED_PROPERTIES = 0x80;
    }
}

bitflags::bitflags! {
    pub(crate) struct CharacteristicExtendedProperties: u8 {
        const RELIABLE_WRITE = 0b0001;
        const WRITABLE_AUXILIARIES = 0b0010;
    }
}

bitflags::bitflags! {
    pub(crate) struct ClientCharacteristicConfiguration: u16 {
        const NOTIFICATION = 0b0001;
        const INDICATION = 0b0010;
    }
}

bitflags::bitflags! {
    pub(crate) struct ServerCharacteristicConfiguration: u16 {
        const BROADCAST = 0b0001;
    }
}

const PRIMARY_SERVICE: Uuid = Uuid::Uuid16(Uuid16::new(0x2800));

const SECONDARY_SERVICE: Uuid = Uuid::Uuid16(Uuid16::new(0x2801));

const INCLUDE: Uuid = Uuid::Uuid16(Uuid16::new(0x2802));

const CHARACTERISTIC: Uuid = Uuid::Uuid16(Uuid16::new(0x2803));

const CHARACTERISTIC_EXTENDED_PROPERTIES: Uuid = Uuid::Uuid16(Uuid16::new(0x2900));

const CHARACTERISTIC_USER_DESCRIPTION: Uuid = Uuid::Uuid16(Uuid16::new(0x2901));

const CLIENT_CHARACTERISTIC_CONFIGURATION: Uuid = Uuid::Uuid16(Uuid16::new(0x2902));

const SERVER_CHARACTERISTIC_CONFIGURATION: Uuid = Uuid::Uuid16(Uuid16::new(0x2903));

const CHARACTERISTIC_PRESENTATION_FORMAT: Uuid = Uuid::Uuid16(Uuid16::new(0x2904));

const CHARACTERISTIC_AGGREGATE_FORMAT: Uuid = Uuid::Uuid16(Uuid16::new(0x2905));

#[derive(Debug)]
pub(crate) enum Attribute {
    Service {
        handle: Handle,
        primary: bool,
        uuid: Uuid,
    },

    #[allow(dead_code)]
    Include {
        handle: Handle,
        included_service_handle: Handle,
        end_group_handle: Handle,
        uuid: Uuid,
    },

    Characteristic {
        handle: Handle,
        properties: CharacteristicProperties,
        value_handle: Handle,
        uuid: Uuid,
    },

    CharacteristicValue {
        handle: Handle,
        attr_type: Uuid,
        value: Bytes,
        permission: Permission,
    },

    CharacteristicExtendedProperties {
        handle: Handle,
        extended_properties: CharacteristicExtendedProperties,
    },

    #[allow(dead_code)]
    CharacteristicUserDescription {
        handle: Handle,
        description: String,
        permission: Permission,
    },

    ClientCharacteristicConfiguration {
        handle: Handle,
        configuration: ClientCharacteristicConfiguration,
        permission: Permission,
    },

    ServerCharacteristicConfiguration {
        handle: Handle,
        configuration: ServerCharacteristicConfiguration,
        permission: Permission,
    },

    #[allow(dead_code)]
    CharacteristicPresentationFormat {
        handle: Handle,
        format: u8,
        exponent: u8,
        unit: u16,
        name_space: u16,
        description: u16,
    },

    #[allow(dead_code)]
    CharacteristicAggregateFormat {
        handle: Handle,
        attribute_handles: Vec<Handle>,
    },

    Descriptor {
        handle: Handle,
        uuid: Uuid,
        value: Bytes,
        permission: Permission,
    },
}

impl Attribute {
    pub(crate) fn new_primary_service(handle: Handle, uuid: Uuid) -> Self {
        Self::Service {
            handle,
            primary: true,
            uuid,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_secondary_service(handle: Handle, uuid: Uuid) -> Self {
        Self::Service {
            handle,
            primary: false,
            uuid,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_include(
        handle: Handle,
        included_service_handle: Handle,
        end_group_handle: Handle,
        uuid: Uuid,
    ) -> Self {
        Self::Include {
            handle,
            included_service_handle,
            end_group_handle,
            uuid,
        }
    }

    pub(crate) fn new_characteristic(
        handle: Handle,
        properties: CharacteristicProperties,
        value_handle: Handle,
        uuid: Uuid,
    ) -> Self {
        Self::Characteristic {
            handle,
            properties,
            value_handle,
            uuid,
        }
    }

    pub(crate) fn new_characteristic_value(
        handle: Handle,
        attr_type: Uuid,
        value: Bytes,
        permission: Permission,
    ) -> Self {
        Self::CharacteristicValue {
            handle,
            attr_type,
            value,
            permission,
        }
    }

    pub(crate) fn new_characteristic_extended_properties(
        handle: Handle,
        extended_properties: CharacteristicExtendedProperties,
    ) -> Self {
        Self::CharacteristicExtendedProperties {
            handle,
            extended_properties,
        }
    }

    pub(crate) fn new_client_characteristic_configuration(
        handle: Handle,
        configuration: ClientCharacteristicConfiguration,
        permission: Permission,
    ) -> Self {
        Self::ClientCharacteristicConfiguration {
            handle,
            configuration,
            permission,
        }
    }

    pub(crate) fn new_server_characteristic_configuration(
        handle: Handle,
        configuration: ServerCharacteristicConfiguration,
        permission: Permission,
    ) -> Self {
        Self::ServerCharacteristicConfiguration {
            handle,
            configuration,
            permission,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_characteristic_presentation_format(
        handle: Handle,
        format: u8,
        exponent: u8,
        unit: u16,
        name_space: u16,
        description: u16,
    ) -> Self {
        Self::CharacteristicPresentationFormat {
            handle,
            format,
            exponent,
            unit,
            name_space,
            description,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_characteristic_aggregate_format(
        handle: Handle,
        attribute_handles: Vec<Handle>,
    ) -> Self {
        Self::CharacteristicAggregateFormat {
            handle,
            attribute_handles,
        }
    }

    pub(crate) fn new_descriptor(
        handle: Handle,
        uuid: Uuid,
        value: Bytes,
        permission: Permission,
    ) -> Self {
        Self::Descriptor {
            handle,
            uuid,
            value,
            permission,
        }
    }
}

impl Attribute {
    pub(crate) fn handle(&self) -> &Handle {
        match self {
            Self::Service { handle, .. } => &handle,
            Self::Include { handle, .. } => &handle,
            Self::Characteristic { handle, .. } => &handle,
            Self::CharacteristicValue { handle, .. } => &handle,
            Self::CharacteristicExtendedProperties { handle, .. } => &handle,
            Self::CharacteristicUserDescription { handle, .. } => &handle,
            Self::ClientCharacteristicConfiguration { handle, .. } => &handle,
            Self::ServerCharacteristicConfiguration { handle, .. } => &handle,
            Self::CharacteristicPresentationFormat { handle, .. } => &handle,
            Self::CharacteristicAggregateFormat { handle, .. } => &handle,
            Self::Descriptor { handle, .. } => &handle,
        }
    }

    pub(crate) fn attr_type(&self) -> &Uuid {
        match self {
            Self::Service { primary, .. } if *primary => &PRIMARY_SERVICE,
            Self::Service { .. } => &SECONDARY_SERVICE,
            Self::Include { .. } => &INCLUDE,
            Self::Characteristic { .. } => &CHARACTERISTIC,
            Self::CharacteristicValue { attr_type, .. } => &attr_type,
            Self::CharacteristicExtendedProperties { .. } => &CHARACTERISTIC_EXTENDED_PROPERTIES,
            Self::CharacteristicUserDescription { .. } => &CHARACTERISTIC_USER_DESCRIPTION,
            Self::ClientCharacteristicConfiguration { .. } => &CLIENT_CHARACTERISTIC_CONFIGURATION,
            Self::ServerCharacteristicConfiguration { .. } => &SERVER_CHARACTERISTIC_CONFIGURATION,
            Self::CharacteristicPresentationFormat { .. } => &CHARACTERISTIC_PRESENTATION_FORMAT,
            Self::CharacteristicAggregateFormat { .. } => &CHARACTERISTIC_AGGREGATE_FORMAT,
            Self::Descriptor { uuid, .. } => &uuid,
        }
    }

    pub(crate) fn permission(&self) -> Permission {
        match self {
            Self::Service { .. } => Permission::READABLE,
            Self::Include { .. } => Permission::READABLE,
            Self::Characteristic { .. } => Permission::READABLE,
            Self::CharacteristicValue { permission, .. } => *permission,
            Self::CharacteristicExtendedProperties { .. } => Permission::READABLE,
            Self::CharacteristicUserDescription { permission, .. } => *permission,
            Self::ClientCharacteristicConfiguration { permission, .. } => *permission,
            Self::ServerCharacteristicConfiguration { permission, .. } => *permission,
            Self::CharacteristicPresentationFormat { .. } => Permission::READABLE,
            Self::CharacteristicAggregateFormat { .. } => Permission::READABLE,
            Self::Descriptor { permission, .. } => *permission,
        }
    }

    pub(crate) fn get(&self, authorized: bool, authenticated: bool) -> Result<Bytes, Error> {
        if !self.permission().contains(Permission::READABLE) {
            return Err(Error::PermissionDenied);
        }

        if !authorized
            && self
                .permission()
                .contains(Permission::AUTHORIZATION_REQUIRED)
        {
            return Err(Error::AuthorizationRequired);
        }

        if !authenticated
            && self
                .permission()
                .contains(Permission::AUTHENTICATION_REQUIRED)
        {
            return Err(Error::AuthenticationRequired);
        }

        Ok(match self {
            Self::Service { uuid, .. } => match uuid {
                Uuid::Uuid16(uuid) => uuid.as_u16().to_le_bytes().to_vec().into(),
                Uuid::Uuid128(uuid) => uuid.as_u128().to_le_bytes().to_vec().into(),
            },

            Self::Include {
                included_service_handle,
                end_group_handle,
                uuid,
                ..
            } => {
                let mut result = BytesMut::new();
                result.extend_from_slice(&included_service_handle.as_u16().to_le_bytes());
                result.extend_from_slice(&end_group_handle.as_u16().to_le_bytes());
                match uuid {
                    Uuid::Uuid16(uuid) => result.extend_from_slice(&uuid.as_u16().to_le_bytes()),
                    Uuid::Uuid128(uuid) => result.extend_from_slice(&uuid.as_u128().to_le_bytes()),
                }
                result.freeze()
            }

            Self::Characteristic {
                properties,
                value_handle,
                uuid,
                ..
            } => {
                let mut result = BytesMut::new();
                result.extend_from_slice(&properties.bits().to_le_bytes());
                result.extend_from_slice(&value_handle.as_u16().to_le_bytes());
                match uuid {
                    Uuid::Uuid16(uuid) => result.extend_from_slice(&uuid.as_u16().to_le_bytes()),
                    Uuid::Uuid128(uuid) => result.extend_from_slice(&uuid.as_u128().to_le_bytes()),
                }
                result.freeze()
            }

            Self::CharacteristicValue { value, .. } => value.clone(),

            Self::CharacteristicExtendedProperties {
                extended_properties,
                ..
            } => {
                let mut result = BytesMut::new();
                result.extend_from_slice(&extended_properties.bits().to_le_bytes());
                result.freeze()
            }

            Self::CharacteristicUserDescription { description, .. } => description.clone().into(),

            Self::ClientCharacteristicConfiguration { configuration, .. } => {
                let mut result = BytesMut::new();
                result.extend_from_slice(&configuration.bits().to_le_bytes());
                result.freeze()
            }

            Self::ServerCharacteristicConfiguration { configuration, .. } => {
                let mut result = BytesMut::new();
                result.extend_from_slice(&configuration.bits().to_le_bytes());
                result.freeze()
            }

            Self::CharacteristicPresentationFormat {
                format,
                exponent,
                unit,
                name_space,
                description,
                ..
            } => {
                let mut result = BytesMut::new();
                result.extend_from_slice(&format.to_le_bytes());
                result.extend_from_slice(&exponent.to_le_bytes());
                result.extend_from_slice(&unit.to_le_bytes());
                result.extend_from_slice(&name_space.to_le_bytes());
                result.extend_from_slice(&description.to_le_bytes());
                result.freeze()
            }

            Self::CharacteristicAggregateFormat {
                attribute_handles, ..
            } => {
                let mut result = BytesMut::new();
                for handle in attribute_handles {
                    result.extend_from_slice(&handle.as_u16().to_le_bytes());
                }
                result.freeze()
            }

            Self::Descriptor { value, .. } => value.clone(),
        })
    }

    pub(crate) fn set<B>(
        &mut self,
        val: &mut B,
        authorized: bool,
        authenticated: bool,
    ) -> Result<(), Error>
    where
        B: Buf,
    {
        if !self.permission().contains(Permission::WRITEABLE) {
            return Err(Error::PermissionDenied);
        }

        if !authorized
            && self
                .permission()
                .contains(Permission::AUTHORIZATION_REQUIRED)
        {
            return Err(Error::AuthorizationRequired);
        }

        if !authenticated
            && self
                .permission()
                .contains(Permission::AUTHENTICATION_REQUIRED)
        {
            return Err(Error::AuthenticationRequired);
        }

        match self {
            Self::Service { uuid, .. } => match val.remaining() {
                2 => *uuid = Uuid::new_uuid16(val.get_u16_le()),
                16 => *uuid = Uuid::new_uuid128(val.get_u128_le()),
                _ => return Err(Error::InvalidDataLength),
            },

            Self::Include {
                included_service_handle,
                end_group_handle,
                uuid,
                ..
            } => {
                match val.remaining() {
                    6 | 20 => {}
                    _ => return Err(Error::InvalidDataLength),
                }
                *included_service_handle = val.get_u16_le().into();
                *end_group_handle = val.get_u16_le().into();
                match val.remaining() {
                    2 => *uuid = Uuid::new_uuid16(val.get_u16_le()),
                    16 => *uuid = Uuid::new_uuid128(val.get_u128_le()),
                    _ => unreachable!(),
                }
            }

            Self::Characteristic {
                properties,
                value_handle,
                uuid,
                ..
            } => {
                match val.remaining() {
                    5 | 19 => {}
                    _ => return Err(Error::InvalidDataLength),
                }
                *properties = CharacteristicProperties::from_bits_truncate(val.get_u8());
                *value_handle = val.get_u16_le().into();
                match val.remaining() {
                    2 => *uuid = Uuid::new_uuid16(val.get_u16_le()),
                    16 => *uuid = Uuid::new_uuid128(val.get_u128_le()),
                    _ => return Err(Error::InvalidDataLength),
                }
            }

            Self::CharacteristicValue { value, .. } => {
                *value = val.copy_to_bytes(val.remaining());
            }

            Self::CharacteristicExtendedProperties {
                extended_properties,
                ..
            } => {
                if !val.has_remaining() {
                    return Err(Error::InvalidDataLength);
                }
                *extended_properties =
                    CharacteristicExtendedProperties::from_bits_truncate(val.get_u8());
            }

            Self::CharacteristicUserDescription { description, .. } => {
                let mut b = vec![0; val.remaining()];
                val.copy_to_slice(&mut b);
                *description = match String::from_utf8(b) {
                    Ok(v) => v,
                    Err(_) => return Err(Error::InvalidDataLength),
                }
            }

            Self::ClientCharacteristicConfiguration { configuration, .. } => {
                if val.remaining() < 2 {
                    return Err(Error::InvalidDataLength);
                }
                *configuration =
                    ClientCharacteristicConfiguration::from_bits_truncate(val.get_u16_le());
            }

            Self::ServerCharacteristicConfiguration { configuration, .. } => {
                if val.remaining() < 2 {
                    return Err(Error::InvalidDataLength);
                }
                *configuration =
                    ServerCharacteristicConfiguration::from_bits_truncate(val.get_u16_le());
            }

            Self::CharacteristicPresentationFormat {
                format,
                exponent,
                unit,
                name_space,
                description,
                ..
            } => {
                if val.remaining() != 8 {
                    return Err(Error::InvalidDataLength);
                }
                *format = val.get_u8();
                *exponent = val.get_u8();
                *unit = val.get_u16_le();
                *name_space = val.get_u16_le();
                *description = val.get_u16_le();
            }

            Self::CharacteristicAggregateFormat {
                attribute_handles, ..
            } => {
                let mut v = vec![];
                while val.remaining() > 2 {
                    v.push(val.get_u16_le().into());
                }
                if val.has_remaining() {
                    return Err(Error::InvalidDataLength);
                }
                *attribute_handles = v;
            }

            Self::Descriptor { value, .. } => *value = val.copy_to_bytes(val.remaining()),
        };
        Ok(())
    }
}
