use super::*;

unsafe impl Sync for CanvasKitRuntimeEffect {}
unsafe impl Send for CanvasKitRuntimeEffect {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "RuntimeEffect")]
    pub type CanvasKitRuntimeEffect;

    // ///
    // /// Returns a shader executed using the given uniform data.
    // /// @param uniforms
    // /// @param isOpaque
    // /// @param localMatrix
    // ///
    // #[wasm_bindgen(method)]
    // pub fn makeShader(
    //     this: &CanvasKitRuntimeEffect,
    //     uniforms: &[f32],
    //     // isOpaque?: boolean,
    //     // localMatrix?: InputMatrix
    // ) -> CanvasKitShader;

    /// Returns a shader executed using the given uniform data and the children as inputs.
    /// @param uniforms
    /// @param children: should be CanvasKitShader
    /// @param localMatrix
    #[wasm_bindgen(method)]
    pub fn makeShaderWithChildren(
        this: &CanvasKitRuntimeEffect,
        uniforms: &[f32],
        children: Option<Vec<JsValue>>,
        // localMatrix: Option<InputMatrix>,
    ) -> CanvasKitShader;
}
