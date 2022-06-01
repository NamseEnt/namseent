use crate::fs::{
    electron,
    types::{Dirent, PathLike},
};
use namui_cfg::namui_cfg;

#[derive(Debug)]
pub enum ReadDirError {
    FileNotFound(String),
    ParseError(serde_json::Error),
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn read_dir(path_like: impl PathLike) -> Result<Vec<Dirent>, ReadDirError> {
    let path = path_like.path();
    let path = path.to_str().unwrap_or("");
    Ok(electron::read_dir(path).await?)
}

impl From<electron::ReadDirError> for ReadDirError {
    fn from(error: electron::ReadDirError) -> Self {
        match error {
            electron::ReadDirError::DirNotFound(message) => Self::FileNotFound(message),
            electron::ReadDirError::ParseError(error) => Self::ParseError(error),
            electron::ReadDirError::Other(message) => Self::Other(message),
        }
    }
}
