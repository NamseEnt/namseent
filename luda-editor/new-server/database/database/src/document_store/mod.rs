mod in_memory;
mod sqlite;

use crate::*;
pub(crate) use in_memory::*;
pub(crate) use sqlite::*;
use std::time::{Duration, SystemTime};

/// * `ttl` - Minimum resolution: seconds
#[allow(async_fn_in_trait)]
pub trait DocumentStore {
    async fn get(&self, name: &str, pk: &str, sk: Option<&str>) -> Result<Option<ValueBuffer>>;
    async fn get_with_expiration(
        &self,
        name: &str,
        pk: &str,
        sk: Option<&str>,
    ) -> Result<Option<(ValueBuffer, Option<SystemTime>)>>;
    async fn put(
        &self,
        name: &str,
        pk: &str,
        sk: Option<&str>,
        value: &impl AsRef<[u8]>,
        ttl: Option<Duration>,
    ) -> Result<()>;
    async fn delete(&self, name: &str, pk: &str, sk: Option<&str>) -> Result<()>;
    // /// optimistic locking update
    // /// Return value: true if the update was successful, false if conflict
    // async fn update<T, Fut>(
    //     &self,
    //     key: &str,
    //     update: impl FnOnce(Option<HeapArchived<T>>) -> Fut,
    // ) -> Result<bool>
    // where
    //     T: rkyv::Archive + rkyv::Serialize<AllocSerializer<64>>,
    //     Fut: Future<Output = Option<Option<T>>>;
    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        name: &str,
        pk: &str,
        sk: Option<&str>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<Duration>,
    ) -> Result<()>;
    async fn transact(
        &self,
        transact_items: impl IntoIterator<Item = crate::TransactItem>,
    ) -> Result<()>;
}
