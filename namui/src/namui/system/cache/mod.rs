use super::InitResult;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

#[wasm_bindgen(raw_module = "./cache.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    async fn cacheGet(key: &str) -> Result<JsValue, JsValue>;
    #[wasm_bindgen(catch)]
    async fn cacheSet(key: &str, value: JsValue) -> Result<(), JsValue>;
}

pub(crate) async fn init() -> InitResult {
    Ok(())
}

pub async fn get(key: &str) -> Result<Option<Box<[u8]>>, Box<dyn std::error::Error>> {
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
        Err(error) => Err(format!("{:?}", error).into()),
    }
}

pub async fn get_serde<T: serde::de::DeserializeOwned>(
    key: &str,
) -> Result<Option<T>, Box<dyn std::error::Error>> {
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
        Err(error) => Err(format!("{:?}", error).into()),
    }
}

pub async fn set(key: &str, value: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let data = js_sys::Uint8Array::from(value);
    cacheSet(key, data.into())
        .await
        .map_err(|error| format!("{:?}", error).into())
}

pub async fn set_serde<T: serde::Serialize>(
    key: &str,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let data = serde_json::to_vec(value)?;
    cacheSet(key, js_sys::Uint8Array::from(data.as_slice()).into())
        .await
        .map_err(|error| format!("{:?}", error).into())
}

pub async fn delete(key: &str) -> Result<(), Box<dyn std::error::Error>> {
    cacheSet(key, JsValue::UNDEFINED)
        .await
        .map_err(|error| format!("{:?}", error).into())
}
