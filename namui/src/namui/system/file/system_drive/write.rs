use super::*;
use namui_cfg::namui_cfg;

#[derive(Debug)]
pub enum WriteError {
    DirNotFound(String),
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn write(path_like: impl types::PathLike, content: Vec<u8>) -> Result<(), WriteError> {
    let path = path_like.path();
    let path = path.to_str().unwrap_or("");
    electron::write_vec_u8(path, content)
        .await
        .map_err(|error| error.into())
}

impl From<electron::WriteVecU8Error> for WriteError {
    fn from(val: electron::WriteVecU8Error) -> Self {
        match val {
            electron::WriteVecU8Error::Other(message) => WriteError::Other(message),
            electron::WriteVecU8Error::DirNotFound(message) => WriteError::DirNotFound(message),
        }
    }
}
