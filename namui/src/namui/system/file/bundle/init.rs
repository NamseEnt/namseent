use super::*;
use crate::file::init::FileSystemInitError;

pub async fn init() -> Result<(), FileSystemInitError> {
    Ok(read_dir::read_dir::init().await?)
}

impl From<BundleDirReaderInitError> for FileSystemInitError {
    fn from(error: BundleDirReaderInitError) -> Self {
        FileSystemInitError::BundleDirReaderInitError(error)
    }
}
