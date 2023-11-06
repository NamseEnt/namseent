use super::*;

unsafe impl Sync for RuntimeEffectFactory {}
unsafe impl Send for RuntimeEffectFactory {}

#[wasm_bindgen]
extern "C" {
    pub type RuntimeEffectFactory;

    ///
    /// Compiles a RuntimeEffect from the given shader code.
    /// @param sksl - Source code for a shader written in SkSL
    ///
    #[wasm_bindgen(method)]
    pub fn Make(this: &RuntimeEffectFactory, sksl: &str) -> Option<CanvasKitRuntimeEffect>;
}
