use crate::change_path_to_platform::change_path_to_platform;
use crate::file::types::Dirent;
use crate::file::types::PathLike;
use crate::system::InitResult;
use anyhow::anyhow;
use std::io::ErrorKind;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::{self, Error};

pub async fn init() -> InitResult {
    Ok(())
}

pub fn to_real_path(path_like: impl PathLike) -> io::Result<PathBuf> {
    let path = change_path_to_platform(bundle_root()?, path_like);
    Ok(path)
}

fn bundle_root() -> io::Result<PathBuf> {
    if cfg!(target_os = "wasi") {
        // wasi doesn't support temp_dir https://github.com/WebAssembly/WASI/issues/306
        Ok(PathBuf::from("/bundle"))
    } else {
        Ok(std::env::temp_dir()
            .join(std::env::current_exe()?.file_name().ok_or(io::Error::new(
                io::ErrorKind::Other,
                anyhow!("Failed to get current executable file name"),
            ))?)
            .join("bundle"))
    }
}

pub async fn read(path_like: impl PathLike) -> io::Result<Vec<u8>> {
    let path = to_real_path(path_like)?;
    eprintln!("got path");
    let result = std::fs::read(&path);
    eprintln!("std test: {:?}", result.map(|x| x.len()));
    fs::read(&path).await
}

pub async fn read_json<T: serde::de::DeserializeOwned>(path_like: impl PathLike) -> io::Result<T> {
    let bytes = read(path_like).await?;
    serde_json::from_slice(bytes.as_ref()).map_err(|error| Error::new(ErrorKind::Other, error))
}

pub fn read_dir(_path: impl PathLike) -> io::Result<Vec<Dirent>> {
    todo!()
}
