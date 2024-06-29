use anyhow::Result;
use crate::system::InitResult;

pub async fn init() -> InitResult {
    Ok(())
}

pub fn get(key: impl AsRef<str>) -> Result<Option<Vec<u8>>> {
    let key = key.as_ref();
    let key_ptr = key.as_ptr();
    let key_len = key.len();
    let value_len = unsafe { _local_storage_get_start(key_ptr, key_len) };
    if value_len < 0 {
        return Ok(None);
    }
    let mut buffer = vec![0; value_len as usize];
    let buffer_ptr = buffer.as_mut_ptr();
    unsafe { _local_storage_get_end(buffer_ptr) };
    Ok(Some(buffer))
}

pub fn set(key: impl AsRef<str>, content: impl AsRef<[u8]>) -> Result<()> {
    let key = key.as_ref();
    let key_ptr = key.as_ptr();
    let key_len = key.len();
    let content = content.as_ref();
    let content_ptr = content.as_ptr();
    let content_len = content.len();
    unsafe { _local_storage_set(key_ptr, key_len, content_ptr, content_len) };
    Ok(())
}

pub fn delete(key: impl AsRef<str>) -> Result<()> {
    let key = key.as_ref();
    let key_ptr = key.as_ptr();
    let key_len = key.len();
    unsafe { _local_storage_set(key_ptr, key_len, std::ptr::null(), 0) };
    Ok(())
}

extern "C" {
    /// # Returns
    /// - `-1` if the key does not exist.
    /// - non-negative integer, the byte length of the value.
    fn _local_storage_get_start(key_ptr: *const u8, key_len: usize) -> i32;
    fn _local_storage_get_end(buffer_ptr: *mut u8);
    /// # Parameters
    /// - `key_ptr` - the pointer to the key.
    /// - `key_len` - the byte length of the key.
    /// - `value_ptr` - the pointer to the value. if `value_ptr` is `NULL`, the key is deleted.
    fn _local_storage_set(
        key_ptr: *const u8,
        key_len: usize,
        value_ptr: *const u8,
        value_len: usize,
    );
}
