use super::*;
use js_sys::Float32Array;

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
        mode: &CanvasKitBlendMode,
        one: &CanvasKitShader,
        two: &CanvasKitShader,
    ) -> CanvasKitShader;

    // /**
    //  * Returns a shader with a given color and colorspace.
    //  * @param color
    //  * @param space
    //  */
    // #[wasm_bindgen(structural, method)]
    //     pub fn MakeColor(color: InputColor, space: ColorSpace) -> Shader;

    // /**
    //  * Returns a shader with Perlin Fractal Noise.
    //  * See SkPerlinNoiseShader.h for more details
    //  * @param baseFreqX - base frequency in the X direction; range [0.0, 1.0]
    //  * @param baseFreqY - base frequency in the Y direction; range [0.0, 1.0]
    //  * @param octaves
    //  * @param seed
    //  * @param tileW - if this and tileH are non-zero, the frequencies will be modified so that the
    //  *                noise will be tileable for the given size.
    //  * @param tileH - if this and tileW are non-zero, the frequencies will be modified so that the
    //  *                noise will be tileable for the given size.
    //  */
    // #[wasm_bindgen(structural, method)]
    //     pub fn MakeFractalNoise(baseFreqX: f32, baseFreqY: f32, octaves: f32, seed: f32,
    //                  tileW: f32, tileH: f32) -> Shader;

    /**
     * Returns a shader that generates a linear gradient between the two specified points.
     * See SkGradientShader.h for more.
     * @param start
     * @param end
     * @param colors - colors to be distributed between start and end.
     * @param pos - May be null. The relative positions of colors. If supplied must be same length
     *              as colors.
     * @param mode
     * @param localMatrix
     * @param flags - By default gradients will interpolate their colors in unpremul space
     *                and then premultiply each of the results. By setting this to 1, the
     *                gradients will premultiply their colors first, and then interpolate
     *                between them.
     * @param colorSpace
     */
    #[wasm_bindgen(structural, method)]
    pub fn MakeLinearGradient(
        this: &ShaderFactory,
        start: &[f32], // [f32; 2]
        end: &[f32],   // [f32; 2]
        colors: Vec<Float32Array>,
        pos: Option<&[f32]>,
        mode: &CanvasKitTileMode,
        localMatrix: Option<&[f32]>, // MallocObj | Matrix4x4 | Matrix3x3 | Matrix3x2 | DOMMatrix | number[];
        flags: Option<f32>,
        colorSpace: Option<&CanvasKitColorSpace>,
    ) -> CanvasKitShader;

    // /**
    //  * Returns a shader that generates a radial gradient given the center and radius.
    //  * See SkGradientShader.h for more.
    //  * @param center
    //  * @param radius
    //  * @param colors - colors to be distributed between the center and edge.
    //  * @param pos - May be null. The relative positions of colors. If supplied must be same length
    //  *              as colors. Range [0.0, 1.0]
    //  * @param mode
    //  * @param localMatrix
    //  * @param flags - 0 to interpolate colors in unpremul, 1 to interpolate colors in premul.
    //  * @param colorSpace
    //  */
    // #[wasm_bindgen(structural, method)]
    //     pub fn MakeRadialGradient(center: &[f32],// [f32; 2]
    //  radius: f32, colors: InputFlexibleColorArray,
    //                    pos: f32[] | null, mode: TileMode, localMatrix?: InputMatrix,
    //                    flags?: f32, colorSpace?: ColorSpace) -> Shader;

    // /**
    //  * Returns a shader that generates a sweep gradient given a center.
    //  * See SkGradientShader.h for more.
    //  * @param cx
    //  * @param cy
    //  * @param colors - colors to be distributed around the center, within the provided angles.
    //  * @param pos - May be null. The relative positions of colors. If supplied must be same length
    //  *              as colors. Range [0.0, 1.0]
    //  * @param mode
    //  * @param localMatrix
    //  * @param flags - 0 to interpolate colors in unpremul, 1 to interpolate colors in premul.
    //  * @param startAngle - angle corresponding to 0.0. Defaults to 0 degrees.
    //  * @param endAngle - angle corresponding to 1.0. Defaults to 360 degrees.
    //  * @param colorSpace
    //  */
    // #[wasm_bindgen(structural, method)]
    //     pub fn MakeSweepGradient(cx: f32, cy: f32, colors: InputFlexibleColorArray,
    //                   pos: f32[] | null, mode: TileMode, localMatrix?: InputMatrix | null,
    //                   flags?: f32, startAngle?: AngleInDegrees, endAngle?: AngleInDegrees,
    //                   colorSpace?: ColorSpace) -> Shader;

    // /**
    //  * Returns a shader with Perlin Turbulence.
    //  * See SkPerlinNoiseShader.h for more details
    //  * @param baseFreqX - base frequency in the X direction; range [0.0, 1.0]
    //  * @param baseFreqY - base frequency in the Y direction; range [0.0, 1.0]
    //  * @param octaves
    //  * @param seed
    //  * @param tileW - if this and tileH are non-zero, the frequencies will be modified so that the
    //  *                noise will be tileable for the given size.
    //  * @param tileH - if this and tileW are non-zero, the frequencies will be modified so that the
    //  *                noise will be tileable for the given size.
    //  */
    // #[wasm_bindgen(structural, method)]
    //     pub fn MakeTurbulence(baseFreqX: f32, baseFreqY: f32, octaves: f32, seed: f32,
    //                tileW: f32, tileH: f32) -> Shader;

    // /**
    //  * Returns a shader that generates a conical gradient given two circles.
    //  * See SkGradientShader.h for more.
    //  * @param start
    //  * @param startRadius
    //  * @param end
    //  * @param endRadius
    //  * @param colors
    //  * @param pos
    //  * @param mode
    //  * @param localMatrix
    //  * @param flags
    //  * @param colorSpace
    //  */
    // #[wasm_bindgen(structural, method)]
    //     pub fn MakeTwoPointConicalGradient(start: &[f32],// [f32; 2]
    //  startRadius: f32,
    // end: &[f32],// [f32; 2]

    //                             endRadius: f32, colors: InputFlexibleColorArray,
    //                             pos: f32[] | null, mode: TileMode, localMatrix?: InputMatrix,
    //                             flags?: f32, colorSpace?: ColorSpace) -> Shader;
}
