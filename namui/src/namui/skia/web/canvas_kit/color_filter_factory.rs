use super::*;

#[wasm_bindgen]
extern "C" {
    pub type ColorFilterFactory;
    pub type CanvasKitColorFilter;

    ///
    /// Makes a color filter with the given color and blend mode.
    /// @param color
    /// @param mode
    ///
    #[wasm_bindgen(method)]
    pub(crate) fn MakeBlend(
        this: &ColorFilterFactory,
        color: &js_sys::Float32Array,
        mode: &CanvasKitBlendMode,
    ) -> CanvasKitColorFilter;

    // ///
    // /// Makes a color filter composing two color filters.
    // /// @param outer
    // /// @param inner
    // ///
    // #[wasm_bindgen(method)]
    // pub(crate) fn MakeCompose(
    //     this: &ColorFilterFactory,
    //     outer: ColorFilter,
    //     inner: ColorFilter,
    // ) -> CanvasKitColorFilter;

    // ///
    // /// Makes a color filter that is linearly interpolated between two other color filters.
    // /// @param t - a float in the range of 0.0 to 1.0.
    // /// @param dst
    // /// @param src
    // ///
    // #[wasm_bindgen(method)]
    // pub(crate) fn MakeLerp(
    //     this: &ColorFilterFactory,
    //     t: number,
    //     dst: ColorFilter,
    //     src: ColorFilter,
    // ) -> CanvasKitColorFilter;

    // ///
    // /// Makes a color filter that converts between linear colors and sRGB colors.
    // ///
    // #[wasm_bindgen(method)]
    // pub(crate) fn MakeLinearToSRGBGamma(this: &ColorFilterFactory) -> CanvasKitColorFilter;

    // ///
    // /// Creates a color filter using the provided color matrix.
    // /// @param cMatrix
    // ///
    // #[wasm_bindgen(method)]
    // pub(crate) fn MakeMatrix(this: &ColorFilterFactory, cMatrix: InputColorMatrix)
    //     -> CanvasKitColorFilter;

    // ///
    // /// Makes a color filter that converts between sRGB colors and linear colors.
    // ///
    // #[wasm_bindgen(method)]
    // pub(crate) fn MakeSRGBToLinearGamma(this: &ColorFilterFactory) -> CanvasKitColorFilter;
}
