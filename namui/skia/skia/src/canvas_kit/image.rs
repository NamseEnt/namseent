use super::*;
use crate::*;
use std::sync::Arc;
use wasm_bindgen::JsCast;
use web_sys::ImageBitmap;

pub struct CkImage {
    pub(crate) canvas_kit_image: CanvasKitImage,
    image_info: ImageInfo,
    src: ImageSource,
    image_bitmap: ImageBitmap,
}

unsafe impl Send for CkImage {}
unsafe impl Sync for CkImage {}

static IMAGE_MAP: StaticHashMap<ImageSource, CkImage> = StaticHashMap::new();

impl CkImage {
    pub(crate) fn load(image_source: ImageSource, image_bitmap: ImageBitmap) {
        let canvas_kit_image =
            canvas_kit().make_lazy_image_from_texture_source(&image_bitmap, None, None);

        let image_info = get_image_info(&canvas_kit_image);

        let ck_image = CkImage {
            canvas_kit_image,
            image_info,
            src: image_source.clone(),
            image_bitmap,
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

    pub(crate) async fn encode_to_png(&self) -> Vec<u8> {
        let offscreen_canvas = web_sys::OffscreenCanvas::new(
            self.image_info.width.as_f32() as u32,
            self.image_info.height.as_f32() as u32,
        )
        .expect("Failed to create offscreen canvas");

        let ctx = offscreen_canvas
            .get_context("2d")
            .expect("Failed to call get_context('2d')")
            .expect("Offscreen canvas return null for 2d context")
            .dyn_into::<web_sys::OffscreenCanvasRenderingContext2d>()
            .expect("Failed to cast 2d context");

        ctx.draw_image_with_image_bitmap(&self.image_bitmap, 0.0, 0.0)
            .expect("Failed to call draw_image_with_image_bitmap()");

        let image_data: web_sys::Blob =
            wasm_bindgen_futures::JsFuture::from(offscreen_canvas.convert_to_blob().unwrap())
                .await
                .expect("Failed to call convert_to_blob()")
                .dyn_into()
                .expect("Failed to cast canvas to blob");

        let array_buffer: js_sys::ArrayBuffer =
            wasm_bindgen_futures::JsFuture::from(image_data.array_buffer())
                .await
                .expect("Failed to call array_buffer()")
                .dyn_into()
                .expect("Failed to cast blob to array buffer");

        let array = js_sys::Uint8Array::new(&array_buffer);
        array.to_vec()
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
            value if ALPHA_TYPE_OPAQUE_VALUE.eq(&value) => AlphaType::Opaque,
            value if ALPHA_TYPE_PREMUL_VALUE.eq(&value) => AlphaType::Premul,
            value if ALPHA_TYPE_UNPREMUL_VALUE.eq(&value) => AlphaType::Unpremul,
            value => panic!("Unknown alpha type: {}", value),
        },
        color_type: match canvas_kit_image_info.colorType().value() {
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
}
