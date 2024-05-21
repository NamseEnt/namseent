use super::*;
use crate::*;
use std::sync::atomic::AtomicU32;

pub(crate) struct CkImage {
    canvas_kit_image: CanvasKitImage,
    image_info: ImageInfo,
}

static IMAGE_MAP: StaticHashMap<u32, CkImage> = StaticHashMap::new();

impl CkImage {
    pub(crate) fn load_image_from_web_image_bitmap(
        image_bitmap: web_sys::ImageBitmap,
    ) -> ImageLoaded {
        static IMAGE_ID: AtomicU32 = AtomicU32::new(0);

        let image_id = IMAGE_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let canvas_kit_image =
            canvas_kit().MakeLazyImageFromTextureSource(image_bitmap.into(), None, None);
        let image_info = get_image_info(&canvas_kit_image);

        IMAGE_MAP.insert(
            image_id,
            CkImage {
                canvas_kit_image,
                image_info,
            },
        );

        ImageLoaded {
            id: image_id,
            image_info,
        }
    }
    pub(crate) fn unload_image(id: u32) {
        IMAGE_MAP.remove(&id);
    }
    pub(crate) fn canvas_kit(&self) -> &CanvasKitImage {
        &self.canvas_kit_image
    }
    pub(crate) fn into_shader(self: Arc<Self>) -> Shader {
        Shader::Image {
            src: Image {
                info: self.image_info,
                ck_image: self,
            },
        }
    }
    pub(crate) fn get(id: u32) -> Image {
        let image = IMAGE_MAP.get(&id).unwrap();
        Image {
            info: image.image_info,
            ck_image: image.clone(),
        }
    }
}

impl Drop for CkImage {
    fn drop(&mut self) {
        self.canvas_kit_image.delete()
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
