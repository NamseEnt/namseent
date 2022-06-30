use super::*;
use js_sys::*;

impl Color {
    pub fn into_float32_array(&self) -> Float32Array {
        let array = Float32Array::new_with_length(4);
        array.set_index(0, (self.r as f32) / 255.0);
        array.set_index(1, (self.g as f32) / 255.0);
        array.set_index(2, (self.b as f32) / 255.0);
        array.set_index(3, (self.a as f32) / 255.0);

        array
    }
}
