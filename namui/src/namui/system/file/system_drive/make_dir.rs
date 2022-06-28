use super::*;
use namui_cfg::namui_cfg;

#[derive(Debug)]
pub enum MakeDirError {
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn make_dir(path_like: impl crate::fs::types::PathLike) -> Result<(), MakeDirError> {
    let path = path_like.path();
    let path = path.to_str().unwrap_or("");
    Ok(electron::make_dir(path).await?)
}

impl From<electron::MakeDirError> for MakeDirError {
    fn from(error: electron::MakeDirError) -> Self {
        match error {
            electron::MakeDirError::Other(message) => Self::Other(message),
        }
    }
}
