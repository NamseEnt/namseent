mod draw;
mod load_image;
mod log;

use anyhow::*;
use draw::*;
use load_image::*;
use namui_skia::*;
use namui_type::*;
use std::sync::{Arc, OnceLock};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

static SKIA: OnceLock<Arc<dyn SkSkia + Send + Sync>> = OnceLock::new();

#[wasm_bindgen]
pub fn init(canvas: web_sys::HtmlCanvasElement) {
    namui_type::set_log(|x| log::log(x));

    SKIA.set(init_skia(Some(&canvas)))
        .map_err(|_| anyhow!("Failed to init skia"))
        .unwrap();
}

#[wasm_bindgen]
pub fn draw(bytes: &[u8]) {
    let input = DrawInput::from_postcard_bytes(bytes);
    let rendering_tree = input.rendering_tree;

    let ctx = DrawContext::new(SKIA.get().unwrap().clone());

    ctx.canvas().clear(Color::WHITE);
    rendering_tree.draw(&ctx);
    ctx.surface().flush();
}

#[wasm_bindgen]
pub fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    SKIA.get().unwrap().load_typeface(typeface_name, bytes);
}

#[wasm_bindgen]
pub fn load_image(image_source: Vec<u8>, image_bitmap: web_sys::ImageBitmap) {
    let image_source: ImageSource = postcard::from_bytes(&image_source).unwrap();
    SKIA.get().unwrap().load_image(&image_source, &image_bitmap);
}

#[wasm_bindgen]
pub fn encode_loaded_image_to_png(image: Vec<u8>) -> Vec<u8> {
    let image = Image::from_postcard_bytes(&image);
    SKIA.get().unwrap().encode_loaded_image_to_png(&image)
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
