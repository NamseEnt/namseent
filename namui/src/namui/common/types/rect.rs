use super::*;
use crate::{Wh, Xy};

#[derive(Debug, serde::Deserialize, serde::Serialize, Clone, PartialEq)]
pub enum Rect<T> {
    Xywh {
        x: T,
        y: T,
        width: T,
        height: T,
    },
    Ltrb {
        left: T,
        top: T,
        right: T,
        bottom: T,
    },
}

pub struct Xywh<T> {
    pub x: T,
    pub y: T,
    pub width: T,
    pub height: T,
}

impl<T: Clone> Rect<T> {
    pub fn from_xy_wh(xy: Xy<T>, wh: Wh<T>) -> Self {
        Rect::Xywh {
            x: xy.x,
            y: xy.y,
            width: wh.width,
            height: wh.height,
        }
    }
    #[inline(always)]
    pub fn x(&self) -> T {
        match self {
            Rect::Xywh { x, .. } => x.clone(),
            Rect::Ltrb { left, .. } => left.clone(),
        }
    }
    #[inline(always)]
    pub fn y(&self) -> T {
        match self {
            Rect::Xywh { y, .. } => y.clone(),
            Rect::Ltrb { top, .. } => top.clone(),
        }
    }
    #[inline(always)]
    pub fn left(&self) -> T {
        match self {
            Rect::Xywh { x, .. } => x.clone(),
            Rect::Ltrb { left, .. } => left.clone(),
        }
    }
    #[inline(always)]
    pub fn top(&self) -> T {
        match self {
            Rect::Xywh { y, .. } => y.clone(),
            Rect::Ltrb { top, .. } => top.clone(),
        }
    }
}

impl<T> Rect<T>
where
    T: Clone + std::ops::Sub<Output = T>,
{
    pub fn as_xywh(&self) -> Xywh<T> {
        match self {
            Rect::Xywh {
                x,
                y,
                width,
                height,
            } => Xywh {
                x: x.clone(),
                y: y.clone(),
                width: width.clone(),
                height: height.clone(),
            },
            Rect::Ltrb {
                left,
                top,
                right,
                bottom,
            } => Xywh {
                x: left.clone(),
                y: top.clone(),
                width: right.clone() - left.clone(),
                height: bottom.clone() - top.clone(),
            },
        }
    }

    pub fn wh(&self) -> Wh<T> {
        let xywh = self.as_xywh();
        Wh {
            width: xywh.width,
            height: xywh.height,
        }
    }
    pub fn xy(&self) -> Xy<T> {
        let xywh = self.as_xywh();
        Xy {
            x: xywh.x,
            y: xywh.y,
        }
    }
    #[inline(always)]
    pub fn width(&self) -> T {
        match self {
            Rect::Xywh { width, .. } => width.clone(),
            Rect::Ltrb { right, left, .. } => right.clone() - left.clone(),
        }
    }
    #[inline(always)]
    pub fn height(&self) -> T {
        match self {
            Rect::Xywh { height, .. } => height.clone(),
            Rect::Ltrb { bottom, top, .. } => bottom.clone() - top.clone(),
        }
    }
}
impl<T> Rect<T>
where
    T: std::ops::Add<Output = T> + Clone,
{
    pub fn as_ltrb(&self) -> Ltrb<T> {
        match self {
            Rect::Xywh {
                x,
                y,
                width,
                height,
            } => Ltrb {
                left: x.clone(),
                top: y.clone(),
                right: x.clone() + width.clone(),
                bottom: y.clone() + height.clone(),
            },
            Rect::Ltrb {
                left,
                top,
                right,
                bottom,
            } => Ltrb {
                left: left.clone(),
                top: top.clone(),
                right: right.clone(),
                bottom: bottom.clone(),
            },
        }
    }
    #[inline(always)]
    pub fn right(&self) -> T {
        match self {
            Rect::Xywh { x, width, .. } => x.clone() + width.clone(),
            Rect::Ltrb { right, .. } => right.clone(),
        }
    }
    #[inline(always)]
    pub fn bottom(&self) -> T {
        match self {
            Rect::Xywh { y, height, .. } => y.clone() + height.clone(),
            Rect::Ltrb { bottom, .. } => bottom.clone(),
        }
    }
}

