#![allow(non_camel_case_types)]
use crate::fmt;
use crate::io::{self, IoSlice, IoSliceMut};
use crate::net::{Ipv4Addr, Ipv6Addr, Shutdown, SocketAddr};
use crate::sys::unsupported;

use crate::os::unix::prelude::*;
pub use crate::sys::{cvt, cvt_r};

use crate::cmp;
use crate::ffi::CStr;
use crate::mem;
use crate::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
use crate::str;
use crate::sys::fd::FileDesc;
use crate::sys_common::net::{getsockopt, setsockopt, sockaddr_to_addr};
use crate::sys_common::{AsInner, FromInner, IntoInner};
use crate::time::{Duration, Instant};

pub extern crate libc as netc;
use core::ffi::c_void;
use netc::c_int;
use netc::{sockaddr, socklen_t, MSG_PEEK};

pub type wrlen_t = netc::size_t;

pub struct TcpStream(!);

impl TcpStream {
    pub fn connect(_: io::Result<&SocketAddr>) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn connect_timeout(_: &SocketAddr, _: Duration) -> io::Result<TcpStream> {
        unsupported()
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn read(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn read_vectored(&self, _: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0
    }

    pub fn is_read_vectored(&self) -> bool {
        self.0
    }

    pub fn write(&self, _: &[u8]) -> io::Result<usize> {
        self.0
    }

    pub fn write_vectored(&self, _: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0
    }

    pub fn is_write_vectored(&self) -> bool {
        self.0
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn shutdown(&self, _: Shutdown) -> io::Result<()> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<TcpStream> {
        self.0
    }

    pub fn set_linger(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn set_nodelay(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for TcpStream {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub struct TcpListener(!);

impl TcpListener {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<TcpListener> {
        unsupported()
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<TcpListener> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn set_only_v6(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn only_v6(&self) -> io::Result<bool> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for TcpListener {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub struct UdpSocket(!);

impl UdpSocket {
    pub fn bind(_: io::Result<&SocketAddr>) -> io::Result<UdpSocket> {
        unsupported()
    }

    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn socket_addr(&self) -> io::Result<SocketAddr> {
        self.0
    }

    pub fn recv_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0
    }

    pub fn peek_from(&self, _: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.0
    }

    pub fn send_to(&self, _: &[u8], _: &SocketAddr) -> io::Result<usize> {
        self.0
    }

    pub fn duplicate(&self) -> io::Result<UdpSocket> {
        self.0
    }

    pub fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        self.0
    }

    pub fn read_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn write_timeout(&self) -> io::Result<Option<Duration>> {
        self.0
    }

    pub fn set_broadcast(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn broadcast(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_multicast_loop_v4(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn multicast_loop_v4(&self) -> io::Result<bool> {
        self.0
    }

    pub fn set_multicast_ttl_v4(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn multicast_ttl_v4(&self) -> io::Result<u32> {
        self.0
    }

    pub fn set_multicast_loop_v6(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn multicast_loop_v6(&self) -> io::Result<bool> {
        self.0
    }

    pub fn join_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0
    }

    pub fn join_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn leave_multicast_v4(&self, _: &Ipv4Addr, _: &Ipv4Addr) -> io::Result<()> {
        self.0
    }

    pub fn leave_multicast_v6(&self, _: &Ipv6Addr, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn set_ttl(&self, _: u32) -> io::Result<()> {
        self.0
    }

    pub fn ttl(&self) -> io::Result<u32> {
        self.0
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        self.0
    }

    pub fn set_nonblocking(&self, _: bool) -> io::Result<()> {
        self.0
    }

    pub fn recv(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn peek(&self, _: &mut [u8]) -> io::Result<usize> {
        self.0
    }

    pub fn send(&self, _: &[u8]) -> io::Result<usize> {
        self.0
    }

    pub fn connect(&self, _: io::Result<&SocketAddr>) -> io::Result<()> {
        self.0
    }
}

impl fmt::Debug for UdpSocket {
    fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
    }
}

pub struct LookupHost(!);

impl LookupHost {
    pub fn port(&self) -> u16 {
        self.0
    }
}

impl Iterator for LookupHost {
    type Item = SocketAddr;
    fn next(&mut self) -> Option<SocketAddr> {
        self.0
    }
}

impl TryFrom<&str> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: &str) -> io::Result<LookupHost> {
        unsupported()
    }
}

impl<'a> TryFrom<(&'a str, u16)> for LookupHost {
    type Error = io::Error;

    fn try_from(_v: (&'a str, u16)) -> io::Result<LookupHost> {
        unsupported()
    }
}

pub struct Socket(FileDesc);

pub fn init() {}

pub fn cvt_gai(err: c_int) -> io::Result<()> {
    if err == 0 {
        return Ok(());
    }

    // We may need to trigger a gnetc workaround. See on_resolver_failure() for details.
    on_resolver_failure();

    #[cfg(not(target_os = "espidf"))]
    if err == netc::EAI_SYSTEM {
        return Err(io::Error::last_os_error());
    }

    #[cfg(not(target_os = "espidf"))]
    let detail = unsafe {
        str::from_utf8(CStr::from_ptr(netc::gai_strerror(err)).to_bytes()).unwrap().to_owned()
    };

    #[cfg(target_os = "espidf")]
    let detail = "";

    Err(io::Error::new(
        io::ErrorKind::Uncategorized,
        &format!("failed to lookup address information: {detail}")[..],
    ))
}

impl Socket {
    pub fn new(addr: &SocketAddr, ty: c_int) -> io::Result<Socket> {
        let fam = match *addr {
            SocketAddr::V4(..) => netc::AF_INET,
            SocketAddr::V6(..) => netc::AF_INET6,
        };
        Socket::new_raw(fam, ty)
    }

    pub fn new_raw(fam: c_int, ty: c_int) -> io::Result<Socket> {
        unsafe {
            cfg_if::cfg_if! {
                if #[cfg(any(
                    target_os = "android",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "illumos",
                    target_os = "linux",
                    target_os = "netbsd",
                    target_os = "openbsd",
                                    ))] {
                    // On platforms that support it we pass the SOCK_CLOEXEC
                    // flag to atomically create the socket and set it as
                    // CLOEXEC. On Linux this was added in 2.6.27.
                    let fd = cvt(netc::socket(fam, ty | netc::SOCK_CLOEXEC, 0))?;
                    Ok(Socket(FileDesc::from_raw_fd(fd)))
                } else {
                    let fd = cvt(netc::socket(fam, ty, 0))?;
                    let fd = FileDesc::from_raw_fd(fd);
                    fd.set_cloexec()?;
                    let socket = Socket(fd);

                    // macOS and iOS use `SO_NOSIGPIPE` as a `setsockopt`
                    // flag to disable `SIGPIPE` emission on socket.
                    #[cfg(target_vendor = "apple")]
                    setsockopt(&socket, netc::SOL_SOCKET, netc::SO_NOSIGPIPE, 1)?;

                    Ok(socket)
                }
            }
        }
    }

    #[cfg(not(target_os = "vxworks"))]
    pub fn new_pair(fam: c_int, ty: c_int) -> io::Result<(Socket, Socket)> {
        unsafe {
            let mut fds = [0, 0];

            cfg_if::cfg_if! {
                if #[cfg(any(
                    target_os = "android",
                    target_os = "dragonfly",
                    target_os = "freebsd",
                    target_os = "illumos",
                    target_os = "linux",
                    target_os = "netbsd",
                    target_os = "openbsd",

                ))] {
                    // Like above, set cloexec atomically
                    cvt(netc::socketpair(fam, ty | netc::SOCK_CLOEXEC, 0, fds.as_mut_ptr()))?;
                    Ok((Socket(FileDesc::from_raw_fd(fds[0])), Socket(FileDesc::from_raw_fd(fds[1]))))
                } else {
                    cvt(netc::socketpair(fam, ty, 0, fds.as_mut_ptr()))?;
                    let a = FileDesc::from_raw_fd(fds[0]);
                    let b = FileDesc::from_raw_fd(fds[1]);
                    a.set_cloexec()?;
                    b.set_cloexec()?;
                    Ok((Socket(a), Socket(b)))
                }
            }
        }
    }

    #[cfg(target_os = "vxworks")]
    pub fn new_pair(_fam: c_int, _ty: c_int) -> io::Result<(Socket, Socket)> {
        unimplemented!()
    }

    pub fn connect_timeout(&self, addr: &SocketAddr, timeout: Duration) -> io::Result<()> {
        self.set_nonblocking(true)?;
        let r = unsafe {
            let (addrp, len) = addr.into_inner();
            cvt(netc::connect(self.as_raw_fd(), addrp, len))
        };
        self.set_nonblocking(false)?;

        match r {
            Ok(_) => return Ok(()),
            // there's no ErrorKind for EINPROGRESS :(
            Err(ref e) if e.raw_os_error() == Some(netc::EINPROGRESS) => {}
            Err(e) => return Err(e),
        }

        let mut pollfd = netc::pollfd { fd: self.as_raw_fd(), events: netc::POLLOUT, revents: 0 };

        if timeout.as_secs() == 0 && timeout.subsec_nanos() == 0 {
            return Err(io::const_io_error!(
                io::ErrorKind::InvalidInput,
                "cannot set a 0 duration timeout",
            ));
        }

        let start = Instant::now();

        loop {
            let elapsed = start.elapsed();
            if elapsed >= timeout {
                return Err(io::const_io_error!(io::ErrorKind::TimedOut, "connection timed out"));
            }

            let timeout = timeout - elapsed;
            let mut timeout = timeout
                .as_secs()
                .saturating_mul(1_000)
                .saturating_add(timeout.subsec_nanos() as u64 / 1_000_000);
            if timeout == 0 {
                timeout = 1;
            }

            let timeout = cmp::min(timeout, c_int::MAX as u64) as c_int;

            match unsafe { netc::poll(&mut pollfd, 1, timeout) } {
                -1 => {
                    let err = io::Error::last_os_error();
                    if err.kind() != io::ErrorKind::Interrupted {
                        return Err(err);
                    }
                }
                0 => {}
                _ => {
                    // linux returns POLLOUT|POLLERR|POLLHUP for refused connections (!), so look
                    // for POLLHUP rather than read readiness
                    if pollfd.revents & netc::POLLHUP != 0 {
                        let e = self.take_error()?.unwrap_or_else(|| {
                            io::const_io_error!(
                                io::ErrorKind::Uncategorized,
                                "no error set after POLLHUP",
                            )
                        });
                        return Err(e);
                    }

                    return Ok(());
                }
            }
        }
    }

    pub fn accept(&self, storage: *mut sockaddr, len: *mut socklen_t) -> io::Result<Socket> {
        // Unfortunately the only known way right now to accept a socket and
        // atomically set the CLOEXEC flag is to use the `accept4` syscall on
        // platforms that support it. On Linux, this was added in 2.6.28,
        // gnetc 2.10 and musl 0.9.5.
        cfg_if::cfg_if! {
            if #[cfg(any(
                target_os = "android",
                target_os = "dragonfly",
                target_os = "freebsd",
                target_os = "illumos",
                target_os = "linux",
                target_os = "netbsd",
                target_os = "openbsd",
                            ))] {
                unsafe {
                    let fd = cvt_r(|| netc::accept4(self.as_raw_fd(), storage, len, netc::SOCK_CLOEXEC))?;
                    Ok(Socket(FileDesc::from_raw_fd(fd)))
                }
            } else {
                unsafe {
                    let fd = cvt_r(|| netc::accept(self.as_raw_fd(), storage, len))?;
                    let fd = FileDesc::from_raw_fd(fd);
                    fd.set_cloexec()?;
                    Ok(Socket(fd))
                }
            }
        }
    }

    pub fn duplicate(&self) -> io::Result<Socket> {
        self.0.duplicate().map(Socket)
    }

    fn recv_with_flags(&self, buf: &mut [u8], flags: c_int) -> io::Result<usize> {
        let ret = cvt(unsafe {
            netc::recv(self.as_raw_fd(), buf.as_mut_ptr() as *mut c_void, buf.len(), flags)
        })?;
        Ok(ret as usize)
    }

    pub fn read(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv_with_flags(buf, 0)
    }

    pub fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        self.recv_with_flags(buf, MSG_PEEK)
    }

    pub fn read_vectored(&self, bufs: &mut [IoSliceMut<'_>]) -> io::Result<usize> {
        self.0.read_vectored(bufs)
    }

    #[inline]
    pub fn is_read_vectored(&self) -> bool {
        self.0.is_read_vectored()
    }

    fn recv_from_with_flags(
        &self,
        buf: &mut [u8],
        flags: c_int,
    ) -> io::Result<(usize, SocketAddr)> {
        let mut storage: netc::sockaddr_storage = unsafe { mem::zeroed() };
        let mut addrlen = mem::size_of_val(&storage) as netc::socklen_t;

        let n = cvt(unsafe {
            netc::recvfrom(
                self.as_raw_fd(),
                buf.as_mut_ptr() as *mut c_void,
                buf.len(),
                flags,
                &mut storage as *mut _ as *mut _,
                &mut addrlen,
            )
        })?;
        Ok((n as usize, sockaddr_to_addr(&storage, addrlen as usize)?))
    }

    pub fn recv_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.recv_from_with_flags(buf, 0)
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    pub fn recv_msg(&self, msg: &mut netc::msghdr) -> io::Result<usize> {
        let n = cvt(unsafe { netc::recvmsg(self.as_raw_fd(), msg, netc::MSG_CMSG_CLOEXEC) })?;
        Ok(n as usize)
    }

    pub fn peek_from(&self, buf: &mut [u8]) -> io::Result<(usize, SocketAddr)> {
        self.recv_from_with_flags(buf, MSG_PEEK)
    }

    pub fn write(&self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    pub fn write_vectored(&self, bufs: &[IoSlice<'_>]) -> io::Result<usize> {
        self.0.write_vectored(bufs)
    }

    #[inline]
    pub fn is_write_vectored(&self) -> bool {
        self.0.is_write_vectored()
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    pub fn send_msg(&self, msg: &mut netc::msghdr) -> io::Result<usize> {
        let n = cvt(unsafe { netc::sendmsg(self.as_raw_fd(), msg, 0) })?;
        Ok(n as usize)
    }

    pub fn set_timeout(&self, dur: Option<Duration>, kind: netc::c_int) -> io::Result<()> {
        let timeout = match dur {
            Some(dur) => {
                if dur.as_secs() == 0 && dur.subsec_nanos() == 0 {
                    return Err(io::const_io_error!(
                        io::ErrorKind::InvalidInput,
                        "cannot set a 0 duration timeout",
                    ));
                }

                let secs = if dur.as_secs() > netc::time_t::MAX as u64 {
                    netc::time_t::MAX
                } else {
                    dur.as_secs() as netc::time_t
                };
                let mut timeout = netc::timeval {
                    tv_sec: secs,
                    tv_usec: dur.subsec_micros() as netc::suseconds_t,
                };
                if timeout.tv_sec == 0 && timeout.tv_usec == 0 {
                    timeout.tv_usec = 1;
                }
                timeout
            }
            None => netc::timeval { tv_sec: 0, tv_usec: 0 },
        };
        setsockopt(self, netc::SOL_SOCKET, kind, timeout)
    }

    pub fn timeout(&self, kind: netc::c_int) -> io::Result<Option<Duration>> {
        let raw: netc::timeval = getsockopt(self, netc::SOL_SOCKET, kind)?;
        if raw.tv_sec == 0 && raw.tv_usec == 0 {
            Ok(None)
        } else {
            let sec = raw.tv_sec as u64;
            let nsec = (raw.tv_usec as u32) * 1000;
            Ok(Some(Duration::new(sec, nsec)))
        }
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        let how = match how {
            Shutdown::Write => netc::SHUT_WR,
            Shutdown::Read => netc::SHUT_RD,
            Shutdown::Both => netc::SHUT_RDWR,
        };
        cvt(unsafe { netc::shutdown(self.as_raw_fd(), how) })?;
        Ok(())
    }

    pub fn set_linger(&self, linger: Option<Duration>) -> io::Result<()> {
        let linger = netc::linger {
            l_onoff: linger.is_some() as netc::c_int,
            l_linger: linger.unwrap_or_default().as_secs() as netc::c_int,
        };

        setsockopt(self, netc::SOL_SOCKET, netc::SO_LINGER, linger)
    }

    pub fn linger(&self) -> io::Result<Option<Duration>> {
        let val: netc::linger = getsockopt(self, netc::SOL_SOCKET, netc::SO_LINGER)?;

        Ok((val.l_onoff != 0).then(|| Duration::from_secs(val.l_linger as u64)))
    }

    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        setsockopt(self, netc::IPPROTO_TCP, netc::TCP_NODELAY, nodelay as c_int)
    }

    pub fn nodelay(&self) -> io::Result<bool> {
        let raw: c_int = getsockopt(self, netc::IPPROTO_TCP, netc::TCP_NODELAY)?;
        Ok(raw != 0)
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    pub fn set_passcred(&self, passcred: bool) -> io::Result<()> {
        setsockopt(self, netc::SOL_SOCKET, netc::SO_PASSCRED, passcred as netc::c_int)
    }

    #[cfg(any(target_os = "android", target_os = "linux"))]
    pub fn passcred(&self) -> io::Result<bool> {
        let passcred: netc::c_int = getsockopt(self, netc::SOL_SOCKET, netc::SO_PASSCRED)?;
        Ok(passcred != 0)
    }

    #[cfg(target_os = "netbsd")]
    pub fn set_passcred(&self, passcred: bool) -> io::Result<()> {
        setsockopt(self, 0 as netc::c_int, netc::LOCAL_CREDS, passcred as netc::c_int)
    }

    #[cfg(target_os = "netbsd")]
    pub fn passcred(&self) -> io::Result<bool> {
        let passcred: netc::c_int = getsockopt(self, 0 as netc::c_int, netc::LOCAL_CREDS)?;
        Ok(passcred != 0)
    }

    #[cfg(not(any(target_os = "solaris", target_os = "illumos")))]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        let mut nonblocking = nonblocking as netc::c_int;
        cvt(unsafe { netc::ioctl(self.as_raw_fd(), netc::FIONBIO, &mut nonblocking) }).map(drop)
    }

    #[cfg(any(target_os = "solaris", target_os = "illumos"))]
    pub fn set_nonblocking(&self, nonblocking: bool) -> io::Result<()> {
        // FIONBIO is inadequate for sockets on illumos/Solaris, so use the
        // fcntl(F_[GS]ETFL)-based method provided by FileDesc instead.
        self.0.set_nonblocking(nonblocking)
    }

    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        let raw: c_int = getsockopt(self, netc::SOL_SOCKET, netc::SO_ERROR)?;
        if raw == 0 { Ok(None) } else { Ok(Some(io::Error::from_raw_os_error(raw as i32))) }
    }

    // This is used by sys_common code to abstract over Windows and Unix.
    pub fn as_raw(&self) -> RawFd {
        self.as_raw_fd()
    }
}

impl AsInner<FileDesc> for Socket {
    fn as_inner(&self) -> &FileDesc {
        &self.0
    }
}

impl IntoInner<FileDesc> for Socket {
    fn into_inner(self) -> FileDesc {
        self.0
    }
}

impl FromInner<FileDesc> for Socket {
    fn from_inner(file_desc: FileDesc) -> Self {
        Self(file_desc)
    }
}

impl AsFd for Socket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        self.0.as_fd()
    }
}

impl AsRawFd for Socket {
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

impl IntoRawFd for Socket {
    fn into_raw_fd(self) -> RawFd {
        self.0.into_raw_fd()
    }
}

impl FromRawFd for Socket {
    unsafe fn from_raw_fd(raw_fd: RawFd) -> Self {
        Self(FromRawFd::from_raw_fd(raw_fd))
    }
}

fn on_resolver_failure() {}
