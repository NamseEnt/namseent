use super::bundle::{self, BundleDirReaderInitError};
use crate::system::InitResult;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug)]
pub enum FileSystemInitError {
    BundleDirReaderInitError(BundleDirReaderInitError),
}
impl Display for FileSystemInitError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            FileSystemInitError::BundleDirReaderInitError(error) => write!(f, "{:?}", error),
        }
    }
}
impl Error for FileSystemInitError {}

pub async fn init() -> InitResult {
    Ok(bundle::init().await?)
}
