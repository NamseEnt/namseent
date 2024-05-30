use ordered_float::OrderedFloat;

pub trait Ratio {
    fn as_f32(&self) -> f32;
}

impl Ratio for i32 {
    fn as_f32(&self) -> f32 {
        *self as f32
    }
}
impl Ratio for f32 {
    fn as_f32(&self) -> f32 {
        *self
    }
}
impl Ratio for f64 {
    fn as_f32(&self) -> f32 {
        *self as f32
    }
}
impl Ratio for usize {
    fn as_f32(&self) -> f32 {
        *self as f32
    }
}
impl Ratio for OrderedFloat<f32> {
    fn as_f32(&self) -> f32 {
        self.0
    }
}
