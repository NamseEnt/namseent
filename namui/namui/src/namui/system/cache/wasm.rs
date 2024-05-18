use crate::system::InitResult;
use crate::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn cacheGet(key: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn cacheSet(key: &str, value: JsValue) -> Result<(), JsValue>;
}

pub(crate) async fn init() -> InitResult {
    Ok(())
}

pub async fn get(key: &str) -> Result<Option<Box<[u8]>>> {
    match cacheGet(key).await {
        Ok(value) => {
            if value.is_undefined() {
                Ok(None)
            } else {
                Ok(Some(
                    value
                        .dyn_into::<js_sys::Uint8Array>()
                        .unwrap()
                        .to_vec()
                        .into_boxed_slice(),
                ))
            }
        }
        Err(error) => Err(anyhow!("{:?}", error)),
    }
}

pub async fn get_serde<T: serde::de::DeserializeOwned>(key: &str) -> Result<Option<T>> {
    match cacheGet(key).await {
        Ok(value) => {
            if value.is_undefined() {
                Ok(None)
            } else {
                Ok(Some(serde_json::from_slice(
                    &value.dyn_into::<js_sys::Uint8Array>().unwrap().to_vec(),
                )?))
            }
        }
        Err(error) => Err(anyhow!("{:?}", error)),
    }
}

pub async fn set(key: &str, value: &[u8]) -> Result<()> {
    let data = js_sys::Uint8Array::from(value);
    cacheSet(key, data.into())
        .await
        .map_err(|error| anyhow!("{:?}", error))
}

pub async fn set_serde<T: serde::Serialize>(key: &str, value: &T) -> Result<()> {
    let data = serde_json::to_vec(value)?;
    cacheSet(key, js_sys::Uint8Array::from(data.as_slice()).into())
        .await
        .map_err(|error| anyhow!("{:?}", error))
}

pub async fn delete(key: &str) -> Result<()> {
    cacheSet(key, JsValue::UNDEFINED)
        .await
        .map_err(|error| anyhow!("{:?}", error))
}
