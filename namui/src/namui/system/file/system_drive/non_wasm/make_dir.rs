use crate::file::types;

#[derive(Debug)]
pub enum MakeDirError {
    Other(String),
}

pub async fn make_dir(path_like: impl types::PathLike) -> Result<(), MakeDirError> {
    tokio::fs::create_dir_all(path_like.path()).await?;
}
