use super::*;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{mpsc, oneshot};

/// Frontend for the IdSet data structure.
pub struct IdSet {
    path: PathBuf,
    request_tx: Arc<mpsc::Sender<Request>>,
    cache: PageCache,
    backend_close_rx: oneshot::Receiver<()>,
}

impl IdSet {
    /// - `path`
    ///   - The path to the file where the data is stored.
    ///   - **Make sure no one is using this file.**
    /// - `cache_limit`
    ///   - 1 cache is 4KB. 100 `cache_limit` will be 400KB.
    ///   - Put enough `cache_limit`.
    ///     - If `IdSet` cannot find data from cache, it will read from disk, which is very slow.
    pub fn new(path: impl AsRef<Path>, cache_limit: usize) -> Result<Arc<Self>> {
        let path = path.as_ref();

        let (request_tx, request_rx) = mpsc::channel(4096);
        let (backend_close_tx, backend_close_rx) = oneshot::channel();

        let this = Arc::new(Self {
            path: path.to_path_buf(),
            request_tx: Arc::new(request_tx),
            cache: PageCache::new(cache_limit),
            backend_close_rx,
        });

        Backend::open(&this.path, request_rx, this.cache.clone(), backend_close_tx)?;

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
    pub async fn try_close(self: Arc<Self>) -> Result<(), Arc<Self>> {
        let inner = Arc::try_unwrap(self)?;

        if inner.request_tx.send(Request::Close).await.is_err() {
            return Ok(());
        }

        _ = inner.backend_close_rx.await;

        Ok(())
    }
}
