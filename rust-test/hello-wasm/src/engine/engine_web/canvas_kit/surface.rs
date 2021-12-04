use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type CanvasKitSurface;
    pub type Canvas;

    ///
    /// Make sure any queued draws are sent to the screen or the GPU.
    ///
    #[wasm_bindgen(structural, method)]
    pub fn flush(this: &CanvasKitSurface);

    ///
    /// Return a canvas that is backed by this surface. Any draws to the canvas will (eventually)
    /// show up on the surface. The returned canvas is owned by the surface and does NOT need to
    /// be cleaned up by the client.
    ///
    #[wasm_bindgen(structural, method)]
    pub fn getCanvas(this: &CanvasKitSurface) -> Canvas;

    // ///
    // /// Returns the height of this surface in pixels.
    // ///
    // #[wasm_bindgen(structural, method)]
    // fn height(this: &CanvasKitSurface) -> number;

    // ///
    // /// Returns the ImageInfo associated with this surface.
    // ///
    // #[wasm_bindgen(structural, method)]
    // fn imageInfo(this: &CanvasKitSurface) -> ImageInfo;

    // ///
    // /// Creates an Image from the provided texture and info. The Image will own the texture;
    // /// when the image is deleted, the texture will be cleaned up.
    // /// @param tex
    // /// @param info - describes the content of the texture.
    // ///
    // #[wasm_bindgen(structural, method)]
    // fn makeImageFromTexture(this: &CanvasKitSurface, tex: WebGLTexture, info: ImageInfo) -> Image | null;

    // ///
    // /// Returns a texture-backed image based on the content in src. It uses RGBA_8888, unpremul
    // /// and SRGB - for more control, use makeImageFromTexture.
    //  *
    // /// Not available for software-backed surfaces.
    // /// @param src
    // /// @param width - If provided, will be used as the width of src. Otherwise, the natural
    // ///                width of src (if available) will be used.
    // /// @param height - If provided, will be used as the height of src. Otherwise, the natural
    // ///                height of src (if available) will be used.
    // ///
    // #[wasm_bindgen(structural, method)]
    // fn makeImageFromTextureSource(this: &CanvasKitSurface, src: TextureSource, width?: number, height?: number) -> Image | null;

    // ///
    // /// Returns current contents of the surface as an Image. This image will be optimized to be
    // /// drawn to another surface of the same type. For example, if this surface is backed by the
    // /// GPU, the returned Image will be backed by a GPU texture.
    // ///
    // #[wasm_bindgen(structural, method)]
    // fn makeImageSnapshot(this: &CanvasKitSurface, bounds?: InputIRect) -> Image;

    // ///
    // /// Returns a compatible Surface, haring the same raster or GPU properties of the original.
    // /// The pixels are not shared.
    // /// @param info - width, height, etc of the Surface.
    // ///
    // #[wasm_bindgen(structural, method)]
    // fn makeSurface(this: &CanvasKitSurface, info: ImageInfo) -> Surface;

    // ///
    // /// Returns if this Surface is a GPU-backed surface or not.
    // ///
    // #[wasm_bindgen(structural, method)]
    // fn reportBackendTypeIsGPU(this: &CanvasKitSurface) -> boolean;

    // ///
    // /// If this surface is GPU-backed, return the sample count of the surface.
    // ///
    // #[wasm_bindgen(structural, method)]
    // fn sampleCnt(this: &CanvasKitSurface) -> number;

    // ///
    // /// Returns the width of this surface in pixels.
    // ///
    // #[wasm_bindgen(structural, method)]
    // fn width(this: &CanvasKitSurface) -> number;
}
