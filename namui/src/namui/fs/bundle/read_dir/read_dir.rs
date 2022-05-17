use super::{BundleDirReaderError, BundleDirReaderRead};
use crate::fs::{
    bundle::BundleDirReader,
    types::{Dirent, PathLike},
};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref BUNDLE_DIR_READER: Arc<BundleDirReader> = BundleDirReader::new();
}

#[derive(Debug)]
pub enum ReadDirError {
    NetworkError(String),
    ParseError(String),
    DirNotExist,
    MetadataFileNotFound(String),
    Other(String),
}

pub async fn read_dir(path_like: impl PathLike) -> Result<Vec<Dirent>, ReadDirError> {
    let dirent_list = BUNDLE_DIR_READER.read(path_like).await?;
    Ok(dirent_list)
}

impl From<BundleDirReaderError> for ReadDirError {
    fn from(error: BundleDirReaderError) -> Self {
        match error {
            BundleDirReaderError::NetworkError(message) => ReadDirError::NetworkError(message),
            BundleDirReaderError::ParseError(message) => ReadDirError::ParseError(message),
            BundleDirReaderError::DirNotExist => ReadDirError::DirNotExist,
            BundleDirReaderError::MetadataFileNotFound(message) => {
                ReadDirError::MetadataFileNotFound(message)
            }
            BundleDirReaderError::Other(message) => ReadDirError::Other(message),
        }
    }
}
