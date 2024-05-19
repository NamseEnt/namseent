use super::*;
use crate::*;
use std::sync::Arc;
use wasm_bindgen::JsValue;
use web_sys::ImageBitmap;

pub struct CkImage {
    pub(crate) canvas_kit_image: CanvasKitImage,
    image_info: ImageInfo,
    src: ImageSource,
}

unsafe impl Send for CkImage {}
unsafe impl Sync for CkImage {}

static IMAGE_MAP: SerdeMap<ImageSource, CkImage> = SerdeMap::new();

impl CkImage {
    pub(crate) fn load(image_source: &ImageSource, image_bitmap: ImageBitmap) {
        let canvas_kit_image = make_lazy_image_from_texture_source(&image_bitmap, None, None);

        let image_info = get_image_info(&canvas_kit_image);

        let ck_image = CkImage {
            canvas_kit_image,
            image_info,
            src: image_source.clone(),
        };

        IMAGE_MAP.insert(image_source, ck_image);
    }

    pub(crate) fn get(image_source: &ImageSource) -> Option<Arc<CkImage>> {
        IMAGE_MAP.get(image_source)
    }

    pub(crate) fn image(&self) -> Image {
        Image {
            wh: self.size(),
            src: self.src.clone(),
        }
    }

    pub fn size(&self) -> Wh<Px> {
        let canvas_kit_image_info = self.info();
        Wh {
            width: canvas_kit_image_info.width,
            height: canvas_kit_image_info.height,
        }
    }
    pub(crate) fn get_default_shader(&self) -> Shader {
        Shader::Image {
            src: self.src.clone(),
        }
    }

    pub(crate) fn canvas_kit(&self) -> &CanvasKitImage {
        &self.canvas_kit_image
    }
}

impl SkImage for CkImage {
    fn info(&self) -> ImageInfo {
        self.image_info
    }
}

impl Drop for CkImage {
    fn drop(&mut self) {
        self.canvas_kit_image.delete();
    }
}

fn get_image_info(canvas_kit_image: &CanvasKitImage) -> ImageInfo {
    let canvas_kit_image_info = canvas_kit_image.getImageInfo();
    ImageInfo {
        width: canvas_kit_image_info.width().px(),
        height: canvas_kit_image_info.height().px(),
        alpha_type: match canvas_kit_image_info.alphaType().value() {
            value if alpha_type_opaque().eq(&value) => AlphaType::Opaque,
            value if alpha_type_premul().eq(&value) => AlphaType::Premul,
            value if alpha_type_unpremul().eq(&value) => AlphaType::Unpremul,
            value => panic!("Unknown alpha type: {}", value),
        },
        color_type: match canvas_kit_image_info.colorType().value() {
            value if color_type_alpha_8().eq(&value) => ColorType::Gray8,
            value if color_type_rgb_565().eq(&value) => ColorType::Rgb565,
            value if color_type_rgba_8888().eq(&value) => ColorType::Rgba8888,
            value if color_type_bgra_8888().eq(&value) => ColorType::Bgra8888,
            value if color_type_rgba_1010102().eq(&value) => ColorType::Rgba1010102,
            value if color_type_rgb_101010x().eq(&value) => ColorType::Rgb101010x,
            value if color_type_gray_8().eq(&value) => ColorType::Gray8,
            value if color_type_rgba_f16().eq(&value) => ColorType::RgbaF16,
            value if color_type_rgba_f32().eq(&value) => ColorType::RgbaF32,
            value => panic!("Unknown color type: {}", value),
        },
    }
}

fn make_lazy_image_from_texture_source(
    src: &JsValue, // NOTE: It can also be an HTMLVideoElement or an HTMLCanvasElement.
    info: Option<ImageInfo>,
    src_is_premul: Option<bool>,
) -> CanvasKitImage {
    let info = info.map(|info| info.to_js_object());
    // image.makeCopyWithDefaultMipmaps() // Do we need this?
    canvas_kit().MakeLazyImageFromTextureSource(src, info, src_is_premul)
}
