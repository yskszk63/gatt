use std::future::Future;
use std::io;
use std::mem::{self, MaybeUninit};
use std::os::unix::io::{AsRawFd, RawFd};
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll};

use socket2::{Domain, Protocol, SockAddr, Socket, Type};
use tokio::io::unix::AsyncFd;

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

impl sockaddr_l2 {
    unsafe fn try_from(s: SockAddr) -> Option<Self> {
        if s.family() != libc::AF_BLUETOOTH as libc::sa_family_t {
            return None;
        }
        assert_eq!(s.len() as usize, mem::size_of::<Self>());

        let mut result = MaybeUninit::<sockaddr_l2>::uninit();
        ptr::copy_nonoverlapping(
            s.as_ptr() as *const u8,
            &mut result as *mut _ as *mut u8,
            s.len() as usize,
        );
        Some(result.assume_init())
    }
}

impl From<sockaddr_l2> for libc::sockaddr {
    fn from(s: sockaddr_l2) -> Self {
        let mut result = MaybeUninit::<libc::sockaddr>::uninit();
        unsafe {
            ptr::copy_nonoverlapping(
                &s as *const _ as *const u8,
                &mut result as *mut _ as *mut u8,
                mem::size_of::<sockaddr_l2>(),
            );
            result.assume_init()
        }
    }
}

macro_rules! ready {
    ($e:expr) => {
        match $e {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(e) => e,
        }
    };
}

fn is_wouldblock(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::WouldBlock
}

fn sock_open() -> io::Result<Socket> {
    let domain = Domain::from(libc::AF_BLUETOOTH);
    let r#type = Type::seqpacket().non_blocking().cloexec();
    let proto = Protocol::from(BTPROTO_L2CAP);
    Socket::new(domain, r#type, Some(proto))
}

fn sock_bind(sock: &Socket, cid: libc::c_ushort) -> io::Result<()> {
    let addr = sockaddr_l2 {
        l2_family: (libc::AF_BLUETOOTH as libc::sa_family_t),
        l2_psm: Default::default(),
        l2_cid: cid.to_le(),
        l2_bdaddr: bdaddr_t { b: [0; 6] },
        l2_bdaddr_type: BDADDR_LE_PUBLIC,
    }
    .into();

    let addr = unsafe {
        SockAddr::from_raw_parts(
            &addr as *const _,
            mem::size_of::<sockaddr_l2>() as libc::socklen_t,
        )
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

#[derive(Debug)]
pub(crate) struct Recv<'a, 'b> {
    inner: &'a AsyncFd<Socket>,
    buf: &'b mut [u8],
}

impl<'a, 'b> Future for Recv<'a, 'b> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { inner, buf } = self.get_mut();
        loop {
            let mut guard = ready!(inner.poll_read_ready(cx)?);

            match inner.get_ref().recv(buf) {
                Err(e) if is_wouldblock(&e) => guard.clear_ready(),
                Err(e) => return Poll::Ready(Err(e)),
                Ok(ret) => return Poll::Ready(Ok(ret)),
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Send<'a, 'b> {
    inner: &'a AsyncFd<Socket>,
    buf: &'b [u8],
}

impl<'a, 'b> Future for Send<'a, 'b> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let Self { inner, buf } = self.get_mut();
        loop {
            let mut guard = ready!(inner.poll_write_ready(cx)?);

            match inner.get_ref().send(buf) {
                Err(e) if is_wouldblock(&e) => guard.clear_ready(),
                Err(e) => return Poll::Ready(Err(e)),
                Ok(ret) => return Poll::Ready(Ok(ret)),
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct AttStream {
    inner: AsyncFd<Socket>,
}

impl AttStream {
    pub(crate) fn recv<'b>(&self, buf: &'b mut [u8]) -> Recv<'_, 'b> {
        Recv {
            inner: &self.inner,
            buf,
        }
    }

    pub(crate) fn send<'b>(&self, buf: &'b [u8]) -> Send<'_, 'b> {
        Send {
            inner: &self.inner,
            buf,
        }
    }
}

#[derive(Debug)]
pub(crate) struct Accept<'a> {
    inner: &'a AsyncFd<Socket>,
}

impl<'a> Future for Accept<'a> {
    type Output = io::Result<(AttStream, crate::Address)>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        loop {
            let pinned = Pin::new(&mut self.inner);
            let mut guard = ready!(pinned.poll_read_ready(cx)?);

            match pinned.get_ref().accept() {
                Err(e) if is_wouldblock(&e) => guard.clear_ready(),
                Err(e) => return Poll::Ready(Err(e)),
                Ok((sock, addr)) => {
                    let addr = unsafe { sockaddr_l2::try_from(addr) };
                    let addr = crate::Address::from(addr.unwrap().l2_bdaddr.b);
                    sock.set_nonblocking(true)?;
                    let sock = AttStream {
                        inner: AsyncFd::new(sock)?,
                    };
                    return Poll::Ready(Ok((sock, addr)));
                }
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

    pub(crate) fn accept(&self) -> Accept {
        Accept { inner: &self.inner }
    }
}
