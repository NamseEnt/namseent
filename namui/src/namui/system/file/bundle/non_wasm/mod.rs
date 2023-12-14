use crate::change_path_to_platform::change_path_to_platform;
use crate::file::types::Dirent;
use crate::file::types::PathLike;
use crate::system::InitResult;
use std::io::ErrorKind;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{self, Error};

pub async fn init() -> InitResult {
    Ok(())
}

fn to_bundle_path(path_like: impl PathLike) -> io::Result<PathBuf> {
    let path = change_path_to_platform(
        std::path::Path::new(std::env::current_exe()?.parent().unwrap()).join("bundle"),
        path_like,
    )
    .map_err(|error| Error::new(ErrorKind::Other, error))?;

    crate::log!("to_bundle_path: {}", path.display());
    Ok(path)
}

pub async fn read(path_like: impl PathLike) -> io::Result<impl AsRef<[u8]>> {
    fs::read(to_bundle_path(path_like)?).await
}

pub async fn read_json<T: serde::de::DeserializeOwned>(path_like: impl PathLike) -> io::Result<T> {
    let bytes = read(path_like).await?;
    serde_json::from_slice(bytes.as_ref()).map_err(|error| Error::new(ErrorKind::Other, error))
}

pub fn read_dir(_path: impl PathLike) -> io::Result<Vec<Dirent>> {
    todo!()
}
