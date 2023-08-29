use wasm_bindgen::JsCast;
use web_sys::Window;

pub fn window() -> Window {
    web_sys::window().unwrap()
}

pub fn document() -> web_sys::Document {
    window().document().unwrap()
}

pub fn canvas_element() -> web_sys::HtmlCanvasElement {
    let canvas_element = document().get_element_by_id("canvas").unwrap();
    canvas_element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap()
}
