use namui_cfg::namui_cfg;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub enum WriteVecU8Error {
    NoSuchFileOrDirector(String),
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn write_vec_u8(path: &str, content: Vec<u8>) -> Result<(), WriteVecU8Error> {
    use wasm_bindgen::JsCast;
    write_vec_u8_to_electron(path, content)
        .await
        .map_err(|error| {
            let error: js_sys::Error = error.dyn_into().unwrap();
            error.into()
        })
}

#[wasm_bindgen]
extern "C" {
    #[namui_cfg(target_env = "electron")]
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi", "fileSystem"], js_name = write)]
    async fn write_vec_u8_to_electron(path: &str, content: Vec<u8>) -> Result<(), JsValue>;
}

impl Into<WriteVecU8Error> for js_sys::Error {
    fn into(self) -> WriteVecU8Error {
        let message = self.message();
        if message.starts_with("ENOENT", 0) {
            WriteVecU8Error::NoSuchFileOrDirector(format!("{}", message))
        } else {
            WriteVecU8Error::Other(format!("{}", message))
        }
    }
}
