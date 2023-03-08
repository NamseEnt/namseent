use super::*;
use crate::{namui::skia::PartialImageInfo, Image};
use js_sys::Float32Array;
use web_sys::HtmlCanvasElement;

unsafe impl Sync for CanvasKit {}
unsafe impl Send for CanvasKit {}
#[wasm_bindgen]
extern "C" {
    pub type CanvasKit;

    #[wasm_bindgen(js_namespace = globalThis, js_name = getCanvasKit)]
    pub(crate) fn canvas_kit() -> CanvasKit;

    ///  
    /// A helper for creating a WebGL backed (aka GPU) surface and falling back to a CPU surface if
    /// the GPU one cannot be created. This works for both WebGL 1 and WebGL 2.
    /// @param canvas - Either the canvas element itself or a string with the DOM id of it.
    /// @param colorSpace - One of the supported color spaces. Default is SRGB.
    /// @param opts - Options that will get passed to the creation of the WebGL context.
    ///
    #[wasm_bindgen(structural, method)]
    pub(crate) fn MakeWebGLCanvasSurface(
        this: &CanvasKit,
        canvas: &HtmlCanvasElement,
        // colorSpace?: ColorSpace,
        // opts?: WebGLOptions
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

    ///
    /// Returns a texture-backed image based on the content in src. It assumes the image is
    /// RGBA_8888, unpremul and SRGB. This image can be re-used across multiple surfaces.
    ///
    /// Not available for software-backed surfaces.
    /// @param src - CanvasKit will take ownership of the TextureSource and clean it up when
    ///              the image is destroyed.
    /// @param info - If provided, will be used to determine the width/height/format of the
    ///               source image. If not, sensible defaults will be used.
    /// @param srcIsPremul - set to true if the src data has premultiplied alpha. Otherwise, it will
    ///         be assumed to be Unpremultiplied. Note: if this is true and info specifies
    ///         Unpremul, Skia will not convert the src pixels first.
    ///
    #[wasm_bindgen(method)]
    fn MakeLazyImageFromTextureSource(
        this: &CanvasKit,
        src: JsValue, // NOTE: It can also be an HTMLVideoElement or an HTMLCanvasElement.
        info: Option<js_sys::Object>, // ImageInfo | PartialImageInfo
        srcIsPremul: Option<bool>,
    ) -> CanvasKitImage;

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

impl CanvasKit {
    pub(crate) fn make_lazy_image_from_texture_source(
        &self,
        src: JsValue, // NOTE: It can also be an HTMLVideoElement or an HTMLCanvasElement.
        info: Option<PartialImageInfo>,
        src_is_premul: Option<bool>,
    ) -> Image {
        let info = info.map(|info| info.into_js_object());
        let image = self.MakeLazyImageFromTextureSource(src, info, src_is_premul);
        let image = image.makeCopyWithDefaultMipmaps();
        Image::new(image)
    }
}
