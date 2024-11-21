mod document_store;
mod fs_store;

pub use document::*;
pub use document_store::DocumentStore;
use fs_store::FsStore;
pub use migration::schema;
use std::sync::Arc;

pub async fn init(mount_point: impl AsRef<std::path::Path>) -> std::io::Result<Database> {
    Ok(Database {
        store: Arc::new(FsStore::new(mount_point).await?),
    })
}

#[derive(Clone)]
pub struct Database {
    store: Arc<FsStore>,
}
impl Database {
    pub async fn get<T: Document>(
        &self,
        document_get: impl DocumentGet<Output = T>,
    ) -> Result<Option<HeapArchived<T>>> {
        let value_buffer = self.store.get(T::name(), document_get.id()).await?;
        Ok(value_buffer.map(|value_buffer| T::heap_archived(value_buffer)))
    }
    pub async fn transact<'a, AbortReason>(
        &'a self,
        transact: impl Transact<'a, AbortReason> + 'a + Send,
    ) -> Result<MaybeAborted<AbortReason>> {
        let transact_items = transact.try_into_transact_items()?;
        self.store.transact(transact_items).await
    }
}

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    SerializationError(SerErr),
    AlreadyExistsOnCreate,
    NotExistsOnUpdate,
    BackupAborted(String),
    Anyhow(anyhow::Error),
    TooManyFileOpened,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}
impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Self {
        Error::Anyhow(e)
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
