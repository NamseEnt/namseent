use super::BundleDirReaderReadError;
use crate::file::{
    bundle::BundleDirReader,
    types::{Dirent, PathLike},
};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    static ref BUNDLE_DIR_READER: Arc<BundleDirReader> = BundleDirReader::new();
}

pub fn read_dir(path: impl PathLike) -> Result<Vec<Dirent>, BundleDirReaderReadError> {
    BUNDLE_DIR_READER.read(&path.path())
}

pub(crate) async fn init() -> Result<(), super::BundleDirReaderInitError> {
    BUNDLE_DIR_READER.init().await
}
