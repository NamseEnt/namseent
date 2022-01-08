use wasm_bindgen::prelude::*;
mod surface;
pub use surface::*;
mod canvas_kit;
pub use canvas_kit::*;
mod typeface_factory;
pub use typeface_factory::*;
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
pub use self::image::*;
mod color_filter_factory;
pub use color_filter_factory::*;

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
