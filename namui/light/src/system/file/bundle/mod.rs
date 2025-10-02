use crate::file::types::PathLike;
use tokio::io::{self, Error};

pub async fn read(path_like: impl PathLike) -> io::Result<Vec<u8>> {
    todo!()
}

pub async fn read_json<T: serde::de::DeserializeOwned>(path_like: impl PathLike) -> io::Result<T> {
    let bytes = read(path_like).await?;
    serde_json::from_slice(bytes.as_ref()).map_err(Error::other)
}
