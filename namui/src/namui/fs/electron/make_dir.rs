use namui_cfg::namui_cfg;
use wasm_bindgen::prelude::wasm_bindgen;
#[allow(unused_imports)]
use wasm_bindgen::JsValue;

pub enum MakeDirError {
    Other(String),
}

#[namui_cfg(target_env = "electron")]
pub async fn make_dir(path: &str) -> Result<(), MakeDirError> {
    use wasm_bindgen::JsCast;
    Ok(make_dir_to_electron(path).await.map_err(|error| {
        let error: js_sys::Error = error.dyn_into().unwrap();
        error
    })?)
}

#[wasm_bindgen]
extern "C" {
    #[namui_cfg(target_env = "electron")]
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace = ["window", "namuiApi", "fileSystem"], js_name = makeDir)]
    async fn make_dir_to_electron(path: &str) -> Result<(), JsValue>;
}

impl From<js_sys::Error> for MakeDirError {
    fn from(error: js_sys::Error) -> Self {
        let message = error.message();
        Self::Other(format!("{}", message))
    }
}
