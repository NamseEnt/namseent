use super::*;

unsafe impl Sync for ShaderFactory {}
unsafe impl Send for ShaderFactory {}

#[wasm_bindgen]
extern "C" {
    pub type ShaderFactory;

    ///
    /// Returns a shader that combines the given shaders with a BlendMode.
    /// @param mode
    /// @param one
    /// @param two
    ///
    #[wasm_bindgen(structural, method)]
    pub fn MakeBlend(
        this: &ShaderFactory,
        mode: CanvasKitBlendMode,
        one: &CanvasKitShader,
        two: &CanvasKitShader,
    ) -> CanvasKitShader;
}
