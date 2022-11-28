use js_sys::Promise;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "navigator", "clipboard"])]
    fn writeText(text: &str) -> Promise;
}

pub async fn write_text(text: impl AsRef<str>) -> Result<(), ()> {
    let text = text.as_ref();
    let promise = writeText(text);
    let result = wasm_bindgen_futures::JsFuture::from(promise).await;
    match result {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}
