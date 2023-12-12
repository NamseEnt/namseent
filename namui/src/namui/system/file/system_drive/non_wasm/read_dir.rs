use crate::file::types;

#[derive(Debug)]
pub enum ReadDirError {
    DirNotFound(String),
    ParseError(serde_json::Error),
    Other(String),
}

pub async fn read_dir(path_like: impl types::PathLike) -> Result<Vec<types::Dirent>, ReadDirError> {
    let path = path_like.path();
    tokio::fs::read_dir(path)?
}
