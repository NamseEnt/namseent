use super::InitResult;
use crate::*;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::ImageBitmap;

pub(super) async fn init() -> InitResult {
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen()]
    fn requestDraw(draw_input: Uint8Array);

    #[wasm_bindgen()]
    fn loadTypeface(typeface_name: &str, buffer: Uint8Array);

    #[wasm_bindgen()]
    fn loadImage(
        imageSource: Vec<u8>, // ImageSource
        imageBitmap: ImageBitmap,
    );
}

static mut LAST_RENDERING_TREE: Option<RenderingTree> = None;

#[wasm_bindgen]
pub fn on_load_image() {
    if let Some(last_rendering_tree) = unsafe { &mut LAST_RENDERING_TREE } {
        let draw_input = DrawInput {
            rendering_tree: last_rendering_tree.clone(),
        };
        let buffer = Uint8Array::from(draw_input.to_vec().as_ref());

        requestDraw(buffer);
    }
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    if let Some(last_rendering_tree) = unsafe { &mut LAST_RENDERING_TREE } {
        if last_rendering_tree == &rendering_tree {
            return;
        }
    }

    unsafe {
        LAST_RENDERING_TREE = Some(rendering_tree.clone());
    }

    let draw_input = DrawInput { rendering_tree };
    let buffer = Uint8Array::from(draw_input.to_vec().as_ref());

    requestDraw(buffer);
}

pub(crate) fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    let buffer = Uint8Array::from(bytes);
    loadTypeface(typeface_name, buffer);
}

pub(crate) fn load_image(image_source: &ImageSource, image_bitmap: ImageBitmap) {
    let image_source = postcard::to_allocvec(image_source).unwrap();
    loadImage(image_source, image_bitmap);
}
