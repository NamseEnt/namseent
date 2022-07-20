use super::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) type CanvasKitShader;
}

unsafe impl Send for CanvasKitShader {}
unsafe impl Sync for CanvasKitShader {}
