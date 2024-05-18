use super::*;

unsafe impl Sync for ImageFilterFactory {}
unsafe impl Send for ImageFilterFactory {}

unsafe impl Sync for CanvasKitImageFilter {}
unsafe impl Send for CanvasKitImageFilter {}

#[wasm_bindgen]
extern "C" {
    pub type ImageFilterFactory;
    pub type CanvasKitImageFilter;

    // /**
    //  * Create a filter that takes a BlendMode and uses it to composite the two filters together.
    //  *
    //  *  At least one of background and foreground should be non-null in nearly all circumstances.
    //  *
    //  *  @param blend       The blend mode that defines the compositing operation
    //  *  @param background The Dst pixels used in blending; if null, use the dynamic source image
    //  *                    (e.g. a saved layer).
    //  *  @param foreground The Src pixels used in blending; if null, use the dynamic source image.
    //  */
    // #[wasm_bindgen(method)]
    // pub fn MakeBlend(this: &ImageFilterFactory,blend: BlendMode, background: Option<CavnasKitImageFilter>,
    //     foreground: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;

    /**
     * Create a filter that blurs its input by the separate X and Y sigmas. The provided tile mode
     * is used when the blur kernel goes outside the input image.
     *
     * @param sigmaX - The Gaussian sigma value for blurring along the X axis.
     * @param sigmaY - The Gaussian sigma value for blurring along the Y axis.
     * @param mode
     * @param input - if null, it will use the dynamic source image (e.g. a saved layer)
     */
    #[wasm_bindgen(method)]
    pub fn MakeBlur(
        this: &ImageFilterFactory,
        sigmaX: f32,
        sigmaY: f32,
        mode: &CanvasKitTileMode,
        input: Option<CanvasKitImageFilter>,
    ) -> CanvasKitImageFilter;

    // /**
    // * Create a filter that applies the color filter to the input filter results.
    // * @param cf
    // * @param input - if null, it will use the dynamic source image (e.g. a saved layer)
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeColorFilter(this: &ImageFilterFactory,cf: ColorFilter, input: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;

    // /**
    // * Create a filter that composes 'inner' with 'outer', such that the results of 'inner' are
    // * treated as the source bitmap passed to 'outer'.
    // * If either param is null, the other param will be returned.
    // * @param outer
    // * @param inner - if null, it will use the dynamic source image (e.g. a saved layer)
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeCompose(this: &ImageFilterFactory,outer: Option<CavnasKitImageFilter>, inner: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;

    // /**
    // *  Create a filter that dilates each input pixel's channel values to the max value within the
    // *  given radii along the x and y axes.
    // *  @param radiusX  The distance to dilate along the x axis to either side of each pixel.
    // *  @param radiusY  The distance to dilate along the y axis to either side of each pixel.
    // *  @param input     if null, it will use the dynamic source image (e.g. a saved layer).
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeDilate(this: &ImageFilterFactory,radiusX: f32, radiusY: f32, input: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;

    // /**
    // *  Create a filter that moves each pixel in its color input based on an (x,y) vector encoded
    // *  in its displacement input filter. Two color components of the displacement image are
    // *  mapped into a vector as scale * (color[xChannel], color[yChannel]), where the channel
    // *  selectors are one of R, G, B, or A.
    // *  The mapping takes the 0-255 RGBA values of the image and scales them to be [-0.5 to 0.5],
    // *  in a similar fashion to https://developer.mozilla.org/en-US/docs/Web/SVG/Element/feDisplacementMap
    // *
    // *  At least one of displacement and color should be non-null in nearly all circumstances.
    // *
    // *  @param xChannel RGBA channel that encodes the x displacement per pixel.
    // *  @param yChannel RGBA channel that encodes the y displacement per pixel.
    // *  @param scale    Scale applied to displacement extracted from image.
    // *  @param displacement The filter defining the displacement image, or null to use source.
    // *  @param color   The filter providing the color pixels to be displaced, or null to use source.
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeDisplacementMap(this: &ImageFilterFactory,xChannel: ColorChannel, yChannel: ColorChannel, scale: f32,
    //                 displacement: Option<CavnasKitImageFilter>, color: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;
    // /**
    // *  Create a filter that draws a drop shadow under the input content. This filter produces an
    // *  image that includes the inputs' content.
    // *  @param dx       The X offset of the shadow.
    // *  @param dy       The Y offset of the shadow.
    // *  @param sigmaX   The blur radius for the shadow, along the X axis.
    // *  @param sigmaY   The blur radius for the shadow, along the Y axis.
    // *  @param color    The color of the drop shadow.
    // *  @param input    The input filter; if null, it will use the dynamic source image.
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeDropShadow(this: &ImageFilterFactory,dx: f32, dy: f32, sigmaX: f32, sigmaY: f32, color: Color,
    //             input: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;

    // /**
    // *  Just like MakeDropShadow, except the input content is not in the resulting image.
    // *  @param dx       The X offset of the shadow.
    // *  @param dy       The Y offset of the shadow.
    // *  @param sigmaX   The blur radius for the shadow, along the X axis.
    // *  @param sigmaY   The blur radius for the shadow, along the Y axis.
    // *  @param color    The color of the drop shadow.
    // *  @param input    The input filter; if null, it will use the dynamic source image.
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeDropShadowOnly(this: &ImageFilterFactory,dx: f32, dy: f32, sigmaX: f32, sigmaY: f32, color: Color,
    //                 input: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;

    // /**
    // *  Create a filter that erodes each input pixel's channel values to the minimum channel value
    // *  within the given radii along the x and y axes.
    // *  @param radiusX  The distance to erode along the x axis to either side of each pixel.
    // *  @param radiusY  The distance to erode along the y axis to either side of each pixel.
    // *  @param input     if null, it will use the dynamic source image (e.g. a saved layer).
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeErode(this: &ImageFilterFactory,radiusX: f32, radiusY: f32, input: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;

    // /**
    // *  Create a filter using the given image as a source. Returns null if 'image' is null.
    // *
    // *  @param img      The image that is output by the filter, subset by 'srcRect'.
    // *  @param sampling The sampling to use when drawing the image.
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeImage(this: &ImageFilterFactory,img: Image, sampling: FilterOptions | CubicResampler) -> Option<CavnasKitImageFilter>;

    // /**
    // *  Create a filter that draws the 'srcRect' portion of image into 'dstRect' using the given
    // *  filter quality. Similar to Canvas.drawImageRect. Returns null if 'image' is null.
    // *
    // *  @param img      The image that is output by the filter, subset by 'srcRect'.
    // *  @param sampling The sampling to use when drawing the image.
    // *  @param srcRect  The source pixels sampled into 'dstRect'.
    // *  @param dstRect  The local rectangle to draw the image into.
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeImage(this: &ImageFilterFactory,img: Image, sampling: FilterOptions | CubicResampler,
    //         srcRect: InputRect, dstRect: InputRect) -> Option<CavnasKitImageFilter>;

    // /**
    // * Create a filter that transforms the input image by 'matrix'. This matrix transforms the
    // * local space, which means it effectively happens prior to any transformation coming from the
    // * Canvas initiating the filtering.
    // * @param matr
    // * @param sampling
    // * @param input - if null, it will use the dynamic source image (e.g. a saved layer)
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeMatrixTransform(this: &ImageFilterFactory,matr: InputMatrix, sampling: FilterOptions | CubicResampler,
    //                 input: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;

    // /**
    // *  Create a filter that offsets the input filter by the given vector.
    // *  @param dx       The x offset in local space that the image is shifted.
    // *  @param dy       The y offset in local space that the image is shifted.
    // *  @param input    The input that will be moved, if null, will use the dynamic source image.
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeOffset(this: &ImageFilterFactory,dx: f32, dy: f32, input: Option<CavnasKitImageFilter>) -> CavnasKitImageFilter;

    // /**
    // * Transforms a shader into an image filter
    // *
    // * @param shader - The Shader to be transformed
    // */
    // #[wasm_bindgen(method)]
    // pub fn MakeShader(this: &ImageFilterFactory,shader: Shader) -> CavnasKitImageFilter;
}
