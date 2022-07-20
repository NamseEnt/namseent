use super::*;

#[wasm_bindgen]
extern "C" {
    pub(crate) type RuntimeEffectFactory;

    ///
    /// Compiles a RuntimeEffect from the given shader code.
    /// @param sksl - Source code for a shader written in SkSL
    ///
    #[wasm_bindgen(method)]
    pub(crate) fn Make(this: &RuntimeEffectFactory, sksl: &str) -> Option<CanvasKitRuntimeEffect>;
}
