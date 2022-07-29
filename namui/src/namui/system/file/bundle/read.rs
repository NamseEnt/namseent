use super::*;
use crate::{
    file::{electron, types::PathLike},
    simple_error_impl,
};
use namui_cfg::namui_cfg;

#[derive(Debug)]
pub enum ReadError {
    NetworkError(String),
    FileNotFound(String),
    Other(String),
}
simple_error_impl!(ReadError);

#[namui_cfg(all(target_env = "electron", not(watch_reload)))]
pub async fn read(path_like: impl PathLike) -> Result<impl AsRef<[u8]>, ReadError> {
    let url = create_bundle_url(path_like);
    electron::read_vec_u8(url.as_str())
        .await
        .map_err(|error| error.into())
}

#[namui_cfg(not(all(target_env = "electron", not(watch_reload))))]
pub async fn read(path_like: impl PathLike) -> Result<impl AsRef<[u8]>, ReadError> {
    let url = create_bundle_url(path_like);
    crate::network::http::get_bytes(url)
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
