mod heap_archived;
mod kv_store;
mod value_buffer;

use anyhow::Result;
pub use heap_archived::*;
pub use kv_store::KvStore;
pub use value_buffer::*;

pub async fn init(
    s3_client: aws_sdk_s3::Client,
    bucket_name: String,
    turn_on_in_memory_cache: bool,
) -> Result<Database> {
    let sqlite = kv_store::SqliteKvStore::new(s3_client, bucket_name).await?;
    let store = kv_store::InMemoryCachedKsStore::new(sqlite, turn_on_in_memory_cache);

    Ok(Database { store })
}

#[derive(Clone)]
pub struct Database {
    store: kv_store::InMemoryCachedKsStore<kv_store::SqliteKvStore>,
}
impl Database {
    pub fn set_memory_cache(&self, turn_on: bool) {
        self.store.set_cache_enabled(turn_on)
    }
}

impl KvStore for Database {
    async fn get(&self, key: impl AsRef<str>) -> Result<Option<ValueBuffer>> {
        self.store.get(key).await
    }

    async fn get_with_expiration(
        &self,
        key: impl AsRef<str>,
    ) -> Result<Option<(ValueBuffer, Option<std::time::SystemTime>)>> {
        self.store.get_with_expiration(key).await
    }

    async fn put(
        &self,
        key: impl AsRef<str>,
        value: &impl AsRef<[u8]>,
        ttl: Option<std::time::Duration>,
    ) -> Result<()> {
        self.store.put(key, value, ttl).await
    }

    async fn delete(&self, key: impl AsRef<str>) -> Result<()> {
        self.store.delete(key).await
    }

    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        key: impl AsRef<str>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<std::time::Duration>,
    ) -> Result<()> {
        self.store.create(key, value_fn, ttl).await
    }
}
