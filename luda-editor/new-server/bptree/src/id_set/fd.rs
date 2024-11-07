//! Custom Fd to manage multiple read/write operations on a file descriptor.

use libc::*;
use std::{
    fs::File,
    io::{self, Error},
    os::fd::{IntoRawFd, RawFd},
};

type Result<T> = anyhow::Result<T>;

pub fn split_file(file: File) -> (ReadFd, WriteFd) {
    let fd = file.into_raw_fd();
    (ReadFd { fd }, WriteFd { fd })
}

#[derive(Debug, Clone)]
pub struct ReadFd {
    fd: RawFd,
}

impl ReadFd {
    pub fn read_exact(&self, buf: &mut [u8], offset: usize) -> Result<()> {
        if buf.is_empty() {
            return Ok(());
        }
        let mut buf_offset = 0;
        while buf_offset < buf.len() {
            let len = unsafe {
                let buf = &mut buf[buf_offset..];
                assert!(!buf.is_empty());
                pread(
                    self.fd,
                    buf.as_mut_ptr() as _,
                    buf.len() as _,
                    (offset + buf_offset) as i64,
                )
            };
            if len < 0 {
                return Err(Error::last_os_error().into());
            }
            if len == 0 {
                println!("self.len(): {:?}", self.len()?);
                println!("offset: {:?}", offset);
                println!("buf.len(): {:?}", buf.len());
                return Err(Error::from(io::ErrorKind::UnexpectedEof).into());
            }
            buf_offset += len as usize;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct WriteFd {
    fd: RawFd,
}

impl WriteFd {
    pub fn write_exact(&mut self, buf: &[u8], offset: usize) -> Result<()> {
        if buf.is_empty() {
            return Ok(());
        }
        let mut buf_offset = 0;
        while buf_offset < buf.len() {
            let len = unsafe {
                let buf = &buf[buf_offset..];
                pwrite(
                    self.fd,
                    buf.as_ptr() as _,
                    buf.len() as _,
                    (offset + buf_offset) as i64,
                )
            };
            if len < 0 {
                return Err(Error::last_os_error().into());
            }
            assert_ne!(len, 0);
            buf_offset += len as usize;
        }

        Ok(())
    }

    pub fn set_len(&mut self, len: usize) -> Result<()> {
        if unsafe { ftruncate(self.fd, len as _) } < 0 {
            Err(Error::last_os_error().into())
        } else {
            Ok(())
        }
    }

    pub fn fsync(&mut self) -> Result<()> {
        if unsafe { fsync(self.fd) } < 0 {
            Err(Error::last_os_error().into())
        } else {
            Ok(())
        }
    }

    pub(crate) fn copy_from(&mut self, source: &impl BorrowFd) -> Result<()> {
        let mut offset = 0;
        let count = source.len()?;

        while (offset as usize) < count {
            let len =
                unsafe { sendfile(self.fd, source.fd(), &mut offset, count - offset as usize) };

            if len < 0 {
                return Err(Error::last_os_error().into());
            }
            assert!(offset > 0);
            offset += len as i64;
            assert_ne!(len, 0);
        }

        Ok(())
    }
}

pub(crate) trait BorrowFd {
    fn fd(&self) -> RawFd;

    /// Length from metadata
    fn len(&self) -> Result<usize> {
        unsafe {
            let mut stat = std::mem::MaybeUninit::<libc::stat64>::uninit();
            if fstat64(self.fd(), stat.as_mut_ptr()) < 0 {
                Err(Error::last_os_error().into())
            } else {
                Ok(stat.assume_init().st_size as usize)
            }
        }
    }
}

impl BorrowFd for ReadFd {
    fn fd(&self) -> RawFd {
        self.fd
    }
}

impl BorrowFd for WriteFd {
    fn fd(&self) -> RawFd {
        self.fd
    }
}

impl BorrowFd for &mut WriteFd {
    fn fd(&self) -> RawFd {
        self.fd
    }
}
