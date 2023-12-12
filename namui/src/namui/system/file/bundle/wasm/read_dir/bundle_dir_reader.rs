use super::{load_bundle_metadata, make_path_dirent_list_map, LoadBundleMetadataError};
use crate::file::types::Dirent;
use dashmap::DashMap;
use std::{path::PathBuf, sync::Arc};

pub struct BundleDirReader {
    path_dirent_list_map: DashMap<PathBuf, Vec<Dirent>>,
}
impl BundleDirReader {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            path_dirent_list_map: DashMap::new(),
        })
    }

    fn get_dirent_list(&self, path: &PathBuf) -> Option<Vec<Dirent>> {
        self.path_dirent_list_map
            .get(path)
            .map(|dirent_list| dirent_list.clone())
    }

    pub(super) async fn init(&self) -> Result<(), BundleDirReaderInitError> {
        Ok(load_bundle_metadata().await.map(|bundle_metadata| {
            let path_dirent_list_map = make_path_dirent_list_map(&bundle_metadata);
            self.path_dirent_list_map.clear();
            for (path, dirent_list) in path_dirent_list_map {
                self.path_dirent_list_map.insert(path, dirent_list);
            }
        })?)
    }

    pub fn read(&self, path: &PathBuf) -> Result<Vec<Dirent>, BundleDirReaderReadError> {
        match self.get_dirent_list(path) {
            Some(dirent_list) => Ok(dirent_list),
            None => Err(BundleDirReaderReadError::DirNotExist),
        }
    }
}

#[derive(Debug)]
pub enum BundleDirReaderReadError {
    DirNotExist,
    Other(String),
}

#[derive(Debug)]
pub enum BundleDirReaderInitError {
    NetworkError(String),
    ParseError(String),
    MetadataFileNotFound(String),
    Other(String),
}
impl From<LoadBundleMetadataError> for BundleDirReaderInitError {
    fn from(error: LoadBundleMetadataError) -> Self {
        match error {
            LoadBundleMetadataError::NetworkError(message) => {
                BundleDirReaderInitError::NetworkError(message)
            }
            LoadBundleMetadataError::ParseError(message) => {
                BundleDirReaderInitError::ParseError(message)
            }
            LoadBundleMetadataError::FileNotFound(message) => {
                BundleDirReaderInitError::MetadataFileNotFound(message)
            }
            LoadBundleMetadataError::Other(message) => BundleDirReaderInitError::Other(message),
        }
    }
}
