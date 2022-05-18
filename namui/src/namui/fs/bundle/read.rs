use crate::fs::{electron, types::PathLike, util::create_url};
use namui_cfg::namui_cfg;

#[derive(Debug)]
pub enum ReadError {
    NetworkError(String),
    FileNotFound(String),
    Other(String),
}

#[namui_cfg(all(target_env = "electron", not(watch_reload)))]
pub async fn read(path_like: impl PathLike) -> Result<Vec<u8>, ReadError> {
    let url = create_url(path_like);
    electron::read_vec_u8(url.as_str())
        .await
        .map_err(|error| error.into())
}

#[namui_cfg(not(all(target_env = "electron", not(watch_reload))))]
pub async fn read(path_like: impl PathLike) -> Result<Vec<u8>, ReadError> {
    let url = create_url(path_like);
    crate::fetch_get_vec_u8(url.as_str())
        .await
        .map_err(|fetch_error| ReadError::NetworkError(fetch_error.to_string()))
}

impl Into<ReadError> for electron::ReadVecU8Error {
    fn into(self) -> ReadError {
        match self {
            electron::ReadVecU8Error::FileNotFound(message) => ReadError::FileNotFound(message),
            electron::ReadVecU8Error::Other(message) => ReadError::Other(message),
        }
    }
}
