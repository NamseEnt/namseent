mod canvas;
mod canvas_kit;
mod color_filter_factory;
mod embind_object;
mod enums;
mod font;
mod font_mgr;
mod font_mgr_factory;
mod image;
mod matrix_3x3_helpers;
mod paint;
mod path;
mod runtime_effect;
mod runtime_effect_factory;
mod shader;
mod shader_factory;
mod surface;
mod text_blob;
mod text_blob_factory;
mod typeface;
mod typeface_factory;

pub use canvas::*;
pub use canvas_kit::*;
pub use color_filter_factory::*;
pub use enums::*;
pub use font::*;
pub use font_mgr::*;
pub use font_mgr_factory::*;
pub use image::*;
use js_sys::Float32Array;
pub use matrix_3x3_helpers::*;
use namui_type::*;
pub use paint::*;
pub use path::*;
pub use runtime_effect::*;
pub use runtime_effect_factory::*;
pub use shader::*;
pub use shader_factory::*;
pub use surface::*;
pub use text_blob::*;
pub use text_blob_factory::*;
pub use typeface::*;
pub use typeface_factory::*;
use wasm_bindgen::prelude::*;

pub trait IntoFloat32Array {
    fn into_float32_array(&self) -> Float32Array;
}
impl IntoFloat32Array for Color {
    fn into_float32_array(&self) -> Float32Array {
        let array = Float32Array::new_with_length(4);
        array.set_index(0, (self.r as f32) / 255.0);
        array.set_index(1, (self.g as f32) / 255.0);
        array.set_index(2, (self.b as f32) / 255.0);
        array.set_index(3, (self.a as f32) / 255.0);

        array
    }
}
impl IntoFloat32Array for Ltrb<Px> {
    fn into_float32_array(&self) -> Float32Array {
        let array = Float32Array::new_with_length(4);
        array.set_index(0, self.left.as_f32());
        array.set_index(1, self.top.as_f32());
        array.set_index(2, self.right.as_f32());
        array.set_index(3, self.bottom.as_f32());

        array
    }
}
