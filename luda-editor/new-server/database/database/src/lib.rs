mod heap_archived;
mod kv_store;
mod value_buffer;

use anyhow::Result;
pub use heap_archived::*;
pub use kv_store::KvStore;
pub use value_buffer::*;

pub async fn init(s3_client: aws_sdk_s3::Client, bucket_name: String) -> Result<Database> {
    let sqlite = kv_store::SqliteKvStore::new(s3_client, bucket_name).await?;

    todo!()
}

#[derive(Clone)]
pub struct Database {}
impl Database {
    pub fn set_memory_cache(&self, turn_on: bool) {
        todo!()
    }
}

impl KvStore for Database {
    async fn get(&self, key: impl AsRef<str>) -> Result<Option<ValueBuffer>> {
        todo!()
    }

    async fn get_with_expiration(
        &self,
        key: impl AsRef<str>,
    ) -> Result<Option<(ValueBuffer, Option<std::time::SystemTime>)>> {
        todo!()
    }

    async fn put(
        &self,
        key: impl AsRef<str>,
        value: &impl AsRef<[u8]>,
        ttl: Option<std::time::Duration>,
    ) -> Result<()> {
        todo!()
    }

    async fn delete(&self, key: impl AsRef<str>) -> Result<()> {
        todo!()
    }

    async fn create<Bytes: AsRef<[u8]>>(
        &self,
        key: impl AsRef<str>,
        value_fn: impl FnOnce() -> Result<Bytes>,
        ttl: Option<std::time::Duration>,
    ) -> Result<()> {
        todo!()
    }
}
