use num::cast::AsPrimitive;

pub trait Ratio {
    fn as_f32(&self) -> f32;
}
