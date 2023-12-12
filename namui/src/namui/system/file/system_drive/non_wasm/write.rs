use crate::file::types;

#[derive(Debug)]
pub enum WriteError {
    DirNotFound(String),
    Other(String),
}

pub async fn write(path_like: impl types::PathLike, content: Vec<u8>) -> Result<(), WriteError> {
    let path = path_like.path();
    tokio::fs::write(path, content).await?;
}
