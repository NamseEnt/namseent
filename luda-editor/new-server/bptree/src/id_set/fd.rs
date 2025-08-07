//! Custom Fd to manage multiple read/write operations on a file descriptor.

use super::{PAGE_LEN, PageOffset};
use libc::*;
use std::{
    fs::File,
    io::{self, Error},
    mem::MaybeUninit,
    os::fd::{IntoRawFd, RawFd},
};

type Result<T> = std::io::Result<T>;

pub fn split_file(file: File) -> (ReadFd, WriteFd) {
    let fd = file.into_raw_fd();
    (ReadFd { fd }, WriteFd { fd })
}

#[derive(Debug, Clone)]
pub struct ReadFd {
    fd: RawFd,
}

impl ReadFd {
    pub async fn read_init<T: Send + 'static>(&self, offset: usize) -> Result<T> {
        let size = size_of::<T>();
        if size == 0 {
            return Ok(unsafe { std::mem::zeroed() });
        }

        let fd = self.fd;
        tokio::task::spawn_blocking(move || {
            let mut t = MaybeUninit::<T>::uninit();
            Self::read(fd, t.as_mut_ptr() as _, size, offset)?;
            Ok(unsafe { t.assume_init() })
        })
        .await?
    }

    pub async fn read_page(&self, page_offset: PageOffset) -> Result<[u8; PAGE_LEN]> {
        let fd = self.fd;
        tokio::task::spawn_blocking(move || {
            let mut buf = std::mem::MaybeUninit::<[u8; PAGE_LEN]>::uninit();
            let buf_ptr = buf.as_mut_ptr();

            Self::read(fd, buf_ptr as _, PAGE_LEN, page_offset.file_offset())?;

            Ok(unsafe { buf.assume_init() })
        })
        .await?
    }

    fn read(fd: i32, buf_ptr: *mut u8, buf_len: usize, offset: usize) -> Result<()> {
        let mut buf_offset = 0;
        while buf_offset < buf_len {
            let len = unsafe {
                let count = buf_len - buf_offset;
                pread(
                    fd,
                    buf_ptr.add(buf_offset) as _,
                    count as _,
                    (offset + buf_offset) as i64,
                )
            };
            if len < 0 {
                return Err(Error::last_os_error());
            }
            if len == 0 {
                return Err(Error::from(io::ErrorKind::UnexpectedEof));
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
                return Err(Error::last_os_error());
            }
            assert_ne!(len, 0);
            buf_offset += len as usize;
        }

        Ok(())
    }

    pub fn set_len(&mut self, len: usize) -> Result<()> {
        if unsafe { ftruncate(self.fd, len as _) } < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }

    pub fn fsync(&mut self) -> Result<()> {
        if unsafe { fsync(self.fd) } < 0 {
            Err(Error::last_os_error())
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
                return Err(Error::last_os_error());
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
                Err(Error::last_os_error())
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
