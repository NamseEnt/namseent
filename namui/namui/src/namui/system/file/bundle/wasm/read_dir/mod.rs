mod bundle_dir_reader;
mod load_bundle_metadata;
mod make_path_dirent_list_map;

use crate::file::types::{Dirent, PathLike};
pub(crate) use bundle_dir_reader::*;
use lazy_static::lazy_static;
pub(crate) use load_bundle_metadata::*;
pub(crate) use make_path_dirent_list_map::*;
use std::sync::Arc;

lazy_static! {
    static ref BUNDLE_DIR_READER: Arc<BundleDirReader> = BundleDirReader::new();
}

pub fn read_dir(path: impl PathLike) -> Result<Vec<Dirent>, BundleDirReaderReadError> {
    BUNDLE_DIR_READER.read(&path.path())
}

pub(crate) async fn init() -> Result<(), BundleDirReaderInitError> {
    BUNDLE_DIR_READER.init().await
}
