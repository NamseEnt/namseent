use super::*;

unsafe impl Sync for MaskFilterFactory {}
unsafe impl Send for MaskFilterFactory {}

unsafe impl Sync for CanvasKitMaskFilter {}
unsafe impl Send for CanvasKitMaskFilter {}

#[wasm_bindgen]
extern "C" {
    pub type MaskFilterFactory;
    pub type CanvasKitMaskFilter;

    ///
    /// Create a blur maskfilter
    /// @param style
    /// @param sigma - Standard deviation of the Gaussian blur to apply. Must be > 0.
    /// @param respectCTM - if true the blur's sigma is modified by the CTM.
    ///
    #[wasm_bindgen(method)]
    pub fn MakeBlur(
        this: &MaskFilterFactory,
        style: &CanvasKitBlurStyle,
        sigma: f32,
        respectCTM: bool,
    ) -> CanvasKitMaskFilter;
}
