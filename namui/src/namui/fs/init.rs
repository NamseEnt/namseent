use strum::Display;

use super::bundle::{self, BundleDirReaderInitError};

#[derive(Display)]
pub enum FileSystemInitError {
    BundleDirReaderInitError(BundleDirReaderInitError),
}

pub async fn init() -> Result<(), FileSystemInitError> {
    bundle::init().await
}
