//! Custom Fd to manage multiple read/write operations on a file descriptor.

use libc::*;
use std::{
    fs::File,
    io::Error,
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
    pub async fn read_exact(&self, file_offset: usize, length: usize) -> Result<Vec<u8>> {
        let fd = self.fd;
        tokio::task::spawn_blocking(move || {
            let mut buf = vec![0; length];
            Self::read(fd, buf.as_mut_ptr() as _, buf.len(), file_offset)?;
            Ok(buf)
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
                println!("buf_offset: {}", buf_offset);
                println!("buf_len: {}", buf_len);
                println!("offset: {}", offset);
                println!("file_len: {}", file_len(fd).unwrap());
                eprintln!("read_exact: Unexpected EOF");
                return Err(Error::from(std::io::ErrorKind::UnexpectedEof));
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
        file_len(self.fd())
    }
}

fn file_len(fd: i32) -> Result<usize> {
    unsafe {
        let mut stat = std::mem::MaybeUninit::<libc::stat64>::uninit();
        if fstat64(fd, stat.as_mut_ptr()) < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(stat.assume_init().st_size as usize)
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
