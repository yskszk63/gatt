use bytes::{Buf, BufMut, Bytes, BytesMut};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("buffer overflow")]
    Overflow,
    #[error("unexpected data {0:}")]
    Unexpected(String),
}

pub type Result<R> = std::result::Result<R, Error>;

pub trait Pack {
    fn pack(self, buf: &mut BytesMut);
}

pub trait Unpack: Sized {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self>;
}

impl Pack for () {
    fn pack(self, _: &mut BytesMut) {}
}

impl Unpack for () {
    fn unpack<B: Buf>(_: &mut B) -> Result<Self> {
        Ok(())
    }
}

impl Pack for bool {
    fn pack(self, buf: &mut BytesMut) {
        (if self { 1u8 } else { 0 }).pack(buf)
    }
}

impl Unpack for bool {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        Ok(u8::unpack(buf)? != 0)
    }
}

impl Pack for u8 {
    fn pack(self, buf: &mut BytesMut) {
        buf.put_u8(self)
    }
}

impl Unpack for u8 {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 1 {
            return Err(Error::Overflow);
        }
        Ok(buf.get_u8())
    }
}

impl Pack for u16 {
    fn pack(self, buf: &mut BytesMut) {
        buf.put_u16_le(self)
    }
}

impl Unpack for u16 {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 2 {
            return Err(Error::Overflow);
        }
        Ok(buf.get_u16_le())
    }
}

impl Pack for u32 {
    fn pack(self, buf: &mut BytesMut) {
        buf.put_u32_le(self)
    }
}

impl Unpack for u32 {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 4 {
            return Err(Error::Overflow);
        }
        Ok(buf.get_u32_le())
    }
}

impl Pack for u128 {
    fn pack(self, buf: &mut BytesMut) {
        buf.put_u128_le(self)
    }
}

impl Unpack for u128 {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.remaining() < 16 {
            return Err(Error::Overflow);
        }
        Ok(buf.get_u128_le())
    }
}

impl<P> Pack for Option<P>
where
    P: Pack,
{
    fn pack(self, buf: &mut BytesMut) {
        if let Some(v) = self {
            v.pack(buf);
        }
    }
}

impl<P> Unpack for Option<P>
where
    P: Unpack,
{
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        if buf.has_remaining() {
            Ok(Some(P::unpack(buf)?))
        } else {
            Ok(None)
        }
    }
}

impl Pack for Bytes {
    fn pack(self, buf: &mut BytesMut) {
        buf.extend_from_slice(&self);
    }
}

impl Unpack for Bytes {
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        Ok(buf.copy_to_bytes(buf.remaining()))
    }
}

impl<P> Pack for Vec<P>
where
    P: Pack,
{
    fn pack(self, buf: &mut BytesMut) {
        (self.len() as u16).pack(buf);
        for item in self {
            item.pack(buf);
        }
    }
}

impl<P> Unpack for Vec<P>
where
    P: Unpack,
{
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        let len = u16::unpack(buf)?;
        (0..len).map(|_| P::unpack(buf)).collect()
    }
}

impl<P> Pack for Box<P>
where
    P: Pack,
{
    fn pack(self, buf: &mut BytesMut) {
        <P as Pack>::pack(*self, buf)
    }
}

impl<P> Unpack for Box<P>
where
    P: Unpack,
{
    fn unpack<B: Buf>(buf: &mut B) -> Result<Self> {
        let inner = <P as Unpack>::unpack(buf)?;
        Ok(Box::new(inner))
    }
}

macro_rules! fixed_length_array {
    ($n:literal) => {
        impl $crate::pack::Pack for [u8; $n] {
            fn pack(self, buf: &mut bytes::BytesMut) {
                buf.extend_from_slice(&self);
            }
        }

        impl $crate::pack::Unpack for [u8; $n] {
            fn unpack<B: bytes::Buf>(buf: &mut B) -> $crate::pack::Result<Self> {
                if buf.remaining() < $n {
                    return Err($crate::pack::Error::Overflow);
                }

                let mut v = [0; $n];
                buf.copy_to_slice(&mut v);
                Ok(v)
            }
        }
    };
}

fixed_length_array!(3);
fixed_length_array!(4);
fixed_length_array!(6);
fixed_length_array!(8);
fixed_length_array!(11);
fixed_length_array!(16);
fixed_length_array!(31);
fixed_length_array!(249);

macro_rules! impl_tuple {
    ($($n:ident : $p:ident),+) => {
        impl<$($p),+> $crate::pack::Pack for ($($p),+) where $($p: $crate::pack::Pack),+ {
            fn pack(self, buf: &mut bytes::BytesMut) {
                let ($($n),+) = self;
                $($n.pack(buf);)+
            }
        }

        impl<$($p),+> $crate::pack::Unpack for ($($p),+) where $($p: $crate::pack::Unpack),+ {
            fn unpack<B: bytes::Buf>(buf: &mut B) -> $crate::pack::Result<Self> {
                Ok((
                    $(<$p as $crate::pack::Unpack>::unpack(buf)?,)+
                ))
            }
        }
    }
}

impl_tuple!(p1: P1, p2: P2);
impl_tuple!(p1: P1, p2: P2, p3: P3);

macro_rules! packable_enum {
    (
        $(#[$attrs:meta])*
        $vis:vis enum $name:ident : $ty:ty {
            $(
                $(#[$vattrs:meta])*
                $vvis:vis $vname:ident => $vval:expr,
            )*
        }
    ) => {
        $(#[$attrs])*
        $vis enum $name {
            $(
                $(#[$vattrs])*
                $vvis $vname,
            )*
        }

        impl $crate::pack::Pack for $name {
            fn pack(self, buf: &mut bytes::BytesMut) {
                let v = match self {
                    $(
                        Self::$vname => $vval,
                    )*
                };
                <$ty as $crate::pack::Pack>::pack(v, buf);
            }
        }

        impl $crate::pack::Unpack for $name {
            fn unpack<B: bytes::Buf>(buf: &mut B) -> $crate::pack::Result<Self> {
                match <$ty as $crate::pack::Unpack>::unpack(buf)? {
                    $(
                        $vval => Ok(Self::$vname),
                    )*
                    v => Err($crate::pack::Error::Unexpected(format!("{}", v)))
                }
            }
        }
    };
}

macro_rules! packable_newtype {
    (
        $(#[$attrs:meta])*
        $vis:vis struct $name:ident ( $ty:ty );
    ) => {
        $(#[$attrs])*
        $vis struct $name ( $ty );

        impl $crate::pack::Pack for $name {
            fn pack(self, buf: &mut bytes::BytesMut) {
                self.0.pack(buf);
            }
        }

        impl $crate::pack::Unpack for $name {
            fn unpack<B: bytes::Buf>(buf: &mut B) -> $crate::pack::Result<Self> {
                Ok(Self($crate::pack::Unpack::unpack(buf)?))
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

        impl $crate::pack::Pack for $name {
            fn pack(self, #[allow(unused_variables)]buf: &mut bytes::BytesMut) {
                $(
                    self.$fname.pack(buf);
                )*
            }
        }

        impl $crate::pack::Unpack for $name {
            fn unpack<B: bytes::Buf>(#[allow(unused_variables)]buf: &mut B) -> $crate::pack::Result<Self> {
                Ok(Self {
                    $($fname : $crate::pack::Unpack::unpack(buf)?,)*
                })
            }
        }
    }
}
