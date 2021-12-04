use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type FontMgr;
    pub type CanvasKitTypeFace;

    ///
    /// Create a typeface for the specified bytes and return it.
    /// @param fontData
    ///
    #[wasm_bindgen(method)]
    pub fn MakeTypefaceFromData(this: &FontMgr, fontData: js_sys::ArrayBuffer)
        -> CanvasKitTypeFace;
}
