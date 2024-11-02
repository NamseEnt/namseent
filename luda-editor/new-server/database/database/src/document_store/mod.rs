// mod in_memory;
mod nfs;
// mod sqlite;

use crate::*;
// pub(crate) use in_memory::*;
pub(crate) use nfs::*;
// pub(crate) use sqlite::*;
use std::time::{Duration, SystemTime};

/// * `ttl` - Minimum resolution: seconds
#[allow(async_fn_in_trait)]
pub trait DocumentStore {
    async fn get(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
    ) -> Result<Option<ValueBuffer>>;
    async fn get_with_expiration(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
    ) -> Result<Option<(ValueBuffer, Option<SystemTime>)>>;
    async fn query(&self, name: &'static str, pk: &[u8]) -> Result<Vec<ValueBuffer>>;
    async fn put(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
        value: &impl AsRef<[u8]>,
        ttl: Option<Duration>,
    ) -> Result<()>;
    async fn delete(&self, name: &'static str, pk: &[u8], sk: Option<&[u8]>) -> Result<()>;
    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        name: &'static str,
        pk: &[u8],
        sk: Option<&[u8]>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<Duration>,
    ) -> Result<()>;
    async fn transact<'a, AbortReason>(
        &'a self,
        transact_items: &mut TransactItems<'a, AbortReason>,
    ) -> Result<MaybeAborted<AbortReason>>;
}
