#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_family = "wasm")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = "log")]
    fn console_log(a: &str);
}

#[cfg(target_family = "wasm")]
#[allow(dead_code)]
pub(crate) fn log(content: impl AsRef<str>) {
    console_log(content.as_ref());
}

#[cfg(not(target_family = "wasm"))]
#[allow(dead_code)]
pub(crate) fn log(content: impl AsRef<str>) {
    println!("{}", content.as_ref());
}
