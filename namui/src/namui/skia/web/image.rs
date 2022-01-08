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

        let opaque = &alpha_type::Opaque;
        let premul = &alpha_type::Premul;
        let unpremul = &alpha_type::Unpremul;

        let alpha8 = &color_type::Alpha8;
        let rgb565 = &color_type::Rgb565;
        let rgba8888 = &color_type::Rgba8888;
        let bgra8888 = &color_type::Bgra8888;
        let rgba1010102 = &color_type::Rgba1010102;
        let rgb101010x = &color_type::Rgb101010x;
        let gray8 = &color_type::Gray8;
        let rgbaf16 = &color_type::RgbaF16;
        let rgbaf32 = &color_type::RgbaF32;

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
    pub fn size(&self) -> Wh<f32> {
        let canvas_kit_image_info = self.canvas_kit_image.getImageInfo();
        Wh {
            width: canvas_kit_image_info.width(),
            height: canvas_kit_image_info.height(),
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
