use super::{read_dir, BundleDirReaderInitError};
use crate::fs::FileSystemInitError;

pub async fn init() -> Result<(), FileSystemInitError> {
    Ok(read_dir::read_dir::init().await?)
}

impl From<BundleDirReaderInitError> for FileSystemInitError {
    fn from(error: BundleDirReaderInitError) -> Self {
        FileSystemInitError::BundleDirReaderInitError(error)
    }
}
