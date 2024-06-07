use crate::file::types::Dirent;
use crate::file::types::PathLike;
use anyhow::anyhow;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::Result;

fn to_local_storage_path(path_like: impl PathLike) -> Result<PathBuf> {
    local_storage_root().map(|root| root.join(path_like.path()))
}

fn local_storage_root() -> std::io::Result<PathBuf> {
    if cfg!(target_os = "wasi") {
        // wasi doesn't support temp_dir https://github.com/WebAssembly/WASI/issues/306
        Ok(PathBuf::from("/local_storage"))
    } else {
        Ok(std::env::temp_dir()
            .join(
                std::env::current_exe()?
                    .file_name()
                    .ok_or(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        anyhow!("Failed to get current executable file name"),
                    ))?,
            )
            .join("local_storage"))
    }
}

pub async fn delete(path_like: impl PathLike) -> Result<()> {
    let path = to_local_storage_path(path_like)?;
    fs::remove_file(path).await
}

pub async fn make_dir(path_like: impl PathLike) -> Result<()> {
    let path = to_local_storage_path(path_like)?;
    fs::create_dir_all(path).await
}

pub async fn read_dir(_path_like: impl PathLike) -> Result<Vec<Dirent>> {
    todo!()
}

pub async fn read(path_like: impl PathLike) -> Result<Vec<u8>> {
    let path = to_local_storage_path(path_like)?;
    fs::read(path).await
}

pub async fn write(path_like: impl PathLike, content: impl AsRef<[u8]>) -> Result<()> {
    let path = to_local_storage_path(path_like)?;
    fs::write(path, content).await
}
