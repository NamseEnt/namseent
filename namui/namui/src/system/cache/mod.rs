use crate::{
    change_path_to_platform::change_path_to_platform, file::types::PathLike, system::InitResult, *,
};
use std::path::PathBuf;
use tokio::io;

pub(crate) async fn init() -> InitResult {
    Ok(())
}

fn get_cache_path(path_like: impl PathLike) -> io::Result<PathBuf> {
    let path = change_path_to_platform(cache_root()?, path_like);

    Ok(path)
}

fn cache_root() -> io::Result<PathBuf> {
    if cfg!(target_os = "wasi") {
        // wasi doesn't support temp_dir https://github.com/WebAssembly/WASI/issues/306
        Ok(PathBuf::from("/cache"))
    } else {
        Ok(std::env::temp_dir()
            .join(
                std::env::current_exe()?
                    .file_name()
                    .ok_or(io::Error::other(anyhow!(
                        "Failed to get current executable file name"
                    )))?,
            )
            .join("cache"))
    }
}

pub async fn get(key: &str) -> io::Result<Option<Box<[u8]>>> {
    let path = get_cache_path(key)?;

    match tokio::fs::read(path).await {
        Ok(buffer) => Ok(Some(buffer.into_boxed_slice())),
        Err(err) => {
            match err.kind() {
                std::io::ErrorKind::NotFound => Ok(None),
                // #[cfg(target_os = "wasi")]
                _ => Ok(None),
                // #[cfg(not(target_os = "wasi"))]
                // _ => Err(err),
            }
        }
    }
}

pub async fn get_serde<T: serde::de::DeserializeOwned>(key: &str) -> io::Result<Option<T>> {
    let Some(value) = get(key).await? else {
        return Ok(None);
    };
    serde_json::from_slice(&value).map_err(|err| {
        io::Error::other(anyhow!(
            "Failed to deserialize value of key `{}`: {:?}",
            key,
            err.to_string()
        ))
    })
}

pub async fn set(key: &str, value: &[u8]) -> io::Result<()> {
    let path = get_cache_path(key)?;
    tokio::fs::create_dir_all(path.parent().unwrap()).await?;
    tokio::fs::write(path, value).await?;
    Ok(())
}

pub async fn set_serde<T: serde::Serialize>(key: &str, value: &T) -> io::Result<()> {
    let data = serde_json::to_vec(value)?;
    set(key, data.as_slice()).await
}

pub async fn delete(key: &str) -> io::Result<()> {
    let path = get_cache_path(key)?;
    tokio::fs::remove_file(path).await?;
    Ok(())
}
