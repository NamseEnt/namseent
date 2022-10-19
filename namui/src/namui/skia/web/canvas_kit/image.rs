use super::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "Image")]
    pub type CanvasKitImage;

    // ///
    // /// Encodes this image's pixels to the specified format and returns them. Must be built with
    // /// the specified codec. If the options are unspecified, sensible defaults will be
    // /// chosen.
    // /// @param fmt - PNG is the default value.
    // /// @param quality - a value from 0 to 100; 100 is the least lossy. May be ignored.
    // #[wasm_bindgen(method)]
    // pub(crate) fn encodeToBytes(this: &CanvasKitImage, fmt?: EncodedImageFormat, quality?: number) -> Uint8Array | null;

    // ///
    // /// Returns the color space associated with this object.
    // /// It is the user's responsibility to call delete() on this after it has been used.
    // #[wasm_bindgen(method)]
    // pub(crate) fn getColorSpace(this: &CanvasKitImage, ) -> ColorSpace;

    ///
    /// Returns the width, height, colorType and alphaType associated with this image.
    /// Colorspace is separate so as to not accidentally leak that memory.
    #[wasm_bindgen(method)]
    pub(crate) fn getImageInfo(this: &CanvasKitImage) -> CanvasKitPartialImageInfo;

    #[wasm_bindgen(js_name = "PartialImageInfo")]
    pub type CanvasKitPartialImageInfo;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn alphaType(this: &CanvasKitPartialImageInfo) -> CanvasKitAlphaType;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn colorType(this: &CanvasKitPartialImageInfo) -> CanvasKitColorType;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn height(this: &CanvasKitPartialImageInfo) -> f32;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn width(this: &CanvasKitPartialImageInfo) -> f32;

    // /// Return the height in pixels of the image.
    // #[wasm_bindgen(method)]
    // pub(crate) fn height(this: &CanvasKitImage, ) -> number;

    ///
    /// Returns an Image with the same "base" pixels as the this image, but with mipmap levels
    /// automatically generated and attached.
    #[wasm_bindgen(method)]
    pub(crate) fn makeCopyWithDefaultMipmaps(this: &CanvasKitImage) -> CanvasKitImage;

    // ///
    // /// Returns this image as a shader with the specified tiling. It will use cubic sampling.
    // /// @param tx - tile mode in the x direction.
    // /// @param ty - tile mode in the y direction.
    // /// @param B - See CubicResampler in SkSamplingOptions.h for more information
    // /// @param C - See CubicResampler in SkSamplingOptions.h for more information
    // /// @param localMatrix
    // #[wasm_bindgen(method)]
    // pub(crate) fn makeShaderCubic(this: &CanvasKitImage, tx: TileMode, ty: TileMode, B: number, C: number,
    //                 localMatrix?: InputMatrix) -> Shader;

    ///
    /// Returns this image as a shader with the specified tiling. It will use cubic sampling.
    /// @param tx - tile mode in the x direction.
    /// @param ty - tile mode in the y direction.
    /// @param fm - The filter mode.
    /// @param mm - The mipmap mode. Note: for settings other than None, the image must have mipmaps
    ///             calculated with makeCopyWithDefaultMipmaps;
    /// @param localMatrix
    #[wasm_bindgen(method)]
    pub(crate) fn makeShaderOptions(
        this: &CanvasKitImage,
        tx: CanvasKitTileMode,
        ty: CanvasKitTileMode,
        fm: CanvasKitFilterMode,
        mm: CanvasKitMipmapMode,
        // localMatrix?: InputMatrix
    ) -> CanvasKitShader;

    // ///
    // /// Returns a TypedArray containing the pixels reading starting at (srcX, srcY) and does not
    // /// exceed the size indicated by imageInfo. See SkImage.h for more on the caveats.
    //  *
    // /// If dest is not provided, we allocate memory equal to the provided height/// the provided
    // /// bytesPerRow to fill the data with.
    //  *
    // /// @param srcX
    // /// @param srcY
    // /// @param imageInfo - describes the destination format of the pixels.
    // /// @param dest - If provided, the pixels will be copied into the allocated buffer allowing
    // ///        access to the pixels without allocating a new TypedArray.
    // /// @param bytesPerRow - number of bytes per row. Must be provided if dest is set. This
    // ///        depends on destination ColorType. For example, it must be at least 4/// width for
    // ///        the 8888 color type.
    // /// @returns a TypedArray appropriate for the specified ColorType. Note that 16 bit floats are
    // ///          not supported in JS, so that colorType corresponds to raw bytes Uint8Array.
    // #[wasm_bindgen(method)]
    // pub(crate) fn readPixels(this: &CanvasKitImage, srcX: number, srcY: number, imageInfo: ImageInfo, dest?: MallocObj,
    //             bytesPerRow?: number) -> Uint8Array | Float32Array | null;

    // ///
    // /// Return the width in pixels of the image.
    // #[wasm_bindgen(method)]
    // pub(crate) fn width(this: &CanvasKitImage, ) -> number;
}
