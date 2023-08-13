use super::*;

unsafe impl Sync for CanvasKitPaint {}
unsafe impl Send for CanvasKitPaint {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Paint")]
    pub type CanvasKitPaint;

    #[wasm_bindgen(constructor, js_class="Paint", js_namespace = ["globalThis", "CanvasKit"])]
    pub fn new() -> CanvasKitPaint;

    ///
    /// Returns a copy of this paint.
    ///
    #[wasm_bindgen(method)]
    pub fn copy(this: &CanvasKitPaint) -> CanvasKitPaint;

    // ///
    // /// Retrieves the alpha and RGB unpremultiplied. RGB are extended sRGB values
    // /// (sRGB gamut, and encoded with the sRGB transfer function).
    // ///
    // #[wasm_bindgen(method)]
    // pub fn getColor(this: &CanvasKitPaint) -> Color;

    ///
    /// Returns the geometry drawn at the beginning and end of strokes.
    ///
    #[wasm_bindgen(method)]
    pub fn getStrokeCap(this: &CanvasKitPaint) -> CanvasKitStrokeCap;

    ///
    /// Returns the geometry drawn at the corners of strokes.
    ///
    #[wasm_bindgen(method)]
    pub fn getStrokeJoin(this: &CanvasKitPaint) -> CanvasKitStrokeJoin;

    ///
    ///  Returns the limit at which a sharp corner is drawn beveled.
    ///
    #[wasm_bindgen(method)]
    pub fn getStrokeMiter(this: &CanvasKitPaint) -> f32;

    ///
    /// Returns the thickness of the pen used to outline the shape.
    ///
    #[wasm_bindgen(method)]
    pub fn getStrokeWidth(this: &CanvasKitPaint) -> f32;

    // ///
    // /// Replaces alpha, leaving RGBA unchanged. 0 means fully transparent, 1.0 means opaque.
    // /// @param alpha
    // ///
    // #[wasm_bindgen(method)]
    // pub fn setAlphaf(this: &CanvasKitPaint, alpha: f32);

    // ///
    // /// Requests, but does not require, that edge pixels draw opaque or with
    // /// partial transparency.
    // /// @param aa
    // ///
    #[wasm_bindgen(method)]
    pub fn setAntiAlias(this: &CanvasKitPaint, aa: bool);

    ///
    /// Sets the blend mode that is, the mode used to combine source color
    /// with destination color.
    /// @param mode
    ///
    #[wasm_bindgen(method)]
    pub fn setBlendMode(this: &CanvasKitPaint, mode: CanvasKitBlendMode);

    ///
    /// Sets alpha and RGB used when stroking and filling. The color is four floating
    /// point values, unpremultiplied. The color values are interpreted as being in
    /// the provided colorSpace.
    /// @param color
    /// @param colorSpace - defaults to sRGB
    ///
    #[wasm_bindgen(method)]
    pub fn setColor(this: &CanvasKitPaint, color: &js_sys::Float32Array);

    // ///
    // /// Sets alpha and RGB used when stroking and filling. The color is four floating
    // /// point values, unpremultiplied. The color values are interpreted as being in
    // /// the provided colorSpace.
    // /// @param r
    // /// @param g
    // /// @param b
    // /// @param a
    // /// @param colorSpace - defaults to sRGB
    // ///
    // #[wasm_bindgen(method)]
    // pub fn setColorComponents(this: &CanvasKitPaint, r: number, g: number, b: number, a: number, colorSpace:Option<ColorSpace);

    ///
    /// Sets the current color filter, replacing the existing one if there was one.
    /// @param filter
    ///
    #[wasm_bindgen(method)]
    pub fn setColorFilter(this: &CanvasKitPaint, filter: &CanvasKitColorFilter);

    // ///
    // /// Sets the color used when stroking and filling. The color values are interpreted as being in
    // /// the provided colorSpace.
    // /// @param color
    // /// @param colorSpace - defaults to sRGB.
    // ///
    // #[wasm_bindgen(method)]
    // pub fn setColorInt(this: &CanvasKitPaint, color: ColorInt, colorSpace:Option<ColorSpace);

    // ///
    // /// Sets the current image filter, replacing the existing one if there was one.
    // /// @param filter
    // ///
    // #[wasm_bindgen(method)]
    // pub fn setImageFilter(this: &CanvasKitPaint, filter: Option<ImageFilter>);

    // ///
    // /// Sets the current mask filter, replacing the existing one if there was one.
    // /// @param filter
    // ///
    // #[wasm_bindgen(method)]
    // pub fn setMaskFilter(this: &CanvasKitPaint, filter: Option<MaskFilter>);

    // ///
    // /// Sets the current path effect, replacing the existing one if there was one.
    // /// @param effect
    // ///
    // #[wasm_bindgen(method)]
    // pub fn setPathEffect(this: &CanvasKitPaint, effect: Option<PathEffect>);

    ///
    /// Sets the current shader, replacing the existing one if there was one.
    /// @param shader
    ///
    #[wasm_bindgen(method)]
    pub fn setShader(this: &CanvasKitPaint, shader: Option<&CanvasKitShader>);

    ///
    /// Sets the geometry drawn at the beginning and end of strokes.
    /// @param cap
    ///
    #[wasm_bindgen(method)]
    pub fn setStrokeCap(this: &CanvasKitPaint, cap: CanvasKitStrokeCap);

    ///
    /// Sets the geometry drawn at the corners of strokes.
    /// @param join
    ///
    #[wasm_bindgen(method)]
    pub fn setStrokeJoin(this: &CanvasKitPaint, join: CanvasKitStrokeJoin);

    // ///
    // /// Sets the limit at which a sharp corner is drawn beveled.
    // /// @param limit
    // ///
    // #[wasm_bindgen(method)]
    // pub fn setStrokeMiter(this: &CanvasKitPaint, limit: number);

    ///
    /// Sets the thickness of the pen used to outline the shape.
    /// @param width
    ///
    #[wasm_bindgen(method)]
    pub fn setStrokeWidth(this: &CanvasKitPaint, width: f32);

    // ///
    // /// Sets whether the geometry is filled or stroked.
    // /// @param style
    // ///
    #[wasm_bindgen(method)]
    pub fn setStyle(this: &CanvasKitPaint, style: CanvasKitPaintStyle);
}
