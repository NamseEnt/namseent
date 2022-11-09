use super::*;
use js_sys::Float32Array;
use web_sys::HtmlCanvasElement;

unsafe impl Sync for CanvasKit {}
unsafe impl Send for CanvasKit {}
#[wasm_bindgen]
extern "C" {
    pub type CanvasKit;
    pub type WebGPUCanvasContext;
    pub type WebGPUDeviceContext;

    #[wasm_bindgen(js_namespace = globalThis, js_name = getCanvasKit)]
    pub(crate) fn canvas_kit() -> CanvasKit;

    ///
    /// Creates a context that operates over the given WebGPU Device.
    /// @param device
    ///
    #[wasm_bindgen(structural, method)]
    pub(crate) fn MakeGPUDeviceContext(
        this: &CanvasKit,
        device: JsValue, // GpuDevice,
    ) -> Option<WebGPUDeviceContext>;

    // ///
    // /// Creates a Surface that draws to the given GPU texture.
    // /// @param ctx
    // /// @param texture - A texture that was created on the GPU device associated with `ctx`.
    // /// @param width - Width of the visible region in pixels.
    // /// @param height - Height of the visible region in pixels.
    // /// @param colorSpace
    // ///
    // #[wasm_bindgen(structural, method)]
    // pub(crate) fn MakeGPUTextureSurface(
    //     this: &CanvasKit,
    //     ctx: WebGPUDeviceContext,
    //     texture: GPUTexture,
    //     width: u32,
    //     height: u32,
    //     colorSpace: ColorSpace,
    // ) -> Option<Surface>;

    ///
    /// Creates and configures a WebGPU context for the given canvas.
    /// @param ctx
    /// @param canvas
    /// @param opts
    ///
    #[wasm_bindgen(structural, method)]
    pub(crate) fn MakeGPUCanvasContext(
        this: &CanvasKit,
        ctx: WebGPUDeviceContext,
        canvas: HtmlCanvasElement,
        // opts: Option<WebGPUCanvasOptions>,
    ) -> Option<WebGPUCanvasContext>;

    ///
    /// Creates a Surface backed by the next available texture in the swapchain associated with the
    /// given WebGPU canvas context. The context must have been already successfully configured using
    /// the same GPUDevice associated with `ctx`.
    /// @param canvasContext - WebGPU context associated with the canvas. The canvas can either be an
    ///                        on-screen HTMLCanvasElement or an OffscreenCanvas.
    /// @param colorSpace
    /// @param width - width of the visible region. If not present, the canvas width from `canvasContext`
    ///                is used.
    /// @param height - height of the visible region. If not present, the canvas width from `canvasContext`
    ///                is used.
    #[wasm_bindgen(structural, method)]
    pub(crate) fn MakeGPUCanvasSurface(
        this: &CanvasKit,
        canvasContext: WebGPUCanvasContext,
        colorSpace: CanvasKitColorSpace,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Option<CanvasKitSurface>;

    // ///
    // /// Decodes the given bytes into an animated image. Returns null if the bytes were invalid.
    // /// The passed in bytes will be copied into the WASM heap, so the caller can dispose of them.
    // /// @param bytes
    // ///
    // // #[wasm_bindgen(js_namespace = CanvasKit)]
    // // fn MakeAnimatedImageFromEncoded(bytes: &[u8]) -> Option<AnimatedImage>;

    // ///
    // /// Returns an image with the given pixel data and format.
    // /// Note that we will always make a copy of the pixel data, because of inconsistencies in
    // /// behavior between GPU and CPU (i.e. the pixel data will be turned into a GPU texture and
    // /// not modifiable after creation).
    // ///
    // /// @param info
    // /// @param bytes - bytes representing the pixel data.
    // /// @param bytesPerRow
    // ///
    // #[wasm_bindgen(js_namespace = CanvasKit)]
    // fn MakeImage(info: ImageInfo, bytes: &[u8], bytesPerRow: u32) -> Option<Image>;

    // /// NOTE: This function load image in blocking way, So it is not recommended to use it.
    // ///
    // /// Return an Image backed by the encoded data, but attempt to defer decoding until the image
    // /// is actually used/drawn. This deferral allows the system to cache the result, either on the
    // /// CPU or on the GPU, depending on where the image is drawn.
    // /// This decoding uses the codecs that have been compiled into CanvasKit. If the bytes are
    // /// invalid (or an unrecognized codec), null will be returned. See Image.h for more details.
    // /// @param bytes
    // ///
    // #[wasm_bindgen(method)]
    // pub(crate) fn MakeImageFromEncoded(this: &CanvasKit, bytes: &[u8]) -> Option<CanvasKitImage>;

    // ///
    // /// Returns an SkPicture which has been serialized previously to the given bytes.
    // /// @param bytes
    // ///
    // // #[wasm_bindgen(js_namespace = CanvasKit)]
    // // fn MakePicture(bytes: &[u8]) -> Option<SkPicture>;

    // ///
    // /// Returns a Skottie animation built from the provided json string.
    // /// Requires that Skottie be compiled into CanvasKit.
    // /// @param json
    // ///
    // // #[wasm_bindgen(js_namespace = CanvasKit)]
    // // fn MakeAnimation(json: String) -> SkottieAnimation;

    ///
    /// Returns a rectangle with rounded corners consisting of the given rectangle and
    /// the same radiusX and radiusY for all four corners.
    /// @param rect - The base rectangle.
    /// @param rx - The radius of the corners in the x direction.
    /// @param ry - The radius of the corners in the y direction.
    ///
    #[wasm_bindgen(method)]
    pub(crate) fn RRectXY(this: &CanvasKit, rect: Float32Array, rx: f32, ry: f32) -> Float32Array;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn FontMgr(this: &CanvasKit) -> FontMgrFactory;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn TextBlob(this: &CanvasKit) -> TextBlobFactory;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn ColorFilter(this: &CanvasKit) -> ColorFilterFactory;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn Matrix(this: &CanvasKit) -> Matrix3x3Helpers;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn RuntimeEffect(this: &CanvasKit) -> RuntimeEffectFactory;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn Typeface(this: &CanvasKit) -> TypefaceFactory;

    #[wasm_bindgen(method, getter)]
    pub(crate) fn Shader(this: &CanvasKit) -> ShaderFactory;
}
