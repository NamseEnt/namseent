use super::*;
use crate::*;
use std::sync::Arc;
use web_sys::ImageBitmap;

pub struct CkImage {
    pub(crate) canvas_kit_image: CanvasKitImage,
    image_info: ImageInfo,
    src: ImageSource,
}

unsafe impl Send for CkImage {}
unsafe impl Sync for CkImage {}

static IMAGE_MAP: StaticHashMap<ImageSource, CkImage> = StaticHashMap::new();

impl CkImage {
    pub(crate) fn load(image_source: &ImageSource, image_bitmap: ImageBitmap) {
        IMAGE_MAP.insert(
            image_source.clone(),
            CkImage::new(
                image_source.clone(),
                canvas_kit().make_lazy_image_from_texture_source(&image_bitmap, None, None),
            ),
        );
    }

    pub(crate) fn load2(image_source: &ImageSource, bytes: &[u8]) {
        IMAGE_MAP.insert(
            image_source.clone(),
            CkImage::new(
                image_source.clone(),
                canvas_kit().MakeImageFromEncoded(bytes).unwrap(),
            ),
        );
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

    fn new(src: ImageSource, canvas_kit_image: CanvasKitImage) -> Self {
        let image_info = {
            let canvas_kit_image_info = canvas_kit_image.getImageInfo();
            let alpha_type = canvas_kit_image_info.alphaType().value();
            let color_type = canvas_kit_image_info.colorType().value();
            ImageInfo {
                width: px(canvas_kit_image_info.width()),
                height: px(canvas_kit_image_info.height()),
                alpha_type: match alpha_type {
                    value if ALPHA_TYPE_OPAQUE_VALUE.eq(&value) => AlphaType::Opaque,
                    value if ALPHA_TYPE_PREMUL_VALUE.eq(&value) => AlphaType::Premul,
                    value if ALPHA_TYPE_UNPREMUL_VALUE.eq(&value) => AlphaType::Unpremul,
                    value => panic!("Unknown alpha type: {}", value),
                },
                color_type: match color_type {
                    value if COLOR_TYPE_ALPHA_8_VALUE.eq(&value) => ColorType::Gray8,
                    value if COLOR_TYPE_RGB_565_VALUE.eq(&value) => ColorType::Rgb565,
                    value if COLOR_TYPE_RGBA_8888_VALUE.eq(&value) => ColorType::Rgba8888,
                    value if COLOR_TYPE_BGRA_8888_VALUE.eq(&value) => ColorType::Bgra8888,
                    value if COLOR_TYPE_RGBA_1010102_VALUE.eq(&value) => ColorType::Rgba1010102,
                    value if COLOR_TYPE_RGB_101010X_VALUE.eq(&value) => ColorType::Rgb101010x,
                    value if COLOR_TYPE_GRAY_8_VALUE.eq(&value) => ColorType::Gray8,
                    value if COLOR_TYPE_RGBA_F16_VALUE.eq(&value) => ColorType::RgbaF16,
                    value if COLOR_TYPE_RGBA_F32_VALUE.eq(&value) => ColorType::RgbaF32,
                    value => panic!("Unknown color type: {}", value),
                },
            }
        };

        CkImage {
            canvas_kit_image,
            image_info,
            src,
        }
    }
    pub fn size(&self) -> Wh<Px> {
        let canvas_kit_image_info = self.info();
        Wh {
            width: canvas_kit_image_info.width,
            height: canvas_kit_image_info.height,
        }
    }
    // pub fn make_shader(
    //     &self,
    //     tile_x: TileMode,
    //     tile_y: TileMode,
    //     filter: FilterMode,
    //     mipmap: MipmapMode,
    // ) -> Arc<CkShader> {
    //     let shader = self.canvas_kit_image.makeShaderOptions(
    //         tile_x.into(),
    //         tile_y.into(),
    //         filter.into(),
    //         mipmap.into(),
    //     );

    //     Arc::new(CkShader::new(shader))
    // }

    pub(crate) fn get_default_shader(&self, dest_rect: Rect<Px>) -> Shader {
        Shader::Image {
            src: self.src.clone(),
            dest_rect,
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
