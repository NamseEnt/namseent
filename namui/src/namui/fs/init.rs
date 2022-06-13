use super::bundle::{self, BundleDirReaderInitError};

#[derive(Debug)]
pub enum FileSystemInitError {
    BundleDirReaderInitError(BundleDirReaderInitError),
}

pub async fn init() -> Result<(), FileSystemInitError> {
    bundle::init().await
}
