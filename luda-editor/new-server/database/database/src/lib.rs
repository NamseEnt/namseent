mod document_store;
mod fs_locker_version;

pub use document::*;
pub use document_store::DocumentStore;
use fs_locker_version::FsLockerVersionDocStore;
use std::sync::Arc;

pub async fn init(mount_point: impl AsRef<std::path::Path>) -> anyhow::Result<Database> {
    todo!()
    // Ok(Database {
    //     store: document_store::NfsV4DocStore::new(mount_point),
    // })
}

#[derive(Clone)]
pub struct Database {
    store: Arc<FsLockerVersionDocStore>,
}
impl Database {
    pub async fn get<T: Document>(
        &self,
        document_get: impl DocumentGet<Output = T>,
    ) -> Result<Option<HeapArchived<T>>> {
        todo!()
        // let value_buffer = self.store.get(T::name(), &document_get.pk()?).await?;
        // Ok(value_buffer.map(|value_buffer| T::heap_archived(value_buffer)))
    }
    pub async fn query<T: Document>(
        &self,
        document_query: impl DocumentQuery<Output = T>,
    ) -> Result<Vec<HeapArchived<T>>> {
        todo!()
        // let value_buffer = self.store.query(T::name(), &document_query.pk()?).await?;
        // Ok(value_buffer
        //     .into_iter()
        //     .map(|value_buffer| T::heap_archived(value_buffer))
        //     .collect())
    }
    pub async fn transact<'a, AbortReason>(
        &'a self,
        transact: impl Transact<'a, AbortReason> + 'a + Send,
    ) -> Result<MaybeAborted<AbortReason>> {
        let mut transact_items = transact.try_into_transact_items()?;
        self.store.transact(&mut transact_items).await
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
