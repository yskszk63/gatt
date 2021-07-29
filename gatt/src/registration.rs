use std::collections::HashMap;
use std::hash::Hash;

use att::{Handle, Uuid};

use crate::attribute::{
    Attribute, CharacteristicExtendedProperties as AttExProperties,
    CharacteristicProperties as AttProperties, ClientCharacteristicConfiguration, Permission,
    ServerCharacteristicConfiguration,
};
use crate::database::Database;

bitflags::bitflags! {
    /// Characteristic Properties
    pub struct CharacteristicProperties: u32 {
        const BROADCAST = 0x0001;
        const READ = 0x0002;
        const WRITE_WITHOUT_RESPONSE = 0x0004;
        const WRITE = 0x0008;
        const NOTIFY = 0x0010;
        const INDICATE = 0x0020;
        const AUTHENTICATED_SIGNED_WRITES = 0x0040;
        const RELIABLE_WRITE = 0x0100;
        const WRITABLE_AUXILIARIES = 0x0200;
        const AUTHORIZATION_REQUIRED = 0x0001_0000;
    }
}

impl CharacteristicProperties {
    fn perm(&self) -> Permission {
        let mut perm = Permission::empty();
        if self.contains(Self::READ) {
            perm |= Permission::READABLE;
        }
        if self.contains(Self::WRITE) || self.contains(Self::WRITE_WITHOUT_RESPONSE) {
            perm |= Permission::WRITEABLE;
        }
        if self.contains(Self::AUTHENTICATED_SIGNED_WRITES) {
            perm |= Permission::AUTHENTICATION_REQUIRED;
        }
        if self.contains(Self::AUTHORIZATION_REQUIRED) {
            perm |= Permission::AUTHORIZATION_REQUIRED;
        }
        perm
    }
}

impl From<CharacteristicProperties> for (AttProperties, AttExProperties) {
    fn from(v: CharacteristicProperties) -> Self {
        let mut prop = AttProperties::from_bits_truncate((v.bits() & 0xFF) as u8);
        let exprop = AttExProperties::from_bits_truncate((v.bits() >> 8) as u8);
        if !exprop.is_empty() {
            prop |= AttProperties::EXTENDED_PROPERTIES;
        }
        (prop, exprop)
    }
}

#[derive(Debug)]
pub struct Registration<T> {
    next_handle: u16,
    attrs: Vec<Attribute>,
    write_handles: HashMap<Handle, T>,
    notify_or_indicate_handles: HashMap<T, Handle>,
}

impl<T> Default for Registration<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Registration<T> {
    pub fn new() -> Self {
        Self {
            next_handle: 0x0001,
            attrs: vec![],
            write_handles: HashMap::new(),
            notify_or_indicate_handles: HashMap::new(),
        }
    }
}

