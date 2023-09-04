use super::*;
use namui_cfg::namui_cfg;

#[derive(Debug)]
pub enum ReadError {
    FileNotFound(String),
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn read(path_like: impl types::PathLike) -> Result<Vec<u8>, ReadError> {
    let path = path_like.path();
    let path = path.to_str().unwrap_or("");
    electron::read_vec_u8(path)
        .await
        .map_err(|error| error.into())
}

impl From<electron::ReadVecU8Error> for ReadError {
    fn from(val: electron::ReadVecU8Error) -> Self {
        match val {
            electron::ReadVecU8Error::FileNotFound(message) => ReadError::FileNotFound(message),
            electron::ReadVecU8Error::Other(message) => ReadError::Other(message),
        }
    }
}
