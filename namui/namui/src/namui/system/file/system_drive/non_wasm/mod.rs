use crate::file::types;
use tokio::io::Result;

pub async fn make_dir(path_like: impl types::PathLike) -> Result<()> {
    tokio::fs::create_dir_all(path_like.path()).await?;
    Ok(())
}

pub async fn read_dir(_path_like: impl types::PathLike) -> Result<Vec<types::Dirent>> {
    todo!()
    // let path = path_like.path();
    // tokio::fs::read_dir(path).await?;
}

pub async fn read(path_like: impl types::PathLike) -> Result<Vec<u8>> {
    let path = path_like.path();
    tokio::fs::read(path).await
}

pub async fn write(path_like: impl types::PathLike, content: Vec<u8>) -> Result<()> {
    let path = path_like.path();
    tokio::fs::create_dir_all(path.parent().unwrap()).await?;
    tokio::fs::write(path, content).await?;
    Ok(())
}
