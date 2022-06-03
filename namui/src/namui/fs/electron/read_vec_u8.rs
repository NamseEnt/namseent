use namui_cfg::namui_cfg;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

pub enum ReadVecU8Error {
    FileNotFound(String),
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn read_vec_u8(path: &str) -> Result<Vec<u8>, ReadVecU8Error> {
    Ok(read_vec_u8_from_electron(path).await.and_then(|file| {
        file.dyn_into()
            .and_then(|array_buffer: js_sys::Uint8Array| Ok(array_buffer.to_vec()))
    })?)
}

#[wasm_bindgen]
extern "C" {
    #[namui_cfg(target_env = "electron")]
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi", "fileSystem"], js_name = read)]
    async fn read_vec_u8_from_electron(
        path: &str,
    ) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>;
}

impl From<JsValue> for ReadVecU8Error {
    fn from(error: JsValue) -> Self {
        let error: js_sys::Error = error.dyn_into().unwrap();
        let message = error.message();
        if message.starts_with("ENOENT", 0) {
            Self::FileNotFound(format!("{}", message))
        } else {
            Self::Other(format!("{}", message))
        }
    }
}
