use super::*;
use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
    pin::pin,
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
    /// # Return
    /// - `None` if there is no more data.
    pub async fn next(&self, exclusive_start_id: Option<u128>) -> Result<Option<Vec<u128>>> {
        if let Some(cached) = self.cache.next(exclusive_start_id) {
            return Ok(cached);
        }

        let (tx, rx) = oneshot::channel();

        self.request_tx
            .send(Request::Next {
                exclusive_start_id,
                tx,
            })
            .await
            .map_err(|_| anyhow::anyhow!("IdSet backend is down"))?;

        rx.await
            .map_err(|_| {
                anyhow::anyhow!(
                    "Failed to received result from rx, exclusive_start_id: {:?}",
                    exclusive_start_id
                )
            })?
            .map_err(|_| {
                anyhow::anyhow!(
                    "Failed to get next of exclusive_start_id exists: {:?}",
                    exclusive_start_id
                )
            })
    }
    pub fn stream(self: &Arc<Self>) -> impl futures::Stream<Item = Result<Id>> + 'static + Unpin {
        #[pin_project::pin_project]
        struct State {
            exclusive_start_id: Option<Id>,
            ids: VecDeque<Id>,
        }
        Box::pin(futures::stream::unfold(
            State {
                exclusive_start_id: None,
                ids: vec![].into(),
            },
            {
                let id_set = self.clone();
                move |mut state| {
                    let id_set = id_set.clone();
                    async move {
                        if let Some(id) = state.ids.pop_front() {
                            return Some((Ok(id), state));
                        }
                        match id_set.next(state.exclusive_start_id).await {
                            Ok(ids) => match ids {
                                Some(ids) => {
                                    state.exclusive_start_id = ids.last().cloned();
                                    state.ids.extend(ids);
                                    let id = state.ids.pop_front().unwrap();
                                    Some((Ok(id), state))
                                }
                                None => None,
                            },
                            Err(err) => Some((Err(err), state)),
                        }
                    }
                }
            },
        ))
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
