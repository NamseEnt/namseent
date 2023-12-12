use crate::{system::InitResult, *};
use std::path::PathBuf;

pub(crate) async fn init() -> InitResult {
    Ok(())
}

fn get_cache_path(key: &str) -> Result<PathBuf> {
    let path = std::env::temp_dir()
        .join(
            std::env::current_exe()?
                .file_name()
                .ok_or(anyhow!("Failed to get current executable file name"))?,
        )
        .join("cache")
        .join(key);
    Ok(path)
}

pub async fn get(key: &str) -> Result<Option<Box<[u8]>>> {
    let path = get_cache_path(key)?;

    match tokio::fs::read(path).await {
        Ok(buffer) => Ok(Some(buffer.into_boxed_slice())),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                return Ok(None);
            }
            _ => {
                return Err(err);
            }
        },
    }
}

pub async fn get_serde<T: serde::de::DeserializeOwned>(key: &str) -> Result<Option<T>> {
    get(key).await.map(|value| {
        value.map(|value| {
            serde_json::from_slice(&value).map_err(|err| {
                anyhow!(
                    "Failed to deserialize value of key `{}`: {:?}",
                    key,
                    err.to_string()
                )
            })
        })
    })?
}

pub async fn set(key: &str, value: &[u8]) -> Result<()> {
    let path = get_cache_path(key)?;
    tokio::fs::write(path, value).await?;
}

pub async fn set_serde<T: serde::Serialize>(key: &str, value: &T) -> Result<()> {
    let data = serde_json::to_vec(value)?;
    set(key, data.as_slice()).await
}

pub async fn delete(key: &str) -> Result<()> {
    let path = get_cache_path(key)?;
    tokio::fs::remove_file(path).await?;
}
