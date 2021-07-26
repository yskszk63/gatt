use std::io;
use std::mem;
use std::net::Shutdown;
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;
use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use tokio::io::unix::AsyncFd;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};

// <bluetooth/bluetooth.h>
const BTPROTO_L2CAP: libc::c_int = 0;
const BDADDR_LE_PUBLIC: u8 = 0x01;
//const BDADDR_LE_RANDOM: u8 = 0x02;
const SOL_BLUETOOTH: libc::c_int = 274;
const BT_SECURITY: libc::c_int = 4;
//pub(crate) const BT_SECURITY_SDP: u8 = 0;
//pub(crate) const BT_SECURITY_LOW: u8 = 1;
pub(crate) const BT_SECURITY_MEDIUM: u8 = 2;
pub(crate) const BT_SECURITY_HIGH: u8 = 3;
//pub(crate) const BT_SECURITY_FIPS: u8 = 4;

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
struct bdaddr_t {
    b: [u8; 6],
}

#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
struct bt_security {
    level: u8,
    key_size: u8,
}

// <bluetooth/l2cap.h>
#[repr(C)]
#[derive(Debug)]
#[allow(non_camel_case_types)]
struct sockaddr_l2 {
    l2_family: libc::sa_family_t,
    l2_psm: libc::c_ushort,
    l2_bdaddr: bdaddr_t,
    l2_cid: libc::c_ushort,
    l2_bdaddr_type: u8,
}

fn sock_open() -> io::Result<Socket> {
    let domain = Domain::from(libc::AF_BLUETOOTH);
    let r#type = Type::SEQPACKET.nonblocking().cloexec();
    let proto = Protocol::from(BTPROTO_L2CAP);
    Socket::new(domain, r#type, Some(proto))
}

fn sock_bind(sock: &Socket, cid: libc::c_ushort) -> io::Result<()> {
    let (_, addr) = unsafe {
        SockAddr::init(|addr, _| {
            let addr = &mut *(addr as *mut sockaddr_l2);
            *addr = sockaddr_l2 {
                l2_family: (libc::AF_BLUETOOTH as libc::sa_family_t),
                l2_psm: Default::default(),
                l2_cid: cid.to_le(),
                l2_bdaddr: bdaddr_t { b: [0; 6] },
                l2_bdaddr_type: BDADDR_LE_PUBLIC,
            };
            Ok(())
        })?
    };
    sock.bind(&addr)?;
    Ok(())
}

fn set_sockopt_bt_security(fd: RawFd, level: u8, key_size: u8) -> io::Result<()> {
    let opt = bt_security { level, key_size };
    let len = mem::size_of::<bt_security>() as libc::socklen_t;

    let r = unsafe {
        libc::setsockopt(
            fd,
            SOL_BLUETOOTH,
            BT_SECURITY,
            &opt as *const _ as *const libc::c_void,
            len,
        )
    };

    if r < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

pub(crate) fn try_from(addr: socket2::SockAddr) -> io::Result<crate::Address> {
    if addr.family() == libc::AF_BLUETOOTH as libc::sa_family_t {
        let addr = unsafe { &*(addr.as_ptr() as *const sockaddr_l2) };
        Ok(addr.l2_bdaddr.b.into())
    } else {
        Err(io::Error::new(io::ErrorKind::Other, "unexpected address family."))
    }
}

#[derive(Debug)]
pub(crate) struct AttStream {
    inner: AsyncFd<Socket>,
}

impl AsyncRead for AttStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        loop {
            let mut guard = match self.inner.poll_read_ready(cx)? {
                Poll::Ready(guard) => guard,
                Poll::Pending => return Poll::Pending,
            };
            let result = guard.try_io(|fd| fd.get_ref().recv(unsafe { buf.unfilled_mut() }));
            match result {
                Ok(Ok(n)) => {
                    unsafe { buf.assume_init(n) };
                    buf.advance(n);
                    return Poll::Ready(Ok(()));
                }
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(..) => {}
            }
        }
    }
}

impl AsyncWrite for AttStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        loop {
            let mut guard = match self.inner.poll_write_ready(cx)? {
                Poll::Ready(guard) => guard,
                Poll::Pending => return Poll::Pending,
            };
            let result = guard.try_io(|fd| fd.get_ref().send(buf));
            match result {
                Ok(Ok(n)) => return Poll::Ready(Ok(n)),
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(..) => {}
            }
        }
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        Poll::Ready(Ok(()))
    }
    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        loop {
            let mut guard = match self.inner.poll_write_ready(cx)? {
                Poll::Ready(guard) => guard,
                Poll::Pending => return Poll::Pending,
            };
            let result = guard.try_io(|fd| fd.get_ref().shutdown(Shutdown::Write));
            match result {
                Ok(Ok(n)) => return Poll::Ready(Ok(n)),
                Ok(Err(err)) => return Poll::Ready(Err(err)),
                Err(..) => {}
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct AttListener {
    inner: AsyncFd<Socket>,
}

impl AttListener {
    pub(crate) fn new() -> io::Result<Self> {
        let sock = sock_open()?;
        sock_bind(&sock, 0x0004)?;
        sock.listen(1)?; // TODO backlog
        Ok(Self {
            inner: AsyncFd::new(sock)?,
        })
    }

    pub(crate) fn set_sockopt_bt_security(&self, level: u8, key_size: u8) -> io::Result<()> {
        set_sockopt_bt_security(self.inner.as_raw_fd(), level, key_size)
    }
}

impl Stream for AttListener {
    type Item = io::Result<(AttStream, socket2::SockAddr)>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let mut guard = match self.inner.poll_read_ready(cx)? {
                Poll::Ready(guard) => guard,
                Poll::Pending => return Poll::Pending,
            };
            let result = guard.try_io(|fd| fd.get_ref().accept());
            match result {
                Ok(Ok((sock, addr))) => {
                    sock.set_nonblocking(true)?;
                    let sock = AttStream {
                        inner: AsyncFd::new(sock)?,
                    };
                    return Poll::Ready(Some(Ok((sock, addr))));
                }
                Ok(Err(err)) => return Poll::Ready(Some(Err(err))),
                Err(..) => {}
            }
        }
    }
}
