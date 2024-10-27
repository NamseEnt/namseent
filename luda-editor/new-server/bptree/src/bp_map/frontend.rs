use super::*;
use bytes::Bytes;
use std::{
    collections::VecDeque,
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::{mpsc, oneshot};

/// Frontend for the keySet data structure.
#[derive(Clone)]
pub struct BpMap {
    path: PathBuf,
    cache: PageCache,
    request_tx: Arc<mpsc::Sender<FeBeRequest>>,
    backend_close_rx: Arc<oneshot::Receiver<()>>,
}

impl Debug for BpMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("keySet").field("path", &self.path).finish()
    }
}

impl BpMap {
    /// - `path`
    ///   - The path to the file where the data is stored.
    ///   - **Make sure no one is using this file.**
    /// - `page_cache_limit`
    ///   - 1 page_cache_limit = 1 page = 4KB.
    ///   - Put enough `page_cache_limit`.
    ///     - If `keySet` cannot find data from cache, it will read from disk, which is very slow.
    pub async fn new(path: impl AsRef<Path>, page_cache_limit: usize) -> std::io::Result<Self> {
        let path = path.as_ref();

        let (request_tx, request_rx) = mpsc::channel(4096);
        let (backend_close_tx, backend_close_rx) = oneshot::channel();

        let this = Self {
            path: path.to_path_buf(),
            request_tx: Arc::new(request_tx),
            cache: PageCache::new(page_cache_limit),
            backend_close_rx: Arc::new(backend_close_rx),
        };

        Backend::open(&this.path, request_rx, this.cache.clone(), backend_close_tx).await?;

        Ok(this)
    }
    pub async fn insert(&self, key: Key, value: Bytes) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_request(FeBeRequest::Insert { key, tx, value }, rx)
            .await
    }
    pub async fn delete(&self, key: Key) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.send_request(FeBeRequest::Delete { key, tx }, rx).await
    }
    pub async fn contains(&self, key: Key) -> Result<bool> {
        if let Some(cached) = self.cache.contains_key(key) {
            return Ok(cached);
        }

        let (tx, rx) = oneshot::channel();
        self.send_request(FeBeRequest::Contains { key, tx }, rx)
            .await
    }
    pub async fn get(&self, key: Key) -> Result<Option<Bytes>> {
        if let Some(cached) = self.cache.get(key) {
            return Ok(cached);
        }

        let (tx, rx) = oneshot::channel();
        self.send_request(FeBeRequest::Get { key, tx }, rx).await
    }
    /// # Return
    /// - `None` if there is no more data.
    pub async fn next(&self, exclusive_start_key: Option<Key>) -> Result<Option<Vec<Entry>>> {
        if let Some(cached) = self.cache.next(exclusive_start_key) {
            return Ok(cached);
        }

        let (tx, rx) = oneshot::channel();
        self.send_request(
            FeBeRequest::Next {
                exclusive_start_key,
                tx,
            },
            rx,
        )
        .await
    }
    pub fn stream(&self) -> impl futures::Stream<Item = Result<Entry>> + 'static + Unpin {
        struct State {
            exclusive_start_key: Option<Key>,
            entries: VecDeque<Entry>,
        }
        Box::pin(futures::stream::unfold(
            State {
                exclusive_start_key: None,
                entries: vec![].into(),
            },
            {
                let key_set = self.clone();
                move |mut state| {
                    let key_set = key_set.clone();
                    async move {
                        if let Some(key) = state.entries.pop_front() {
                            return Some((Ok(key), state));
                        }
                        match key_set.next(state.exclusive_start_key).await {
                            Ok(entries) => match entries {
                                Some(entries) => {
                                    state.exclusive_start_key = entries.last().map(|x| x.key);
                                    state.entries.extend(entries);
                                    let key = state.entries.pop_front().unwrap();
                                    Some((Ok(key), state))
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
    pub async fn file_size(&self) -> Result<usize> {
        if let Some(header) = self.cache.header() {
            return Ok(header.file_size());
        }

        let (tx, rx) = oneshot::channel();
        self.send_request(FeBeRequest::FileSize { tx }, rx).await
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
