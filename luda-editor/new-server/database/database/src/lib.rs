mod document_store;

pub use document::*;
pub use document_store::DocumentStore;
pub use migration::schema;

pub async fn init(
    s3_client: aws_sdk_s3::Client,
    bucket_name: String,
    turn_on_in_memory_cache: bool,
) -> anyhow::Result<Database> {
    let sqlite = document_store::SqliteKvStore::new(s3_client, bucket_name).await?;
    let store = document_store::InMemoryCachedKsStore::new(sqlite, turn_on_in_memory_cache);

    Ok(Database { store })
}

#[derive(Clone)]
pub struct Database {
    store: document_store::InMemoryCachedKsStore<document_store::SqliteKvStore>,
}
impl Database {
    pub fn set_memory_cache(&self, turn_on: bool) {
        self.store.set_cache_enabled(turn_on)
    }
    pub async fn get<T: Document>(
        &self,
        document_get: impl DocumentGet<Output = T>,
    ) -> Result<Option<HeapArchived<T>>> {
        let value_buffer = self
            .store
            .get(
                T::name(),
                &document_get.pk()?,
                document_get.sk()?.as_deref(),
            )
            .await?;
        Ok(value_buffer.map(|value_buffer| T::heap_archived(value_buffer)))
    }
    pub async fn query<T: Document>(
        &self,
        document_query: impl DocumentQuery<Output = T>,
    ) -> Result<Vec<HeapArchived<T>>> {
        let value_buffer = self.store.query(T::name(), &document_query.pk()?).await?;
        Ok(value_buffer
            .into_iter()
            .map(|value_buffer| T::heap_archived(value_buffer))
            .collect())
    }
    pub async fn transact<'a, AbortReason>(
        &'a self,
        transact: impl Transact<'a, AbortReason> + 'a + Send,
    ) -> Result<MaybeAborted<AbortReason>> {
        let mut transact_items = transact.try_into_transact_items()?;
        self.store.transact(&mut transact_items).await
    }
    pub async fn wait_backup(&self) -> Result<()> {
        self.store.wait_backup().await
    }
}

#[derive(Debug)]
pub enum Error {
    SqliteError(rusqlite::Error),
    SerializationError(SerErr),
    AlreadyExistsOnCreate,
    NotExistsOnUpdate,
    BackupAborted(String),
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

pub enum MaybeAborted<AbortReason> {
    Aborted { reason: AbortReason },
    No,
}

impl<AbortReason> MaybeAborted<AbortReason> {
    fn is_aborted(&self) -> bool {
        matches!(self, MaybeAborted::Aborted { .. })
    }

    pub fn err_if_aborted<Err>(
        self,
        func: impl FnOnce(AbortReason) -> Err,
    ) -> std::result::Result<(), Err> {
        match self {
            MaybeAborted::Aborted { reason } => Err(func(reason)),
            MaybeAborted::No => Ok(()),
        }
    }
}

impl MaybeAborted<()> {
    pub fn unwrap(self) {
        match self {
            MaybeAborted::Aborted { .. } => unreachable!("You should make AbortReason generic"),
            MaybeAborted::No => (),
        }
    }
}
