use super::*;

#[wasm_bindgen]
extern "C" {
    pub type TextBlobFactory;
    // ///
    // /// Return a TextBlob with a single run of text.
    // ///
    // /// It does not perform typeface fallback for characters not found in the Typeface.
    // /// It does not perform kerning or other complex shaping; glyphs are positioned based on their
    // /// default advances.
    // /// @param glyphs - if using Malloc'd array, be sure to use CanvasKit.MallocGlyphIDs().
    // /// @param font
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub fn MakeFromGlyphs(this: &TextBlobFactory, glyphs: InputGlyphIDArray, font: Font) -> CanvasKitTextBlob;

    // ///
    // /// Returns a TextBlob built from a single run of text with rotation, scale, and translations.
    // ///
    // /// It uses the default character-to-glyph mapping from the typeface in the font.
    // /// @param str
    // /// @param rsxforms
    // /// @param font
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub fn MakeFromRSXform(this: &TextBlobFactory, str: string, rsxforms: InputFlattenedRSXFormArray, font: Font) -> CanvasKitTextBlob;

    // ///
    // /// Returns a TextBlob built from a single run of text with rotation, scale, and translations.
    // ///
    // /// @param glyphs - if using Malloc'd array, be sure to use CanvasKit.MallocGlyphIDs().
    // /// @param rsxforms
    // /// @param font
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub fn MakeFromRSXformGlyphs(this: &TextBlobFactory, glyphs: InputGlyphIDArray, rsxforms: InputFlattenedRSXFormArray,
    //                       font: Font) -> CanvasKitTextBlob;

    ///
    /// Return a TextBlob with a single run of text.
    ///
    /// It uses the default character-to-glyph mapping from the typeface in the font.
    /// It does not perform typeface fallback for characters not found in the Typeface.
    /// It does not perform kerning or other complex shaping; glyphs are positioned based on their
    /// default advances.
    /// @param str
    /// @param font
    ///
    #[wasm_bindgen(structural, method)]
    pub fn MakeFromText(
        this: &TextBlobFactory,
        str: &str,
        font: &CanvasKitFont,
    ) -> CanvasKitTextBlob;

    // ///
    // /// Returns a TextBlob that has the glyphs following the contours of the given path.
    // ///
    // /// It is a convenience wrapper around MakeFromRSXform and ContourMeasureIter.
    // /// @param str
    // /// @param path
    // /// @param font
    // /// @param initialOffset - the length in pixels to start along the path.
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub fn MakeOnPath(this: &TextBlobFactory, str: string, path: Path, font: Font, initialOffset?: number) -> CanvasKitTextBlob;
}
