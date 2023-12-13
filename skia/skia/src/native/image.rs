use super::*;
use crate::*;
use std::sync::Arc;

pub struct NativeImage {
    pub(crate) skia_image: skia_safe::Image,
    image_info: ImageInfo,
    src: ImageSource,
}

unsafe impl Send for NativeImage {}
unsafe impl Sync for NativeImage {}

static IMAGE_MAP: StaticHashMap<ImageSource, NativeImage> = StaticHashMap::new();

impl NativeImage {
    // TODO
    // pub(crate) fn load(image_source: ImageSource, image_bitmap: ImageBitmap) {
    //     let skia_image =
    //         canvas_kit().make_lazy_image_from_texture_source(&image_bitmap, None, None);

    //     let image_info = get_image_info(&skia_image);

    //     let ck_image = NativeImage {
    //         skia_image,
    //         image_info,
    //         src: image_source.clone(),
    //         image_bitmap,
    //     };

    //     IMAGE_MAP.insert(image_source, ck_image);
    // }

    pub(crate) fn get(image_source: &ImageSource) -> Option<Arc<NativeImage>> {
        IMAGE_MAP.get(image_source)
    }

    pub(crate) fn image(&self) -> Image {
        Image {
            wh: self.size(),
            src: self.src.clone(),
        }
    }

    pub fn size(&self) -> Wh<Px> {
        let skia_image_info = self.info();
        Wh {
            width: skia_image_info.width,
            height: skia_image_info.height,
        }
    }
    pub(crate) fn get_default_shader(&self) -> Shader {
        Shader::Image {
            src: self.src.clone(),
        }
    }

    pub(crate) fn canvas_kit(&self) -> &skia_safe::Image {
        &self.skia_image
    }

    pub(crate) async fn encode_to_png(&self) -> Vec<u8> {
        unimplemented!()
    }
}

impl SkImage for NativeImage {
    fn info(&self) -> ImageInfo {
        self.image_info
    }
}

// fn get_image_info(skia_image: &skia_safe::Image) -> ImageInfo {
//     let skia_image_info = skia_image.getImageInfo();
//     ImageInfo {
//         width: skia_image_info.width().px(),
//         height: skia_image_info.height().px(),
//         alpha_type: match skia_image_info.alphaType().value() {
//             value if ALPHA_TYPE_OPAQUE_VALUE.eq(&value) => AlphaType::Opaque,
//             value if ALPHA_TYPE_PREMUL_VALUE.eq(&value) => AlphaType::Premul,
//             value if ALPHA_TYPE_UNPREMUL_VALUE.eq(&value) => AlphaType::Unpremul,
//             value => panic!("Unknown alpha type: {}", value),
//         },
//         color_type: match skia_image_info.colorType().value() {
//             value if COLOR_TYPE_ALPHA_8_VALUE.eq(&value) => ColorType::Gray8,
//             value if COLOR_TYPE_RGB_565_VALUE.eq(&value) => ColorType::Rgb565,
//             value if COLOR_TYPE_RGBA_8888_VALUE.eq(&value) => ColorType::Rgba8888,
//             value if COLOR_TYPE_BGRA_8888_VALUE.eq(&value) => ColorType::Bgra8888,
//             value if COLOR_TYPE_RGBA_1010102_VALUE.eq(&value) => ColorType::Rgba1010102,
//             value if COLOR_TYPE_RGB_101010X_VALUE.eq(&value) => ColorType::Rgb101010x,
//             value if COLOR_TYPE_GRAY_8_VALUE.eq(&value) => ColorType::Gray8,
//             value if COLOR_TYPE_RGBA_F16_VALUE.eq(&value) => ColorType::RgbaF16,
//             value if COLOR_TYPE_RGBA_F32_VALUE.eq(&value) => ColorType::RgbaF32,
//             value => panic!("Unknown color type: {}", value),
//         },
//     }
// }
