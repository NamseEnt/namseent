use super::bundle::{self, BundleDirReaderInitError};
use strum::Display;

#[derive(Display)]
pub enum FileSystemInitError {
    BundleDirReaderInitError(BundleDirReaderInitError),
}

pub async fn init() -> Result<(), FileSystemInitError> {
    bundle::init().await
}
