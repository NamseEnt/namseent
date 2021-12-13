use super::*;

#[wasm_bindgen]
extern "C" {
    pub type FontMgr;

    ///
    /// Create a typeface for the specified bytes and return it.
    /// @param fontData
    ///
    #[wasm_bindgen(method)]
    pub fn MakeTypefaceFromData(this: &FontMgr, fontData: js_sys::ArrayBuffer)
        -> CanvasKitTypeface;
}
