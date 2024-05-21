mod log;

use namui_skia::*;
use namui_type::*;
use std::cell::RefCell;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(test)]
#[cfg(target_family = "wasm")]
wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

thread_local! {
    static SKIA: RefCell<Option<CkSkia>> = const { RefCell::new(None) };
}

#[wasm_bindgen]
pub async fn init(canvas: web_sys::HtmlCanvasElement) -> Result<(), String> {
    namui_type::set_log(|x| log::log(x));
    namui_panic_hook::set_once();
    
    let skia = init_skia(&canvas).await.map_err(|e| e.to_string())?;
    SKIA.set(Some(skia));

    Ok(())
}

#[wasm_bindgen]
pub fn draw(bytes: &[u8]) {
    let rendering_tree = RenderingTree::from_postcard_bytes(bytes);

    SKIA.with_borrow_mut(|skia| namui_drawer_sys::draw(skia.as_mut().unwrap(), rendering_tree))
}

#[wasm_bindgen]
pub fn load_typeface(typeface_name: &str, bytes: &[u8]) -> Result<(), String> {
    SKIA.with_borrow(|skia| skia.as_ref().unwrap().load_typeface(typeface_name, bytes))
        .map_err(|e| e.to_string())
}

#[wasm_bindgen]
/// output: ImageLoaded
pub fn load_image(image_bitmap: web_sys::ImageBitmap) -> Vec<u8> {
    SKIA.with_borrow(|skia| {
        let image_loaded = skia
            .as_ref()
            .unwrap()
            .load_image_from_web_image_bitmap(image_bitmap);
        postcard::to_stdvec(&image_loaded).unwrap()
    })
}

#[wasm_bindgen]
pub fn unload_image(image_id: u32) {
    SKIA.with_borrow_mut(|skia| skia.as_mut().unwrap().unload_image(image_id))
}

#[wasm_bindgen]
pub async fn refresh_surface(canvas: web_sys::HtmlCanvasElement) {
    let new_skia = init_skia(&canvas).await.unwrap();
    SKIA.with_borrow_mut(|skia| skia.replace(new_skia)).unwrap();
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {{
        $crate::log::log(format!($($arg)*));
    }}
}
