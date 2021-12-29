use crate::{namui::skia::LtrbRect, Xy};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn center(&self) -> Xy<f32> {
        Xy {
            x: self.x + self.width / 2.0,
            y: self.y + self.height / 2.0,
        }
    }
}
