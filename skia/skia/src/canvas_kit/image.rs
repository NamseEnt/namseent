use super::*;
use crate::*;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use web_sys::ImageBitmap;

pub struct CkImage {
    pub(crate) canvas_kit_image: CanvasKitImage,
    image_info: ImageInfo,
    src: ImageSource,
    // default_shader: Arc<Shader>,
    image_bitmap: ImageBitmap,
}

unsafe impl Send for CkImage {}
unsafe impl Sync for CkImage {}

static IMAGE_MAP: StaticHashMap<ImageSource, CkImage> = StaticHashMap::new();

impl CkImage {
    pub(crate) fn load(image_source: &ImageSource, image_bitmap: ImageBitmap) {
        IMAGE_MAP.insert(
            image_source.clone(),
            CkImage::new(image_source.clone(), image_bitmap),
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

    fn new(src: ImageSource, image_bitmap: ImageBitmap) -> Self {
        let canvas_kit_image =
            canvas_kit().make_lazy_image_from_texture_source(&image_bitmap, None, None);

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
        // let default_shader = Arc::new(Shader::new(canvas_kit_image.makeShaderOptions(
        //     TileMode::Clamp.into(),
        //     TileMode::Clamp.into(),
        //     FilterMode::Linear.into(),
        //     MipmapMode::Linear.into(),
        // )));

        CkImage {
            canvas_kit_image,
            image_info,
            // default_shader,
            image_bitmap,
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
    pub fn make_shader(
        &self,
        tile_x: TileMode,
        tile_y: TileMode,
        filter: FilterMode,
        mipmap: MipmapMode,
    ) -> Arc<CkShader> {
        let shader = self.canvas_kit_image.makeShaderOptions(
            tile_x.into(),
            tile_y.into(),
            filter.into(),
            mipmap.into(),
        );

        Arc::new(CkShader::new(shader))
    }

    // pub fn get_default_shader(&self) -> Arc<Shader> {
    //     self.default_shader.clone()
    // }
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
