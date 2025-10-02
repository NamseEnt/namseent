use crate::{file::types::PathLike, *};
use tokio::io::{self, Error};

pub async fn read(path_like: impl PathLike) -> io::Result<Vec<u8>> {
    println!("bundle read, {}", path_like.path().display());
    let response = http::Request::get(format!("/{}", path_like.path().display()))
        .body(())
        .map_err(io::Error::other)?
        .send()
        .await
        .map_err(io::Error::other)?;

    if response.status().is_success() {
        let body = response.bytes().await.map_err(io::Error::other)?;
        Ok(body)
    } else {
        Err(io::Error::other(response.status().to_string()))
    }
}

pub async fn read_json<T: serde::de::DeserializeOwned>(path_like: impl PathLike) -> io::Result<T> {
    let bytes = read(path_like).await?;
    serde_json::from_slice(bytes.as_ref()).map_err(Error::other)
}
