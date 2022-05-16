#[allow(unused_imports)]
use crate::{
    fetch_get_vec_u8,
    fs::{types::PathLike, util::create_url},
};
#[allow(unused_imports)]
use js_sys::Uint8Array;
use namui_cfg::namui_cfg;
#[allow(unused_imports)]
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

#[derive(Debug)]
pub enum ReadError {
    NetworkError(String),
    FileNotFound(String),
    Other(String),
}

#[namui_cfg(all(target_env = "electron", not(watch_reload)))]
pub async fn read(path_like: impl PathLike) -> Result<Vec<u8>, ReadError> {
    let url = create_url(path_like);
    read_from_electron(url.as_str())
        .await
        .and_then(|file| {
            file.dyn_into()
                .and_then(|array_buffer: Uint8Array| Ok(array_buffer.to_vec()))
        })
        .map_err(|error| {
            let error: js_sys::Error = error.dyn_into().unwrap();
            error.into()
        })
}

#[namui_cfg(not(all(target_env = "electron", not(watch_reload))))]
pub async fn read(path_like: impl PathLike) -> Result<Vec<u8>, ReadError> {
    let url = create_url(path_like);
    fetch_get_vec_u8(url.as_str())
        .await
        .map_err(|fetch_error| ReadError::NetworkError(fetch_error.to_string()))
}

#[wasm_bindgen]
extern "C" {
    #[namui_cfg(all(target_env = "electron", not(watch_reload)))]
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi", "fileSystem"], js_name = read)]
    async fn read_from_electron(path: &str) -> Result<JsValue, JsValue>;
}

#[namui_cfg(all(target_env = "electron", not(watch_reload)))]
impl Into<ReadError> for js_sys::Error {
    fn into(self) -> ReadError {
        let message = self.message();
        if message.starts_with("ENOENT", 0) {
            ReadError::FileNotFound(format!("{}", message))
        } else {
            ReadError::Other(format!("{}", message))
        }
    }
}
