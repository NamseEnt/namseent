use super::InitResult;
use anyhow::Result;
use serde_bytes::{ByteBuf, Bytes};

pub(crate) async fn init() -> InitResult {
    Ok(())
}

async fn get_cache_internal<T: serde::de::DeserializeOwned>(key: &str) -> T {
    crate::system::web::execute_async_function(
        "
    return await cacheGet(key);
    ",
    )
    .arg("key", key)
    .run()
    .await
}

async fn set_cache_internal(key: &str, value: impl serde::Serialize) {
    crate::system::web::execute_async_function(
        "
    return await cacheSet(key, value);
    ",
    )
    .arg("key", key)
    .arg("value", value)
    .run::<Option<()>>()
    .await;
}

pub async fn get(key: &str) -> Result<Option<Box<[u8]>>> {
    let value: Option<serde_bytes::ByteBuf> = get_cache_internal(key).await;
    Ok(value.map(|v| v.into_vec().into_boxed_slice()))
}

pub async fn get_serde<T: serde::de::DeserializeOwned>(key: &str) -> Result<Option<T>> {
    let value: Option<T> = get_cache_internal(key).await;
    Ok(value)
}

pub async fn set(key: &str, value: &[u8]) -> Result<()> {
    set_cache_internal(key, Bytes::new(value)).await;

    Ok(())
}

pub async fn set_serde<T: serde::Serialize>(key: &str, value: &T) -> Result<()> {
    set_cache_internal(key, value).await;

    Ok(())
}

pub async fn delete(key: &str) -> Result<()> {
    set_cache_internal(key, Option::<()>::None).await;

    Ok(())
}
