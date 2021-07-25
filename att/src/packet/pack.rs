use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("no data available.")]
    NoDataAvailable,

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error("unexpected {0}")]
    Unexpected(String),
}

pub type Result<R> = std::result::Result<R, Error>;

pub trait Pack {
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write;
}

pub trait Unpack: Sized {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read;
}

fn fill<R>(mut this: R, mut buf: &mut [u8]) -> Result<()>
where
    R: io::Read,
{
    let mut total_read = 0;
    while !buf.is_empty() {
        match this.read(buf) {
            Ok(0) => break,
            Ok(n) => {
                total_read += n;
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(e) => return Err(e.into()),
        }
    }

    if total_read == 0 {
        Err(Error::NoDataAvailable)
    } else if !buf.is_empty() {
        Err(io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer").into())
    } else {
        Ok(())
    }
}
impl Pack for () {
    fn pack<W>(self, _: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        Ok(())
    }
}

impl Unpack for () {
    fn unpack<R>(_: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(())
    }
}

impl<const N: usize> Pack for [u8; N] {
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        write.write_all(&self)?;
        Ok(())
    }
}

impl<const N: usize> Unpack for [u8; N] {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let mut this = [0; N];
        fill(read, &mut this)?;
        Ok(this)
    }
}

impl Pack for bool {
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        (if self { 1u8 } else { 0 }).pack(write)
    }
}

impl Unpack for bool {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(u8::unpack(read)? != 0)
    }
}

impl Pack for u8 {
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_le_bytes().pack(write)
    }
}

impl Unpack for u8 {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(Self::from_le_bytes(Unpack::unpack(read)?))
    }
}

impl Pack for u16 {
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_le_bytes().pack(write)
    }
}

impl Unpack for u16 {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(Self::from_le_bytes(Unpack::unpack(read)?))
    }
}

impl Pack for u32 {
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_le_bytes().pack(write)
    }
}

impl Unpack for u32 {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(Self::from_le_bytes(Unpack::unpack(read)?))
    }
}

impl Pack for u128 {
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        self.to_le_bytes().pack(write)
    }
}

impl Unpack for u128 {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        Ok(Self::from_le_bytes(Unpack::unpack(read)?))
    }
}

impl<P> Pack for Option<P>
where
    P: Pack,
{
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        if let Some(v) = self {
            v.pack(write)?;
        }
        Ok(())
    }
}

impl<P> Unpack for Option<P>
where
    P: Unpack,
{
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        match P::unpack(read) {
            Ok(v) => Ok(Some(v)),
            Err(Error::NoDataAvailable) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

impl<P> Pack for Vec<P>
where
    P: Pack,
{
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        (self.len() as u16).pack(write)?;
        for item in self {
            item.pack(write)?;
        }
        Ok(())
    }
}

impl<P> Unpack for Vec<P>
where
    P: Unpack,
{
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let len = u16::unpack(read)?;
        (0..len).map(|_| P::unpack(read)).collect()
    }
}

impl Pack for Box<[u8]> {
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        write.write_all(&self)?;
        Ok(())
    }
}

impl Unpack for Box<[u8]> {
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let mut buf = vec![];
        read.read_to_end(&mut buf)?;
        Ok(buf.into_boxed_slice())
    }
}

macro_rules! impl_tuple {
    ($($n:ident : $p:ident),+) => {
        impl<$($p),+> Pack for ($($p),+) where $($p: Pack),+ {
            fn pack<W>(self, write: &mut W) -> Result<()> where W: io::Write {
                let ($($n),+) = self;
                $( $n.pack(write)?; )+
                Ok(())
            }
        }

        impl<$($p),+> Unpack for ($($p),+) where $($p: Unpack),+ {
            fn unpack<R>(read: &mut R) -> Result<Self> where R: io::Read {
                Ok((
                    $( <$p as Unpack>::unpack(read)?, )+
                ))
            }
        }
    }
}

impl_tuple!(p1: P1, p2: P2);
impl_tuple!(p1: P1, p2: P2, p3: P3);

pub struct RemainingVec<I>(pub Vec<I>);

impl<I> Pack for RemainingVec<I>
where
    I: Pack,
{
    fn pack<W>(self, write: &mut W) -> Result<()>
    where
        W: io::Write,
    {
        for item in self.0 {
            item.pack(write)?;
        }
        Ok(())
    }
}

impl<I> Unpack for RemainingVec<I>
where
    I: Unpack,
{
    fn unpack<R>(read: &mut R) -> Result<Self>
    where
        R: io::Read,
    {
        let mut v = vec![];
        loop {
            match I::unpack(read) {
                Ok(item) => v.push(item),
                Err(Error::NoDataAvailable) => break,
                Err(err) => return Err(err),
            }
        }
        Ok(Self(v))
    }
}
