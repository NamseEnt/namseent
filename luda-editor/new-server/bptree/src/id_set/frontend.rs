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
    cache: PageCache,
}

type OpenedPaths = HashMap<PathBuf, Weak<IdSet>>;
static OPENED_PATHS: OnceLock<Arc<Mutex<OpenedPaths>>> = OnceLock::new();

impl IdSet {
    /// - `cache_limit`
    ///   - 1 cache is 4KB. 100 `cache_limit` will be 400KB.
    ///   - Put enough `cache_limit`.
    ///     - If `IdSet` cannot find data from cache, it will read from disk, which is very slow.
    pub fn new(path: impl AsRef<Path>, cache_limit: usize) -> Result<Arc<Self>> {
        let path = path.as_ref();

        let (request_tx, request_rx) = mpsc::channel(4096);

        let this = Arc::new(Self {
            path: path.to_path_buf(),
            request_tx: Arc::new(request_tx),
            cache: PageCache::new(cache_limit),
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

        Backend::open(&this.path, request_rx, this.cache.clone())?;

        Ok(this)
    }
    pub async fn insert(&self, id: u128) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        self.request_tx
            .send(Request::Insert { id, tx })
            .await
            .map_err(|_| anyhow::anyhow!("IdSet backend is down"))?;

        rx.await
            .map_err(|_| anyhow::anyhow!("Failed to received result from rx, id: {}", id))?
            .map_err(|_| anyhow::anyhow!("Failed to insert id: {}", id))
    }
    pub async fn delete(&self, id: u128) -> Result<()> {
        let (tx, rx) = oneshot::channel();

        self.request_tx
            .send(Request::Delete { id, tx })
            .await
            .map_err(|_| anyhow::anyhow!("IdSet backend is down"))?;

        rx.await
            .map_err(|_| anyhow::anyhow!("Failed to received result from rx, id: {}", id))?
            .map_err(|_| anyhow::anyhow!("Failed to delete id: {}", id))
    }
    pub async fn contains(&self, id: u128) -> Result<bool> {
        if let Some(cached) = self.cache.contains_id(id) {
            return Ok(cached);
        }

        let (tx, rx) = oneshot::channel();

        self.request_tx
            .send(Request::Contains { id, tx })
            .await
            .map_err(|_| anyhow::anyhow!("IdSet backend is down"))?;

        rx.await
            .map_err(|_| anyhow::anyhow!("Failed to received result from rx, id: {}", id))?
            .map_err(|_| anyhow::anyhow!("Failed to check if id exists: {}", id))
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
