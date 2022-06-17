use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "alert")]
    fn _alert(message: &str);
}

#[wasm_bindgen]
pub fn alert(message: &str) {
    _alert(message);
}
