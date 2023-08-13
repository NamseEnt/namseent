use super::*;

unsafe impl Sync for CanvasKitTextBlob {}
unsafe impl Send for CanvasKitTextBlob {}

#[wasm_bindgen]
extern "C" {
    pub type CanvasKitTextBlob;
}
