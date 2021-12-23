use super::*;
use crate::namui::*;
pub use base::*;
use serde::Serialize;
use std::sync::atomic::{AtomicUsize, Ordering};

unsafe impl Sync for CanvasKitImage {}
unsafe impl Send for CanvasKitImage {}

#[derive(Serialize)]
pub struct Image {
    id: String,
    #[serde(skip)]
    pub canvas_kit_image: CanvasKitImage,
}

static IMAGE_ID: AtomicUsize = AtomicUsize::new(0);

impl Image {
    pub fn from(canvas_kit_image: CanvasKitImage) -> Self {
        let id = format!("image-{}", IMAGE_ID.fetch_add(1, Ordering::Relaxed));
        Image {
            id,
            canvas_kit_image,
        }
    }
    pub fn get_image_info(&self) -> PartialImageInfo {
        let canvas_kit_image_info = self.canvas_kit_image.getImageInfo();

        let opaque = &ALPHA_TYPE_OPAQUE_VALUE;
        let premul = &ALPHA_TYPE_PREMUL_VALUE;
        let unpremul = &ALPHA_TYPE_UNPREMUL_VALUE;

        let alpha8 = &COLOR_TYPE_ALPHA_8_VALUE;
        let rgb565 = &COLOR_TYPE_RGB_565_VALUE;
        let rgba8888 = &COLOR_TYPE_RGBA_8888_VALUE;
        let bgra8888 = &COLOR_TYPE_BGRA_8888_VALUE;
        let rgba1010102 = &COLOR_TYPE_RGBA_1010102_VALUE;
        let rgb101010x = &COLOR_TYPE_RGB_101010X_VALUE;
        let gray8 = &COLOR_TYPE_GRAY_8_VALUE;
        let rgbaf16 = &COLOR_TYPE_RGBA_F16_VALUE;
        let rgbaf32 = &COLOR_TYPE_RGBA_F32_VALUE;

        PartialImageInfo {
            width: canvas_kit_image_info.width(),
            height: canvas_kit_image_info.height(),
            alphaType: match canvas_kit_image_info.alphaType().value() {
                opaque => AlphaType::Premul,
                premul => AlphaType::Unpremul,
                unpremul => AlphaType::Opaque,
            },
            colorType: match canvas_kit_image_info.colorType().value() {
                alpha8 => ColorType::Gray8,
                rgb565 => ColorType::Rgb565,
                rgba8888 => ColorType::Rgba8888,
                bgra8888 => ColorType::Bgra8888,
                rgba1010102 => ColorType::Rgba1010102,
                rgb101010x => ColorType::Rgb101010x,
                gray8 => ColorType::Gray8,
                rgbaf16 => ColorType::RgbaF16,
                rgbaf32 => ColorType::RgbaF32,
            },
        }
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
