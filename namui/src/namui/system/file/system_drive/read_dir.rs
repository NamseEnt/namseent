use super::*;
use namui_cfg::namui_cfg;

#[derive(Debug)]
pub enum ReadDirError {
    DirNotFound(String),
    ParseError(serde_json::Error),
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn read_dir(
    path_like: impl crate::fs::types::PathLike,
) -> Result<Vec<crate::fs::types::Dirent>, ReadDirError> {
    let path = path_like.path();
    let path = path.to_str().unwrap_or("");
    Ok(electron::read_dir(path).await?)
}

impl From<electron::ReadDirError> for ReadDirError {
    fn from(error: electron::ReadDirError) -> Self {
        match error {
            electron::ReadDirError::DirNotFound(message) => Self::DirNotFound(message),
            electron::ReadDirError::ParseError(error) => Self::ParseError(error),
            electron::ReadDirError::Other(message) => Self::Other(message),
        }
    }
}
