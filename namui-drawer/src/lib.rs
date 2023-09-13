mod draw;
mod load_image;
mod log;

use anyhow::*;
use draw::*;
use load_image::*;
use namui_skia::*;
use namui_type::*;
use std::sync::{Arc, OnceLock, RwLock};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

static SKIA: OnceLock<RwLock<Arc<dyn SkSkia + Send + Sync>>> = OnceLock::new();

fn skia() -> Arc<dyn SkSkia + Send + Sync> {
    SKIA.get().unwrap().read().unwrap().clone()
}

#[wasm_bindgen]
pub fn init(canvas: web_sys::HtmlCanvasElement) {
    namui_panic_hook::set_once();

    namui_type::set_log(|x| log::log(x));

    SKIA.set(RwLock::new(init_skia(Some(&canvas))))
        .map_err(|_| anyhow!("Failed to init skia"))
        .unwrap();
}

#[wasm_bindgen]
pub fn draw(bytes: &[u8]) {
    let input = DrawInput::from_postcard_bytes(bytes);
    let rendering_tree = input.rendering_tree;

    let ctx = { DrawContext::new(skia()) };

    ctx.canvas().clear(Color::WHITE);
    rendering_tree.draw(&ctx);
    ctx.surface().flush();
}

#[wasm_bindgen]
pub fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    skia().load_typeface(typeface_name, bytes);
}

#[wasm_bindgen]
pub fn load_image(image_source: Vec<u8>, image_bitmap: web_sys::ImageBitmap) {
    let image_source: ImageSource = postcard::from_bytes(&image_source).unwrap();
    skia().load_image(image_source, image_bitmap);
}

#[wasm_bindgen]
pub async fn encode_loaded_image_to_png(image: Vec<u8>) -> wasm_bindgen::JsValue {
    let image = Image::from_postcard_bytes(&image);

    let vec = skia().encode_loaded_image_to_png(&image).await;

    js_sys::Uint8Array::from(vec.as_ref()).into()
}

#[wasm_bindgen]
pub fn refresh_surface(canvas: web_sys::HtmlCanvasElement) {
    *SKIA.get().unwrap().write().unwrap() = init_skia(Some(&canvas));
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
