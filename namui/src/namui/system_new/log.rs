use super::InitResult;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    fn console_log(a: &str);
}

pub fn log(content: impl AsRef<str>) {
    console_log(content.as_ref());
}
pub(crate) async fn init() -> InitResult {
    Ok(())
}
