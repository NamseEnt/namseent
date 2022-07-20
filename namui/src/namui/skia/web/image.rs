use super::*;
use crate::*;
pub use base::*;
use serde::Serialize;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

unsafe impl Sync for CanvasKitImage {}
unsafe impl Send for CanvasKitImage {}

#[derive(Serialize)]
pub struct Image {
    id: String,
    #[serde(skip)]
    pub(crate) canvas_kit_image: CanvasKitImage,
    image_info: PartialImageInfo,
}

static IMAGE_ID: AtomicUsize = AtomicUsize::new(0);

impl Image {
    pub fn new(canvas_kit_image: CanvasKitImage) -> Self {
        let id = format!("image-{}", IMAGE_ID.fetch_add(1, Ordering::Relaxed));
        let image_info = {
            let canvas_kit_image_info = canvas_kit_image.getImageInfo();
            let alpha_type = canvas_kit_image_info.alphaType().value();
            let color_type = canvas_kit_image_info.colorType().value();
            PartialImageInfo {
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
        Image {
            id,
            canvas_kit_image,
            image_info,
        }
    }
    pub fn get_image_info(&self) -> PartialImageInfo {
        self.image_info
    }
    pub fn size(&self) -> Wh<Px> {
        let canvas_kit_image_info = self.get_image_info();
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
    ) -> Arc<Shader> {
        let shader = self.canvas_kit_image.makeShaderOptions(
            tile_x.into_canvas_kit(),
            tile_y.into_canvas_kit(),
            filter.into_canvas_kit(),
            mipmap.into_canvas_kit(),
        );

        Arc::new(Shader::new(shader))
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        self.canvas_kit_image.delete();
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}
