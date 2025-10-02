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

mod wasi;

use wasi as inner;

use anyhow::Result;

pub async fn init() -> Result<()> {
    inner::init().await
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
