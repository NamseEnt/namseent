//! Custom Fd to manage multiple read/write operations on a file descriptor.

use nix::{
    libc::{fstat, fsync, ftruncate},
    sys::uio::{pread, pwrite},
};
use std::{
    fs::File,
    os::fd::{AsRawFd, BorrowedFd, RawFd},
};

pub fn split_file(file: File) -> (ReadFd, WriteFd) {
    let fd = file.as_raw_fd();
    (ReadFd { fd }, WriteFd { fd })
}

#[derive(Clone)]
pub struct ReadFd {
    fd: RawFd,
}

impl ReadFd {
    pub fn read_exact(&self, buf: &mut [u8], offset: usize) -> std::io::Result<()> {
        if buf.is_empty() {
            return Ok(());
        }
        let mut buf_offset = 0;
        while buf_offset < buf.len() {
            let len = pread(
                unsafe { BorrowedFd::borrow_raw(self.fd) },
                buf[buf_offset..].as_mut(),
                (offset + buf_offset) as _,
            )?;
            buf_offset += len;
        }

        Ok(())
    }
}

pub struct WriteFd {
    fd: RawFd,
}

impl WriteFd {
    pub fn write_exact(&mut self, buf: &[u8], offset: usize) -> std::io::Result<()> {
        if buf.is_empty() {
            return Ok(());
        }
        let mut buf_offset = 0;
        while buf_offset < buf.len() {
            let len = pwrite(
                unsafe { BorrowedFd::borrow_raw(self.fd) },
                &buf[buf_offset..],
                (offset + buf_offset) as i64,
            )?;
            buf_offset += len;
        }

        Ok(())
    }

    pub fn set_len(&mut self, len: usize) -> std::io::Result<()> {
        let errno = unsafe { ftruncate(self.fd, len as _) };
        if errno < 0 {
            Err(std::io::Error::from_raw_os_error(errno))
        } else {
            Ok(())
        }
    }

    pub fn fsync(&mut self) -> std::io::Result<()> {
        let errno = unsafe { fsync(self.fd) };
        if errno < 0 {
            Err(std::io::Error::from_raw_os_error(errno))
        } else {
            Ok(())
        }
    }

    /// Length from metadata
    pub(crate) fn len(&mut self) -> std::io::Result<usize> {
        unsafe {
            let mut stat = std::mem::MaybeUninit::<nix::libc::stat>::uninit();
            let errno = fstat(self.fd, stat.as_mut_ptr());
            if errno < 0 {
                return Err(std::io::Error::from_raw_os_error(errno));
            }
            Ok(stat.assume_init().st_size as usize)
        }
    }
}
