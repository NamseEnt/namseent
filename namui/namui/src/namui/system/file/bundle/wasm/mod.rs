mod create_bundle_url;

use self::create_bundle_url::create_bundle_url;
use crate::file::types::Dirent;
use crate::file::types::PathLike;
use crate::system::InitResult;
use crate::tokio::io::{self, Error};
use std::io::ErrorKind;

pub async fn init() -> InitResult {
    Ok(())
}

pub async fn read(path_like: impl PathLike) -> io::Result<Vec<u8>> {
    let url = create_bundle_url(path_like);
    crate::system::network::http::get_bytes(url)
        .await
        .map_err(|fetch_error| {
            if let crate::system::network::http::HttpError::Status { status, message } =
                &fetch_error
            {
                if status.eq(&404) {
                    return Error::new(ErrorKind::NotFound, message.to_string());
                }
            }
            Error::new(ErrorKind::Other, fetch_error.to_string())
        })
        .map(|response| response.as_ref().to_vec())
}

pub async fn read_json<T: serde::de::DeserializeOwned>(path_like: impl PathLike) -> io::Result<T> {
    let bytes = read(path_like).await?;
    serde_json::from_slice(bytes.as_ref()).map_err(|error| Error::new(ErrorKind::Other, error))
}

pub fn read_dir(_path: impl PathLike) -> io::Result<Vec<Dirent>> {
    todo!()
}
