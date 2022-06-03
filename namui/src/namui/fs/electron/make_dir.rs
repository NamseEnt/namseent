use namui_cfg::namui_cfg;
use wasm_bindgen::{prelude::wasm_bindgen, JsCast, JsValue};

pub enum MakeDirError {
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn make_dir(path: &str) -> Result<(), MakeDirError> {
    Ok(make_dir_to_electron(path).await?)
}

#[wasm_bindgen]
extern "C" {
    #[namui_cfg(target_env = "electron")]
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi", "fileSystem"], js_name = makeDir)]
    async fn make_dir_to_electron(path: &str) -> Result<(), JsValue>;
}

impl From<JsValue> for MakeDirError {
    fn from(error: JsValue) -> Self {
        let error: js_sys::Error = error.dyn_into().unwrap();
        let message = error.message();
        Self::Other(format!("{}", message))
    }
}
