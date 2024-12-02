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
    ) -> Result<Option<T>> {
        let Some(value_buffer) = self.store.get(T::name(), document_get.id()).await? else {
            return Ok(None);
        };
        let deserialized = T::from_slice(&value_buffer)?;
        Ok(Some(deserialized))
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

#[cfg(test)]
mod test {
    use super::*;

    #[derive(serde::Serialize, serde::Deserialize)]
    struct Doc {
        id: u128,
    }
    impl Document for Doc {
        fn name() -> &'static str {
            "Doc"
        }

        fn from_slice(bytes: &[u8]) -> document::Result<Self>
        where
            Self: Sized,
        {
            serde_json::from_slice(bytes)
        }

        fn to_bytes(&self) -> document::Result<Vec<u8>> {
            serde_json::to_vec(self)
        }
    }
    struct DocGet {
        id: u128,
    }
    impl DocumentGet for DocGet {
        type Output = Doc;

        fn id(&self) -> u128 {
            self.id
        }
    }

    #[derive(serde::Serialize, serde::Deserialize)]
    struct DocPut {
        id: u128,
    }
    impl<'a, AbortReason> TryInto<TransactItem<'a, AbortReason>> for DocPut {
        type Error = document::SerErr;

        fn try_into(self) -> std::result::Result<TransactItem<'a, AbortReason>, Self::Error> {
            Ok(TransactItem::Put {
                name: Doc::name(),
                id: self.id,
                value: serde_json::to_vec(&self).unwrap(),
            })
        }
    }

    #[tokio::test]
    async fn get_not_exists() {
        let path = "/tmp/test_get_not_exists";
        _ = std::fs::remove_dir_all(path);
        let db = super::init(path).await.unwrap();

        let doc = db.get(DocGet { id: 0 }).await.unwrap();
        assert!(doc.is_none());
    }

    #[tokio::test]
    async fn set_and_get() {
        let path = "/tmp/set_and_get";
        _ = std::fs::remove_dir_all(path);
        let db = super::init(path).await.unwrap();

        db.transact::<()>(DocPut { id: 0 }).await.unwrap().unwrap();
        let doc = db.get(DocGet { id: 0 }).await.unwrap().unwrap();
        assert_eq!(doc.id, 0);
    }
}
