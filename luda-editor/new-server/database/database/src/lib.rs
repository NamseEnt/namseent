mod kv_store;

use document::*;
pub use kv_store::KvStore;
pub use migration::schema;

pub async fn init(
    s3_client: aws_sdk_s3::Client,
    bucket_name: String,
    turn_on_in_memory_cache: bool,
) -> anyhow::Result<Database> {
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
    pub async fn get<T: Document>(
        &self,
        document_get: impl DocumentGet<Output = T>,
    ) -> Result<Option<HeapArchived<T>>> {
        let key = document_get.key();
        let value_buffer = self.store.get(key).await?;
        Ok(value_buffer.map(|value_buffer| T::heap_archived(value_buffer)))
    }

    pub async fn transact(&self, transact: impl Transact) -> Result<()> {
        let transact_items = transact.try_into_transact_items()?;
        KvStore::transact(self, transact_items).await
    }
}

#[derive(Debug)]
pub enum Error {
    SqliteError(rusqlite::Error),
    SerializationError(SerErr),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

impl From<rusqlite::Error> for Error {
    fn from(e: rusqlite::Error) -> Self {
        Error::SqliteError(e)
    }
}
impl From<SerErr> for Error {
    fn from(e: SerErr) -> Self {
        Error::SerializationError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

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

    async fn transact(
        &self,
        transact_items: impl IntoIterator<Item = crate::TransactItem>,
    ) -> Result<()> {
        self.store.transact(transact_items).await
    }
}
