use crate::*;
use js_sys::Float32Array;

pub trait ToFloat32Array {
    fn to_float32_array(&self) -> Float32Array;
}
impl ToFloat32Array for Color {
    fn to_float32_array(&self) -> Float32Array {
        let array = Float32Array::new_with_length(4);
        array.set_index(0, (self.r as f32) / 255.0);
        array.set_index(1, (self.g as f32) / 255.0);
        array.set_index(2, (self.b as f32) / 255.0);
        array.set_index(3, (self.a as f32) / 255.0);

        array
    }
}
impl ToFloat32Array for Ltrb<Px> {
    fn to_float32_array(&self) -> Float32Array {
        let array = Float32Array::new_with_length(4);
        array.set_index(0, self.left.as_f32());
        array.set_index(1, self.top.as_f32());
        array.set_index(2, self.right.as_f32());
        array.set_index(3, self.bottom.as_f32());

        array
    }
}
