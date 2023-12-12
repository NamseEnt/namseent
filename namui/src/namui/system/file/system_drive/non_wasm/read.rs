use crate::file::types;

#[derive(Debug)]
pub enum ReadError {
    FileNotFound(String),
    Other(String),
}

pub async fn read(path_like: impl types::PathLike) -> Result<Vec<u8>, ReadError> {
    let path = path_like.path();
    tokio::fs::read(path).await?
}
