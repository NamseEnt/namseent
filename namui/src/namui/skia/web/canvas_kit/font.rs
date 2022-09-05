use super::super::super::types::*;
use super::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Font")]
    pub type CanvasKitFont;

    #[wasm_bindgen(constructor, js_class="Font", js_namespace = ["globalThis", "CanvasKit"])]
    pub(crate) fn new(face: &CanvasKitTypeface, size: i16) -> CanvasKitFont;

    ///
    /// Returns the FontMetrics for this font.
    ///
    #[wasm_bindgen(structural, method)]
    pub(crate) fn getMetrics(this: &CanvasKitFont) -> CanvasKitFontMetrics;

    #[wasm_bindgen(js_name = "FontMetrics")]
    pub type CanvasKitFontMetrics;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn ascent(this: &CanvasKitFontMetrics) -> f32;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn descent(this: &CanvasKitFontMetrics) -> f32;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn leading(this: &CanvasKitFontMetrics) -> f32;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn bounds(this: &CanvasKitFontMetrics) -> Option<Box<[f32]>>;

    ///
    /// Retrieves the bounds for each glyph in glyphs.
    /// If paint is not null, its stroking, PathEffect, and MaskFilter fields are respected.
    /// These are returned as flattened rectangles.  For each glyph, there will be 4 floats for
    /// left, top, right, bottom (relative to 0, 0) for that glyph.
    /// @param glyphs
    /// @param paint
    /// @param output - if provided, the results will be copied into this array.
    ///
    #[wasm_bindgen(structural, method)]
    pub(crate) fn getGlyphBounds(
        this: &CanvasKitFont,
        glyphs: GlyphIds,
        paint: Option<&CanvasKitPaint>,
    ) -> js_sys::Float32Array;

    ///
    /// Retrieves the glyph ids for each code point in the provided string. This call is passed to
    /// the typeface of this font. Note that glyph IDs are typeface-dependent; different faces
    /// may have different ids for the same code point.
    /// @param str
    /// @param numCodePoints - the number of code points in the string. Defaults to str.length.
    /// @param output - if provided, the results will be copied into this array.
    ///
    #[wasm_bindgen(structural, method)]
    pub(crate) fn getGlyphIDs(this: &CanvasKitFont, str: &str) -> GlyphIds;

    ///
    /// Retrieves the advanceX measurements for each glyph.
    /// If paint is not null, its stroking, PathEffect, and MaskFilter fields are respected.
    /// One width per glyph is returned in the returned array.
    /// @param glyphs
    /// @param paint
    /// @param output - if provided, the results will be copied into this array.
    ///
    #[wasm_bindgen(structural, method)]
    pub(crate) fn getGlyphWidths(
        this: &CanvasKitFont,
        glyphs: GlyphIds,
        paint: Option<&CanvasKitPaint>,
    ) -> js_sys::Float32Array;

    // ///
    // /// Computes any intersections of a thick "line" and a run of positionsed glyphs.
    // /// The thick line is represented as a top and bottom coordinate (positive for
    // /// below the baseline, negative for above). If there are no intersections
    // /// (e.g. if this is intended as an underline, and there are no "collisions")
    // /// then the returned array will be empty. If there are intersections, the array
    // /// will contain pairs of X coordinates [start, end] for each segment that
    // /// intersected with a glyph.
    //  *
    // /// @param glyphs        the glyphs to intersect with
    // /// @param positions     x,y coordinates (2 per glyph) for each glyph
    // /// @param top           top of the thick "line" to use for intersection testing
    // /// @param bottom        bottom of the thick "line" to use for intersection testing
    // /// @return              array of [start, end] x-coordinate pairs. Maybe be empty.
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn getGlyphIntercepts(this: &CanvasKitFont, glyphs: InputGlyphIDArray, positions: Float32Array | number[],
    //                    top: number, bottom: number)-> js_sys::Float32Array;

    // ///
    // /// Returns text scale on x-axis. Default value is 1.
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn getScaleX(this: &CanvasKitFont, )-> number;

    // ///
    // /// Returns text size in points.
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn getSize(this: &CanvasKitFont, )-> number;

    // ///
    // /// Returns text skew on x-axis. Default value is zero.
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn getSkewX(this: &CanvasKitFont, )-> number;

    // ///
    // /// Returns embolden effect for this font. Default value is false.
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn isEmbolden(this: &CanvasKitFont, )-> boolean;

    // ///
    // /// Returns the Typeface set for this font.
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn getTypeface(this: &CanvasKitFont, )-> Typeface | null;

    // ///
    // /// Requests, but does not require, that edge pixels draw opaque or with partial transparency.
    // /// @param edging
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setEdging(this: &CanvasKitFont, edging: FontEdging)-> void;

    // ///
    // /// Requests, but does not require, to use bitmaps in fonts instead of outlines.
    // /// @param embeddedBitmaps
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setEmbeddedBitmaps(this: &CanvasKitFont, embeddedBitmaps: boolean)-> void;

    // ///
    // /// Sets level of glyph outline adjustment.
    // /// @param hinting
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setHinting(this: &CanvasKitFont, hinting: FontHinting)-> void;

    // ///
    // /// Requests, but does not require, linearly scalable font and glyph metrics.
    //  *
    // /// For outline fonts 'true' means font and glyph metrics should ignore hinting and rounding.
    // /// Note that some bitmap formats may not be able to scale linearly and will ignore this flag.
    // /// @param linearMetrics
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setLinearMetrics(this: &CanvasKitFont, linearMetrics: boolean)-> void;

    // ///
    // /// Sets the text scale on the x-axis.
    // /// @param sx
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setScaleX(this: &CanvasKitFont, sx: number)-> void;

    // ///
    // /// Sets the text size in points on this font.
    // /// @param points
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setSize(this: &CanvasKitFont, points: number)-> void;

    // ///
    // /// Sets the text-skew on the x axis for this font.
    // /// @param sx
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setSkewX(this: &CanvasKitFont, sx: number)-> void;

    // ///
    // /// Set embolden effect for this font.
    // /// @param embolden
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setEmbolden(this: &CanvasKitFont, embolden: boolean)-> void;

    // ///
    // /// Requests, but does not require, that glyphs respect sub-pixel positioning.
    // /// @param subpixel
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setSubpixel(this: &CanvasKitFont, subpixel: boolean)-> void;

    // ///
    // /// Sets the typeface to use with this font. null means to clear the typeface and use the
    // /// default one.
    // /// @param face
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn setTypeface(this: &CanvasKitFont, face: Typeface | null)-> void;
}