impl<T> Registration<T>
where
    T: Hash + Eq + Clone,
{
    fn next_handle(&mut self) -> Handle {
        let result = Handle::new(self.next_handle);
        self.next_handle += 1;
        result
    }

    pub fn add_primary_service<U>(&mut self, uuid: U)
    where
        U: Into<Uuid>,
    {
        let uuid = uuid.into();
        let handle = self.next_handle();
        self.attrs
            .push(Attribute::new_primary_service(handle, uuid));
    }

    pub fn add_characteristic<U, B>(
        &mut self,
        uuid: U,
        val: B,
        properties: CharacteristicProperties,
    ) where
        U: Into<Uuid>,
        B: AsRef<[u8]>,
    {
        self.add_characteristic_internal(None, uuid, val.as_ref(), properties)
    }

    pub fn add_characteristic_with_token<U, B>(
        &mut self,
        token: T,
        uuid: U,
        val: B,
        properties: CharacteristicProperties,
    ) where
        U: Into<Uuid>,
        T: Hash + Eq + Clone,
        B: AsRef<[u8]>,
    {
        self.add_characteristic_internal(Some(token), uuid, val.as_ref(), properties)
    }

    fn add_characteristic_internal<U>(
        &mut self,
        token: Option<T>,
        uuid: U,
        val: &[u8],
        properties: CharacteristicProperties,
    ) where
        U: Into<Uuid>,
    {
        let uuid = uuid.into();
        let val = val.into();

        let decl_handle = self.next_handle();
        let val_handle = self.next_handle();
        let perm = properties.perm();
        let writable = perm.contains(Permission::WRITEABLE);
        let notify = properties.contains(CharacteristicProperties::NOTIFY);
        let indicate = properties.contains(CharacteristicProperties::INDICATE);
        let broadcast = properties.contains(CharacteristicProperties::BROADCAST);
        let (prop, exprop) = properties.into();

        self.attrs.push(Attribute::new_characteristic(
            decl_handle,
            prop,
            val_handle.clone(),
            uuid.clone(),
        ));
        self.attrs.push(Attribute::new_characteristic_value(
            val_handle.clone(),
            uuid,
            val,
            perm,
        ));
        if !exprop.is_empty() {
            let handle = self.next_handle();
            self.attrs
                .push(Attribute::new_characteristic_extended_properties(
                    handle, exprop,
                ));
        }
        if notify || indicate {
            let handle = self.next_handle();
            if let Some(token) = &token {
                self.notify_or_indicate_handles
                    .insert(token.clone(), val_handle.clone());
            }
            self.attrs
                .push(Attribute::new_client_characteristic_configuration(
                    handle,
                    ClientCharacteristicConfiguration::empty(),
                    Permission::READABLE | Permission::WRITEABLE,
                ));
        }
        if broadcast {
            let handle = self.next_handle();
            self.attrs
                .push(Attribute::new_server_characteristic_configuration(
                    handle,
                    ServerCharacteristicConfiguration::empty(),
                    Permission::READABLE | Permission::WRITEABLE,
                ));
        }

        if writable {
            if let Some(token) = &token {
                self.write_handles.insert(val_handle, token.clone());
            }
        }
    }

    pub fn add_descriptor<U, B>(&mut self, uuid: U, val: B, writable: bool)
    where
        U: Into<Uuid>,
        B: AsRef<[u8]>,
    {
        let uuid = uuid.into();
        let handle = self.next_handle();
        let perm = if writable {
            Permission::READABLE | Permission::WRITEABLE
        } else {
            Permission::READABLE
        };
        self.attrs
            .push(Attribute::new_descriptor(handle, uuid, val.as_ref().into(), perm));
    }

    pub(crate) fn build(self) -> (Database, HashMap<Handle, T>, HashMap<T, Handle>) {
        let Self {
            attrs,
            write_handles,
            notify_or_indicate_handles,
            ..
        } = self;
        let db = attrs.into_iter().collect();
        (db, write_handles, notify_or_indicate_handles)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        #[derive(Debug, PartialEq, Eq, Hash, Clone)]
        enum Token {}

        let mut registration = Registration::<Token>::new();

        registration.add_primary_service(Uuid::new_uuid16(0x1800));
        registration.add_characteristic(
            Uuid::new_uuid16(0x2A00),
            "abc",
            CharacteristicProperties::WRITE,
        );
        registration.add_characteristic(
            Uuid::new_uuid16(0x2A01),
            "abc",
            CharacteristicProperties::READ,
        );

        registration.add_primary_service(Uuid::new_uuid16(0x1801));
        registration.add_characteristic(
            Uuid::new_uuid16(0x2A05),
            "",
            CharacteristicProperties::INDICATE,
        );

        registration.add_primary_service(Uuid::new_uuid16(0x180A));
        registration.add_characteristic(
            Uuid::new_uuid16(0x2A29),
            "",
            CharacteristicProperties::READ,
        );
        registration.add_characteristic(
            Uuid::new_uuid16(0x2A24),
            "",
            CharacteristicProperties::READ,
        );
        registration.add_characteristic(
            Uuid::new_uuid16(0x2A25),
            "",
            CharacteristicProperties::READ,
        );

        registration.add_primary_service(Uuid::new_uuid16(0x180F));
        registration.add_characteristic(
            Uuid::new_uuid16(0x2A19),
            "",
            CharacteristicProperties::NOTIFY,
        );

        println!("{:#?}", registration.build());
    }
}
