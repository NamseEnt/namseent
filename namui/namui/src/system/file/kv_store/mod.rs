//! kv store is the persistent key-value store.
//!
//! # Lock
//! kv store will be locked for get(shared) and set(exclusive) operations.
//! For wasi version, the lock is based on key because it uses OPFS on browser.
//! But non-wasi version, the lock is based on the whole store because it uses sqlite.
//!
//! # Sync fn
//! All functions are sync because it assumed that the store will be used in tokio::task::spawn_blocking context.
//!

#[cfg(not(target_os = "wasi"))]
mod posix;
#[cfg(target_os = "wasi")]
mod wasi;

#[cfg(not(target_os = "wasi"))]
use posix as inner;
#[cfg(target_os = "wasi")]
use wasi as inner;

use crate::system::InitResult;
use anyhow::Result;

pub fn init() -> InitResult {
    inner::init()
}
pub fn get(key: impl AsRef<str>) -> Result<Option<Vec<u8>>> {
    inner::get(key)
}
pub fn set(key: impl AsRef<str>, bytes: &[u8]) -> Result<()> {
    inner::set(key, bytes)
}
pub fn delete(key: impl AsRef<str>) -> Result<()> {
    inner::delete(key)
}
