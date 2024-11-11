use super::*;
use std::{
    collections::VecDeque,
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{mpsc, oneshot};

/// Frontend for the IdSet data structure.
#[derive(Clone)]
pub struct IdSet {
    path: PathBuf,
    cache: PageCache,
    request_tx: Arc<mpsc::Sender<FeBeRequest>>,
    backend_close_rx: Arc<oneshot::Receiver<()>>,
}

impl Debug for IdSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IdSet").field("path", &self.path).finish()
    }
}

impl IdSet {
    /// - `path`
    ///   - The path to the file where the data is stored.
    ///   - **Make sure no one is using this file.**
    /// - `cache_limit`
    ///   - 1 cache is 4KB. 100 `cache_limit` will be 400KB.
    ///   - Put enough `cache_limit`.
    ///     - If `IdSet` cannot find data from cache, it will read from disk, which is very slow.
    pub async fn new(path: impl AsRef<Path>, cache_limit: usize) -> Result<Self> {
        let path = path.as_ref();

        let (request_tx, request_rx) = mpsc::channel(4096);
        let (backend_close_tx, backend_close_rx) = oneshot::channel();

        let this = Self {
            path: path.to_path_buf(),
            request_tx: Arc::new(request_tx),
            cache: PageCache::new(cache_limit),
            backend_close_rx: Arc::new(backend_close_rx),
        };

        Backend::open(&this.path, request_rx, this.cache.clone(), backend_close_tx).await?;

        Ok(this)
    }
    pub async fn insert(&self, id: u128) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_request(FeBeRequest::Insert { id, tx }, rx).await
    }
    pub async fn delete(&self, id: u128) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_request(FeBeRequest::Delete { id, tx }, rx).await
    }
    pub async fn contains(&self, id: u128) -> Result<bool> {
        if let Some(cached) = self.cache.contains_id(id) {
            return Ok(cached);
        }

        let (tx, rx) = oneshot::channel();
        self.send_request(FeBeRequest::Contains { id, tx }, rx)
            .await
    }
    /// # Return
    /// - `None` if there is no more data.
    pub async fn next(&self, exclusive_start_id: Option<u128>) -> Result<Option<Vec<u128>>> {
        if let Some(cached) = self.cache.next(exclusive_start_id) {
            return Ok(cached);
        }

        let (tx, rx) = oneshot::channel();
        self.send_request(
            FeBeRequest::Next {
                exclusive_start_id,
                tx,
            },
            rx,
        )
        .await
    }
    pub fn stream(&self) -> impl futures::Stream<Item = Result<Id>> + 'static + Unpin {
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
    pub async fn try_close(self) -> std::result::Result<(), Self> {
        let Self {
            path,
            cache,
            request_tx,
            backend_close_rx,
        } = self;
        match Arc::try_unwrap(backend_close_rx) {
            Ok(backend_close_rx) => {
                if request_tx.send(FeBeRequest::Close).await.is_err() {
                    return Ok(());
                }

                _ = backend_close_rx.await;
                Ok(())
            }
            Err(backend_close_rx) => Err(Self {
                path,
                cache,
                request_tx,
                backend_close_rx,
            }),
        }
    }
    async fn send_request<T>(
        &self,
        request: FeBeRequest,
        rx: oneshot::Receiver<std::result::Result<T, ()>>,
    ) -> Result<T> {
        self.request_tx
            .send(request)
            .await
            .map_err(|_| Error::Broken)?;

        rx.await
            .map_err(|_| Error::Broken)?
            .map_err(|_| Error::Temporary)
    }
}
