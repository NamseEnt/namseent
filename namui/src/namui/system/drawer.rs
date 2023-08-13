use super::InitResult;
use crate::*;
use js_sys::{ArrayBuffer, Uint8Array};
use wasm_bindgen::prelude::wasm_bindgen;

pub(super) async fn init() -> InitResult {
    Ok(())
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen()]
    fn requestDraw(draw_input: ArrayBuffer);

    #[wasm_bindgen()]
    fn loadTypeface(typeface_name: &str, buffer: ArrayBuffer);
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    static mut LAST_RENDERING_TREE: Option<RenderingTree> = None;

    if let Some(last_rendering_tree) = unsafe { &mut LAST_RENDERING_TREE } {
        if last_rendering_tree == &rendering_tree {
            return;
        }
    }

    unsafe {
        LAST_RENDERING_TREE = Some(rendering_tree.clone());
    }

    let draw_input = DrawInput { rendering_tree };
    let buffer = Uint8Array::from(draw_input.to_vec().as_ref()).buffer();

    requestDraw(buffer);
}

pub(crate) fn load_typeface(typeface_name: &str, bytes: &[u8]) {
    let buffer = Uint8Array::from(bytes).buffer();
    loadTypeface(typeface_name, buffer);
}
