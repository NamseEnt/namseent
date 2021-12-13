use crate::namui::skia::LtrbRect;

pub struct XywhRect<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl XywhRect<f32> {
    pub fn into_ltrb(&self) -> LtrbRect {
        LtrbRect {
            left: self.x as f32,
            top: self.y as f32,
            right: self.x + self.width as f32,
            bottom: self.y + self.height as f32,
        }
    }
}
