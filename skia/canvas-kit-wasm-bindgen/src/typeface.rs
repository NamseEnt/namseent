use super::*;

unsafe impl Sync for CanvasKitTypeface {}
unsafe impl Send for CanvasKitTypeface {}

#[wasm_bindgen]
extern "C" {
    pub type CanvasKitTypeface;
}
