use super::file_system_directory_handle::FileSystemDirectoryHandle;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub async fn get_root_directory() -> Result<FileSystemDirectoryHandle, JsValue> {
    get_directory().await.map(|js_value| js_value.into())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(catch)]
    #[wasm_bindgen(js_namespace=["window", "navigator", "storage"], js_name=getDirectory)]
    async fn get_directory() -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsValue>;
}
