use crate::*;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis)]
    fn getLocationSearch() -> String;

    #[wasm_bindgen(js_namespace = globalThis)]
    fn getInitialWindowSize() -> JsValue;
}

pub fn get_location_search() -> String {
    getLocationSearch()
}

pub fn get_initial_window_size() -> Wh<Px> {
    #[derive(serde::Deserialize)]
    struct Response {
        width: f32,
        height: f32,
    }
    let response: Response = serde_wasm_bindgen::from_value(getInitialWindowSize()).unwrap();

    Wh {
        width: response.width.px(),
        height: response.height.px(),
    }
}
