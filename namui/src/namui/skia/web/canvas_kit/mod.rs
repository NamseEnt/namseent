use wasm_bindgen::prelude::*;
mod surface;
pub use surface::*;
mod canvas_kit;
pub use canvas_kit::*;
mod font_mgr_factory;
pub use font_mgr_factory::*;
mod font_mgr;
pub use font_mgr::*;
mod font;
pub use font::*;
mod paint;
pub use paint::*;
mod text_blob;
pub use text_blob::*;
mod text_blob_factory;
pub use text_blob_factory::*;
mod canvas;
pub use canvas::*;
mod embind_object;
pub use embind_object::*;
mod typeface;
pub use typeface::*;
mod enums;
pub use enums::*;
mod path;
pub use path::*;
mod image;
pub use image::*;
mod color_filter_factory;
pub use color_filter_factory::*;
mod matrix_3x3_helpers;
pub use matrix_3x3_helpers::*;

#[wasm_bindgen]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[wasm_bindgen]
pub struct Rect {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[wasm_bindgen]
pub struct RRect {
    rect: Rect,
    rx: f32,
    ry: f32,
}
// struct AnimatedImage;
struct Image;
