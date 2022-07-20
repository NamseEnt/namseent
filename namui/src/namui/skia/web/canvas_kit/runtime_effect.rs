use super::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "RuntimeEffect")]
    pub(crate) type CanvasKitRuntimeEffect;

    // ///
    // /// Returns a shader executed using the given uniform data.
    // /// @param uniforms
    // /// @param isOpaque
    // /// @param localMatrix
    // ///
    // #[wasm_bindgen(method)]
    // pub(crate) fn makeShader(
    //     this: &CanvasKitRuntimeEffect,
    //     uniforms: &[f32],
    //     // isOpaque?: boolean,
    //     // localMatrix?: InputMatrix
    // ) -> CanvasKitShader;

    /// Returns a shader executed using the given uniform data and the children as inputs.
    /// @param uniforms
    /// @param isOpaque
    /// @param children: should be CanvasKitShader
    /// @param localMatrix
    ///
    #[wasm_bindgen(method)]
    pub(crate) fn makeShaderWithChildren(
        this: &CanvasKitRuntimeEffect,
        uniforms: &[f32],
        isOpaque: Option<bool>,
        children: Option<Vec<JsValue>>,
        // localMatrix: Option<InputMatrix>
    ) -> CanvasKitShader;
}

unsafe impl Send for CanvasKitRuntimeEffect {}
unsafe impl Sync for CanvasKitRuntimeEffect {}
