mod in_memory;
mod sqlite;

use crate::*;
use anyhow::Result;
pub(crate) use in_memory::*;
pub(crate) use sqlite::*;
use std::time::{Duration, SystemTime};

/// * `ttl` - Minimum resolution: seconds
#[allow(async_fn_in_trait)]
pub trait KvStore {
    async fn get(&self, key: impl AsRef<str>) -> Result<Option<ValueBuffer>>;
    async fn get_with_expiration(
        &self,
        key: impl AsRef<str>,
    ) -> Result<Option<(ValueBuffer, Option<SystemTime>)>>;
    async fn put(
        &self,
        key: impl AsRef<str>,
        value: &impl AsRef<[u8]>,
        ttl: Option<Duration>,
    ) -> Result<()>;
    async fn delete(&self, key: impl AsRef<str>) -> Result<()>;
    // /// optimistic locking update
    // /// Return value: true if the update was successful, false if conflict
    // async fn update<T, Fut>(
    //     &self,
    //     key: impl AsRef<str>,
    //     update: impl FnOnce(Option<HeapArchived<T>>) -> Fut,
    // ) -> Result<bool>
    // where
    //     T: rkyv::Archive + rkyv::Serialize<AllocSerializer<64>>,
    //     Fut: Future<Output = Option<Option<T>>>;
    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        key: impl AsRef<str>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<Duration>,
    ) -> Result<()>;
}
