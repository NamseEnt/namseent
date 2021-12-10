use super::*;
use js_sys::Float32Array;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    pub type CanvasKit;

    #[wasm_bindgen(js_namespace = globalThis, js_name = getCanvasKit)]
    pub fn canvas_kit() -> CanvasKit;

    /// Surface related functions
    ///
    /// Creates a Surface on a given canvas. If both GPU and CPU modes have been compiled in, this
    /// will first try to create a GPU surface and then fallback to a CPU one if that fails. If just
    /// the CPU mode has been compiled in, a CPU surface will be created.
    /// @param canvas - either the canvas element itself or a string with the DOM id of it.
    ///
    #[wasm_bindgen(structural, method)]
    pub fn MakeCanvasSurface(
        this: &CanvasKit,
        canvas: &HtmlCanvasElement,
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

    // ///
    // /// Return an Image backed by the encoded data, but attempt to defer decoding until the image
    // /// is actually used/drawn. This deferral allows the system to cache the result, either on the
    // /// CPU or on the GPU, depending on where the image is drawn.
    // /// This decoding uses the codecs that have been compiled into CanvasKit. If the bytes are
    // /// invalid (or an unrecognized codec), null will be returned. See Image.h for more details.
    // /// @param bytes
    // ///
    // #[wasm_bindgen(js_namespace = CanvasKit)]
    // fn MakeImageFromEncoded(bytes: &[u8]) -> Option<Image>;

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
    pub fn RRectXY(this: &CanvasKit, rect: Float32Array, rx: f32, ry: f32) -> Float32Array;

    #[wasm_bindgen(method, getter)]
    pub fn FontMgr(this: &CanvasKit) -> FontMgrFactory;

    #[wasm_bindgen(method, getter)]
    pub fn TextBlob(this: &CanvasKit) -> TextBlobFactory;

    #[wasm_bindgen(method, getter)]
    pub fn PaintStyle(this: &CanvasKit) -> CanvasKitPaintStyleEnumValues;
}
