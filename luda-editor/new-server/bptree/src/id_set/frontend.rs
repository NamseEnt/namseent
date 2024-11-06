use super::*;
use std::{
    collections::{hash_map::Entry, HashMap},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock, Weak},
};
use tokio::sync::{mpsc, oneshot};

/// Frontend for the IdSet data structure.
#[derive(Clone)]
pub struct IdSet {
    path: PathBuf,
    request_tx: Arc<mpsc::Sender<Request>>,
}

type OpenedPaths = HashMap<PathBuf, Weak<IdSet>>;
static OPENED_PATHS: OnceLock<Arc<Mutex<OpenedPaths>>> = OnceLock::new();

impl IdSet {
    pub fn new(path: impl AsRef<Path>) -> Result<Arc<Self>> {
        let path = path.as_ref();

        let (request_tx, request_rx) = mpsc::channel(4096);

        let this = Arc::new(Self {
            path: path.to_path_buf(),
            request_tx: Arc::new(request_tx),
        });

        {
            match OPENED_PATHS
                .get_or_init(Default::default)
                .lock()
                .unwrap()
                .entry(this.path.clone())
            {
                Entry::Occupied(_) => {
                    return Err(anyhow::anyhow!("IdSet already opened at path: {:?}", path));
                }
                Entry::Vacant(entry) => {
                    entry.insert(Arc::downgrade(&this));
                }
            }
        }

        backend::Backend::open(&this.path, request_rx)?;

        Ok(this)
    }
    pub async fn insert(&self, id: u128) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        self.request_tx
            .send(Request::Insert { id, tx })
            .await
            .map_err(|_| anyhow::anyhow!("IdSet backend is down"))?;

        match rx
            .await
            .map_err(|_| anyhow::anyhow!("Failed to received result from rx, id: {}", id))?
        {
            true => Ok(()),
            false => Err(anyhow::anyhow!("Failed to insert id: {}", id)),
        }
    }
    pub async fn delete(&self, id: u128) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        self.request_tx
            .send(Request::Delete { id, tx })
            .await
            .map_err(|_| anyhow::anyhow!("IdSet backend is down"))?;

        match rx
            .await
            .map_err(|_| anyhow::anyhow!("Failed to received result from rx, id: {}", id))?
        {
            true => Ok(()),
            false => Err(anyhow::anyhow!("Failed to delete id: {}", id)),
        }
    }
}

impl Drop for IdSet {
    fn drop(&mut self) {
        OPENED_PATHS
            .get()
            .unwrap()
            .lock()
            .unwrap()
            .remove(&self.path);
    }
}
