use super::InitResult;

#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    fn console_log(a: &str);
}

pub fn log(content: impl AsRef<str>) {
    #[cfg(target_family = "wasm")]
    console_log(content.as_ref());

    #[cfg(not(target_family = "wasm"))]
    println!("{}", content.as_ref());
}
pub(crate) async fn init() -> InitResult {
    Ok(())
}
