use crate::file::types::Dirent;
use crate::file::types::PathLike;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::Result;

fn to_local_storage_path(path_like: impl PathLike) -> Result<PathBuf> {
    if cfg!(not(target_family = "wasm")) {
        Ok(std::env::temp_dir()
            .join(std::env::current_exe()?.file_name().unwrap())
            .join("local_storage")
            .join(path_like.path()))
    } else {
        todo!()
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

pub async fn read_dir(path_like: impl PathLike) -> Result<Vec<Dirent>> {
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
