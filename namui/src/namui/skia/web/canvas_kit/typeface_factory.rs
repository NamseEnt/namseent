use super::*;

#[wasm_bindgen]
extern "C" {
    pub type TypefaceFactory;

    ///
    /// Create a typeface using Freetype from the specified bytes and return it. CanvasKit supports
    /// .ttf, .woff and .woff2 fonts. It returns null if the bytes cannot be decoded.
    /// @param fontData
    ///
    #[wasm_bindgen(structural, method)]
    pub(crate) fn MakeFreeTypeFaceFromData(
        this: &TypefaceFactory,
        fontData: js_sys::ArrayBuffer,
    ) -> CanvasKitTypeface;
}
