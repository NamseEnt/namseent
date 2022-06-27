use web_sys::{window, HtmlAnchorElement, HtmlCanvasElement, HtmlImageElement};

pub fn create_anchor_element() -> HtmlAnchorElement {
    let document = window()
        .expect("failed to get window")
        .document()
        .expect("failed to get window.document");
    let element = document
        .create_element("a")
        .expect("failed to create HTMLAnchorElement");
    let anchor_element = wasm_bindgen::JsCast::dyn_into::<HtmlAnchorElement>(element)
        .expect("failed to cast HTMLAnchorElement");
    anchor_element
}

pub fn create_canvas_element() -> HtmlCanvasElement {
    let document = window()
        .expect("failed to get window")
        .document()
        .expect("failed to get window.document");
    let element = document
        .create_element("canvas")
        .expect("failed to create HTMLCanvasElement");
    let canvas_element = wasm_bindgen::JsCast::dyn_into::<HtmlCanvasElement>(element)
        .expect("failed to cast HTMLCanvasElement");
    canvas_element
}

pub fn create_image_element() -> HtmlImageElement {
    let document = window()
        .expect("failed to get window")
        .document()
        .expect("failed to get window.document");
    let element = document
        .create_element("img")
        .expect("failed to create HTMLImageElement");
    let image_element = wasm_bindgen::JsCast::dyn_into::<HtmlImageElement>(element)
        .expect("failed to cast HTMLImageElement");
    image_element
}
