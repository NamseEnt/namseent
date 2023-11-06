use super::*;

unsafe impl Sync for CanvasKitShader {}
unsafe impl Send for CanvasKitShader {}

#[wasm_bindgen]
extern "C" {
    pub type CanvasKitShader;
}
