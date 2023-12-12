use crate::file::types::Dirent;
use crate::file::types::PathLike;
use crate::system::InitResult;
use std::io::ErrorKind;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::Error;
use tokio::io::Result;

pub async fn init() -> InitResult {
    Ok(())
}

fn to_bundle_path(path_like: impl PathLike) -> Result<PathBuf> {
    if cfg!(not(target_family = "wasm")) {
        Ok(std::env::temp_dir()
            .join(std::env::current_exe()?.file_name().unwrap())
            .join("bundle")
            .join(path_like.path()))
    } else {
        todo!()
    }
}

pub async fn read(path_like: impl PathLike) -> Result<impl AsRef<[u8]>> {
    fs::read(to_bundle_path(path_like)?).await
}

pub async fn read_json<T: serde::de::DeserializeOwned>(path_like: impl PathLike) -> Result<T> {
    let bytes = read(path_like).await?;
    serde_json::from_slice(bytes.as_ref()).map_err(|error| Error::new(ErrorKind::Other, error))
}

pub fn read_dir(path: impl PathLike) -> Result<Vec<Dirent>> {
    todo!()
}
