use crate::system::InitResult;
use anyhow::Result;
use std::io::{Read, Write};

pub fn init() -> InitResult {
    Ok(())
}

pub fn get(key: impl AsRef<str>) -> Result<Option<Vec<u8>>> {
    let Some(mut read) = open_read(key)? else {
        return Ok(None);
    };

    let mut buffer = Vec::new();
    read.read_to_end(&mut buffer)?;
    Ok(Some(buffer))
}

pub fn set(key: impl AsRef<str>, bytes: &[u8]) -> Result<()> {
    let mut write = open_write(key)?;

    write.write_all(bytes)?;
    Ok(())
}

pub fn delete(key: impl AsRef<str>) -> Result<()> {
    let key = key.as_ref();
    unsafe { _storage_delete(key.as_ptr(), key.len()) };
    Ok(())
}

fn open_read(key: impl AsRef<str>) -> Result<Option<StorageRead>> {
    let key = key.as_ref();

    let fd = unsafe { _storage_open_read(key.as_ptr(), key.len()) };
    if fd == 0 {
        return Ok(None);
    }

    Ok(Some(StorageRead { fd }))
}

fn open_write(key: impl AsRef<str>) -> Result<StorageWrite> {
    let key = key.as_ref();

    let fd = unsafe { _storage_open_write(key.as_ptr(), key.len()) };

    Ok(StorageWrite { fd })
}

unsafe extern "C" {
    /// # Returns
    /// 0: not found
    /// non-zero: file descriptor
    fn _storage_open_read(key_ptr: *const u8, key_len: usize) -> usize;
    /// # Parameters
    /// - `is_done`:
    ///     - 0: not done
    ///     - non-zero: done
    fn _storage_read(
        fd: usize,
        buffer_ptr: *mut u8,
        buffer_len: usize,
        read_byte_length_ptr: *mut i32,
        is_done_ptr: *mut i32,
    );
    fn _storage_open_write(key_ptr: *const u8, key_len: usize) -> usize;
    fn _storage_write(fd: usize, buffer_ptr: *const u8, buffer_len: usize) -> WriteReturnCode;
    fn _storage_flush(fd: usize);
    fn _storage_close(fd: usize);
    fn _storage_delete(key_ptr: *const u8, key_len: usize);
}

#[repr(u8)]
#[allow(dead_code)]
enum WriteReturnCode {
    Success = 0,
    OutOfSpace = 1,
}

pub struct StorageRead {
    fd: usize,
}

impl Drop for StorageRead {
    fn drop(&mut self) {
        unsafe {
            _storage_close(self.fd);
        }
    }
}

impl Read for StorageRead {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let mut read_byte_length = 0;
        let mut is_done = 0;
        unsafe {
            _storage_read(
                self.fd,
                buf.as_mut_ptr(),
                buf.len(),
                &mut read_byte_length,
                &mut is_done,
            );
        }

        Ok(read_byte_length as usize)
    }
}

pub struct StorageWrite {
    fd: usize,
}

impl Drop for StorageWrite {
    fn drop(&mut self) {
        unsafe {
            _storage_close(self.fd);
        }
    }
}

impl Write for StorageWrite {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let write_return_code = unsafe { _storage_write(self.fd, buf.as_ptr(), buf.len()) };

        match write_return_code {
            WriteReturnCode::Success => Ok(buf.len()),
            // Stuck on https://github.com/rust-lang/rust/issues/86442
            WriteReturnCode::OutOfSpace => Err(std::io::Error::other("Out of space")),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        unsafe { _storage_flush(self.fd) };
        Ok(())
    }
}
