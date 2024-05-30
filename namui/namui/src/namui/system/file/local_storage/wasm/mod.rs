use crate::file::types::Dirent;
use crate::file::types::PathLike;
use crate::tokio::io::Result;

pub async fn delete(path_like: impl PathLike) -> Result<()> {
    todo!()
}

pub async fn make_dir(path_like: impl PathLike) -> Result<()> {
    todo!()
}

pub async fn read_dir(_path_like: impl PathLike) -> Result<Vec<Dirent>> {
    todo!()
}

pub async fn read(path_like: impl PathLike) -> Result<Vec<u8>> {
    todo!()
}

pub async fn write(path_like: impl PathLike, content: impl AsRef<[u8]>) -> Result<()> {
    todo!()
}