impl<'a, T> Rect<T>
where
    T: 'a + std::ops::Div<f32, Output = T>,
    &'a T: std::ops::Add<&'a T, Output = T>
        + std::ops::Div<f32, Output = T>
        + std::ops::Add<T, Output = T>,
{
    pub fn center(&'a self) -> Xy<T> {
        match self {
            Rect::Xywh {
                x,
                y,
                width,
                height,
            } => Xy {
                x: x + width / 2.0,
                y: y + height / 2.0,
            },
            Rect::Ltrb {
                left,
                top,
                right,
                bottom,
            } => Xy {
                x: (left + right) / 2.0,
                y: (top + bottom) / 2.0,
            },
        }
    }
}

impl<T> Rect<T>
where
    T: PartialOrd + std::ops::Add<T, Output = T> + Clone,
{
    pub fn intersect(&self, other: Rect<T>) -> Option<Rect<T>> {
        let my_ltrb = self.as_ltrb();
        let other_ltrb = other.as_ltrb();

        let is_intersect = my_ltrb.left <= other_ltrb.right
            && my_ltrb.right >= other_ltrb.left
            && my_ltrb.top <= other_ltrb.bottom
            && my_ltrb.bottom >= other_ltrb.top;

        is_intersect.then(|| Rect::Ltrb {
            left: if my_ltrb.left > other_ltrb.left {
                my_ltrb.left
            } else {
                other_ltrb.left
            },
            top: if my_ltrb.top > other_ltrb.top {
                my_ltrb.top
            } else {
                other_ltrb.top
            },
            right: if my_ltrb.right < other_ltrb.right {
                my_ltrb.right
            } else {
                other_ltrb.right
            },
            bottom: if my_ltrb.bottom < other_ltrb.bottom {
                my_ltrb.bottom
            } else {
                other_ltrb.bottom
            },
        })
    }

    pub fn is_xy_outside(&self, xy: Xy<T>) -> bool {
        let Ltrb {
            left,
            top,
            right,
            bottom,
        } = self.as_ltrb();

        xy.x < left || xy.x > right || xy.y < top || xy.y > bottom
    }

    pub fn is_xy_on_border(&self, xy: Xy<T>) -> bool {
        let Ltrb {
            left,
            top,
            right,
            bottom,
        } = self.as_ltrb();

        ((xy.x == left || xy.x == right) && (top <= xy.y && xy.y <= bottom))
            || ((xy.y == top || xy.y == bottom) && (left <= xy.x && xy.x <= right))
    }

    pub fn is_xy_inside(&self, xy: Xy<T>) -> bool {
        let Ltrb {
            left,
            top,
            right,
            bottom,
        } = self.as_ltrb();

        left <= xy.x && xy.x <= right && top <= xy.y && xy.y <= bottom
    }

    pub fn get_minimum_rectangle_containing(&self, other: Rect<T>) -> Rect<T> {
        let my_ltrb = self.as_ltrb();
        let other_ltrb = other.as_ltrb();
        Rect::Ltrb {
            left: if my_ltrb.left < other_ltrb.left {
                my_ltrb.left
            } else {
                other_ltrb.left
            },
            top: if my_ltrb.top < other_ltrb.top {
                my_ltrb.top
            } else {
                other_ltrb.top
            },
            right: if my_ltrb.right > other_ltrb.right {
                my_ltrb.right
            } else {
                other_ltrb.right
            },
            bottom: if my_ltrb.bottom > other_ltrb.bottom {
                my_ltrb.bottom
            } else {
                other_ltrb.bottom
            },
        }
    }
}

impl<T> Copy for Rect<T> where T: Copy {}
impl<T> Default for Rect<T>
where
    T: Default,
{
    fn default() -> Self {
        Rect::Ltrb {
            left: T::default(),
            top: T::default(),
            right: T::default(),
            bottom: T::default(),
        }
    }
}
