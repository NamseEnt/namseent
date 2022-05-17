use super::{load_bundle_metadata, make_path_dirent_list_map, LoadBundleMetadataError};
use crate::fs::types::{Dirent, PathLike};
use dashmap::{lock::RwLock, DashMap};
use futures::{future::LocalBoxFuture, Future, FutureExt};
use std::{
    path::PathBuf,
    pin::Pin,
    sync::Arc,
    task::{Poll, Waker},
};

pub struct BundleDirReader {
    path_dirent_list_map: DashMap<PathBuf, Vec<Dirent>>,
    load_state: RwLock<BundleDirReaderLoadState>,
    load_wakers: RwLock<Vec<Waker>>,
}
impl BundleDirReader {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            path_dirent_list_map: DashMap::new(),
            load_state: RwLock::new(BundleDirReaderLoadState::Unset),
            load_wakers: RwLock::new(Vec::new()),
        })
    }

    fn get_load_state(&self) -> BundleDirReaderLoadState {
        self.load_state.read().clone()
    }

    fn set_load_state(&self, state: BundleDirReaderLoadState) {
        *self.load_state.write() = state;
    }

    fn get_dirent_list(&self, path: &PathBuf) -> Option<Vec<Dirent>> {
        self.path_dirent_list_map
            .get(path)
            .map(|dirent_list| dirent_list.clone())
    }

    async fn load(self: Arc<Self>) -> Result<(), LoadBundleMetadataError> {
        self.set_load_state(BundleDirReaderLoadState::Loading);
        let load_result = match load_bundle_metadata().await {
            Ok(bundle_metadata) => {
                let path_dirent_list_map = make_path_dirent_list_map(&bundle_metadata);
                self.path_dirent_list_map.clear();
                for (path, dirent_list) in path_dirent_list_map {
                    self.path_dirent_list_map.insert(path, dirent_list);
                }
                self.set_load_state(BundleDirReaderLoadState::Loaded);
                Ok(())
            }
            Err(error) => {
                self.set_load_state(BundleDirReaderLoadState::Unset);
                Err(error)
            }
        };
        self.wake_wakers();
        load_result
    }

    fn wake_wakers(&self) {
        let mut load_wakers = self.load_wakers.write();
        for waker in load_wakers.iter() {
            waker.clone().wake();
        }
        load_wakers.clear();
    }

    fn push_waker(&self, waker: Waker) {
        let mut load_wakers = self.load_wakers.write();
        load_wakers.push(waker);
    }
}
impl BundleDirReaderRead for Arc<BundleDirReader> {
    fn read(&self, path_like: impl PathLike) -> BundleDirReaderReadTask {
        let path = path_like.path();
        BundleDirReaderReadTask {
            path,
            bundle_dir_reader: self.clone(),
            load_future: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum BundleDirReaderLoadState {
    Unset,
    Loading,
    Loaded,
}

#[derive(Debug)]
pub enum BundleDirReaderError {
    NetworkError(String),
    ParseError(String),
    DirNotExist,
    MetadataFileNotFound(String),
    Other(String),
}
impl From<LoadBundleMetadataError> for BundleDirReaderError {
    fn from(error: LoadBundleMetadataError) -> Self {
        match error {
            LoadBundleMetadataError::NetworkError(message) => {
                BundleDirReaderError::NetworkError(message)
            }
            LoadBundleMetadataError::ParseError(message) => {
                BundleDirReaderError::ParseError(message)
            }
            LoadBundleMetadataError::FileNotFound(message) => {
                BundleDirReaderError::MetadataFileNotFound(message)
            }
            LoadBundleMetadataError::Other(message) => BundleDirReaderError::Other(message),
        }
    }
}

pub trait BundleDirReaderRead {
    fn read(&self, path_like: impl PathLike) -> BundleDirReaderReadTask;
}

pub struct BundleDirReaderReadTask {
    path: PathBuf,
    bundle_dir_reader: Arc<BundleDirReader>,
    load_future: Option<LocalBoxFuture<'static, Result<(), LoadBundleMetadataError>>>,
}
impl Future for BundleDirReaderReadTask {
    type Output = Result<Vec<Dirent>, BundleDirReaderError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let load_state = self.bundle_dir_reader.get_load_state();
        let load_needed =
            load_state == BundleDirReaderLoadState::Unset && self.load_future.is_none();
        if load_needed {
            self.load_future = Some(self.bundle_dir_reader.clone().load().boxed_local());
        }
        match load_state {
            BundleDirReaderLoadState::Unset | BundleDirReaderLoadState::Loading => {
                match self.load_future.as_mut() {
                    Some(load_future) => {
                        let load_future_poll = load_future.as_mut().poll(cx);
                        match load_future_poll {
                            Poll::Ready(load_result) => match load_result {
                                Ok(_) => self.return_dirent_list_poll(),
                                Err(error) => Poll::Ready(Err(error.into())),
                            },
                            Poll::Pending => Poll::Pending,
                        }
                    }
                    None => {
                        self.bundle_dir_reader.push_waker(cx.waker().clone());
                        Poll::Pending
                    }
                }
            }
            BundleDirReaderLoadState::Loaded => self.return_dirent_list_poll(),
        }
    }
}
impl BundleDirReaderReadTask {
    fn return_dirent_list_poll(
        self: Pin<&mut Self>,
    ) -> Poll<Result<Vec<Dirent>, BundleDirReaderError>> {
        match self.bundle_dir_reader.get_dirent_list(&self.path) {
            Some(dirent_list) => Poll::Ready(Ok(dirent_list)),
            None => Poll::Ready(Err(BundleDirReaderError::DirNotExist)),
        }
    }
}
