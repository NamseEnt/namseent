use super::*;
use crate::{image::ImageBitmap, *};
pub use base::*;
use js_sys::{Array, Function, Promise, Reflect};
use serde::Serialize;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use wasm_bindgen::JsValue;
use web_sys::{Blob, OffscreenCanvas};

unsafe impl Sync for CanvasKitImage {}
unsafe impl Send for CanvasKitImage {}

#[derive(Serialize)]
pub struct Image {
    id: String,
    #[serde(skip)]
    pub(crate) canvas_kit_image: CanvasKitImage,
    image_info: PartialImageInfo,
    #[serde(skip)]
    default_shader: Arc<Shader>,
    #[serde(skip)]
    image_bitmap: ImageBitmap,
}

static IMAGE_ID: AtomicUsize = AtomicUsize::new(0);

impl Image {
    pub(crate) fn new(canvas_kit_image: CanvasKitImage, image_bitmap: ImageBitmap) -> Self {
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
        let default_shader = Arc::new(Shader::new(canvas_kit_image.makeShaderOptions(
            TileMode::Clamp.into_canvas_kit(),
            TileMode::Clamp.into_canvas_kit(),
            FilterMode::Linear.into_canvas_kit(),
            MipmapMode::Linear.into_canvas_kit(),
        )));

        Image {
            id,
            canvas_kit_image,
            image_info,
            default_shader,
            image_bitmap,
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

    pub(crate) fn get_default_shader(&self) -> Arc<Shader> {
        self.default_shader.clone()
    }

    pub(crate) fn from_image_bitmap(image_bitmap: ImageBitmap) -> Image {
        let canvas_kit_image =
            canvas_kit().make_lazy_image_from_texture_source(&image_bitmap, None, None);
        Image::new(canvas_kit_image, image_bitmap)
    }
    pub(crate) async fn as_png_blob(&self) -> Blob {
        let image_size = self.size();
        let offscreen_canvas = OffscreenCanvas::new(
            image_size.width.as_f32() as u32,
            image_size.height.as_f32() as u32,
        )
        .unwrap();

        let context = offscreen_canvas
            .get_context("bitmaprenderer")
            .unwrap()
            .expect("Fail to get image bitmap rendering context")
            .into();
        let transfer_from_image_bitmap: Function =
            Reflect::get(&context, &JsValue::from_str("transferFromImageBitmap"))
                .unwrap()
                .into();
        let arguments = {
            let arguments = Array::new();
            arguments.push(&self.image_bitmap);
            arguments
        };
        Reflect::apply(&transfer_from_image_bitmap, &context, &arguments).unwrap();

        // Wrong implemented on web-sys https://github.com/rustwasm/wasm-bindgen/pull/3341
        let convert_to_blob: Function =
            Reflect::get(&offscreen_canvas, &JsValue::from_str("convertToBlob"))
                .unwrap()
                .into();

        let promise: Promise = Reflect::apply(&convert_to_blob, &offscreen_canvas, &Array::new())
            .unwrap()
            .into();

        let blob: Blob = wasm_bindgen_futures::JsFuture::from(promise)
            .await
            .unwrap()
            .into();

        blob
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

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
