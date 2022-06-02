use namui_cfg::namui_cfg;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

pub enum WriteVecU8Error {
    NoSuchFileOrDirector(String),
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn write_vec_u8(path: &str, content: Vec<u8>) -> Result<(), WriteVecU8Error> {
    Ok(write_vec_u8_to_electron(path, content).await?)
}

#[wasm_bindgen]
extern "C" {
    #[namui_cfg(target_env = "electron")]
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi", "fileSystem"], js_name = write)]
    async fn write_vec_u8_to_electron(path: &str, content: Vec<u8>) -> Result<(), JsValue>;
}

impl From<JsValue> for WriteVecU8Error {
    fn from(error: JsValue) -> Self {
        let error: js_sys::Error = error.dyn_into().unwrap();
        let message = error.message();
        if message.starts_with("ENOENT", 0) {
            Self::NoSuchFileOrDirector(format!("{}", message))
        } else {
            Self::Other(format!("{}", message))
        }
    }
}
