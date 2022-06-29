use crate::{namui::skia::LtrbRect, Wh, Xy};
use serde::{Deserialize, Serialize};
use std::hash::Hash;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
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
impl<T> XywhRect<T>
where
    T: Clone,
{
    pub fn wh(&self) -> Wh<T> {
        Wh {
            width: self.width.clone(),
            height: self.height.clone(),
        }
    }
    pub fn xy(&self) -> Xy<T> {
        Xy {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
    pub fn from_xy_wh(xy: Xy<T>, wh: Wh<T>) -> Self {
        XywhRect {
            x: xy.x,
            y: xy.y,
            width: wh.width,
            height: wh.height,
        }
    }
}

impl<T> XywhRect<T>
where
    T: Copy + std::cmp::PartialOrd + std::ops::Add<Output = T>,
{
    pub fn is_xy_in(&self, xy: &Xy<T>) -> bool {
        self.x <= xy.x
            && xy.x <= self.x + self.width
            && self.y <= xy.y
            && xy.y <= self.y + self.height
    }
}
