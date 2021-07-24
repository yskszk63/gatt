macro_rules! packable_enum {
    (
        $(#[$attrs:meta])*
        $vis:vis enum $name:ident : $ty:ty {
            $(
                $(#[$vattrs:meta])*
                $vvis:vis $vname:ident $( = $vval:expr )?,
            )*
        }
    ) => {
        $(#[$attrs])*
        $vis enum $name {
            $(
                $(#[$vattrs])*
                $vvis $vname $( = $vval )?,
            )*
        }

        impl $crate::packet::pack::Pack for $name {
            fn pack<W>(self, write: &mut W) -> $crate::packet::pack::Result<()> where W: std::io::Write {
                <$ty as $crate::packet::pack::Pack>::pack(self as $ty, write)
            }
        }

        impl $crate::packet::pack::Unpack for $name {
            fn unpack<R>(read: &mut R) -> $crate::packet::pack::Result<Self> where R: std::io::Read {
                #![allow(non_upper_case_globals)]
                $( const $vname: $ty = $name::$vname as $ty; )*
                Ok(match <$ty as $crate::packet::pack::Unpack>::unpack(read)? {
                    $( $vname => Self::$vname, )*
                    unknown => return Err($crate::packet::pack::Error::Unexpected(format!("{:X?}", unknown))),
                })
            }
        }
    }
}

macro_rules! packable_newtype {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident ( $ty:ty );
    ) => {
        $(#[$attrs])*
        $vis struct $name ( $ty );

        impl $crate::packet::pack::Pack for $name {
            fn pack<W>(self, write: &mut W) -> $crate::packet::pack::Result<()> where W: std::io::Write {
                self.0.pack(write)
            }
        }

        impl $crate::packet::pack::Unpack for $name {
            fn unpack<R>(read: &mut R) -> $crate::packet::pack::Result<Self> where R: std::io::Read {
                Ok(Self($crate::packet::pack::Unpack::unpack(read)?))
            }
        }
    }
}

macro_rules! packable_struct {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$fattrs:meta])*
                $fvis:vis $fname:ident : $fty:ty,
            )*
        }
    ) => {
        $(#[$attrs])*
        pub struct $name {
            $(
                $(#[$fattrs])*
                $fvis $fname : $fty,
            )*
        }

        impl $crate::packet::pack::Pack for $name {
            fn pack<W>(self, #[allow(unused_variables)]write: &mut W) -> $crate::packet::pack::Result<()> where W: std::io::Write {
                $( self.$fname.pack(write)?; )*
                Ok(())
            }
        }

        impl $crate::packet::pack::Unpack for $name {
            fn unpack<R>(#[allow(unused_variables)]read: &mut R) -> $crate::packet::pack::Result<Self> where R: std::io::Read {
                Ok(Self {
                    $( $fname : $crate::packet::pack::Unpack::unpack(read)?, )*
                })
            }
        }
    }
}

