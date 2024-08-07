use crate::*;
use num::cast::AsPrimitive;

#[type_derives(Copy, Eq, Hash)]
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

#[type_derives(Copy, Eq)]
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
    pub fn zero_wh(wh: Wh<T>) -> Self
    where
        T: From<f32>,
    {
        Rect::Xywh {
            x: 0.0.into(),
            y: 0.0.into(),
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
    #[inline(always)]
    pub fn set_x(&mut self, x: T) {
        match self {
            Rect::Xywh { x: _x, .. } => *_x = x,
            Rect::Ltrb { left: _left, .. } => *_left = x,
        }
    }
    #[inline(always)]
    pub fn set_y(&mut self, y: T) {
        match self {
            Rect::Xywh { y: _y, .. } => *_y = y,
            Rect::Ltrb { top: _top, .. } => *_top = y,
        }
    }
    #[inline(always)]
    pub fn set_left(&mut self, left: T) {
        match self {
            Rect::Xywh { x: _x, .. } => *_x = left,
            Rect::Ltrb { left: _left, .. } => *_left = left,
        }
    }
    #[inline(always)]
    pub fn set_top(&mut self, top: T) {
        match self {
            Rect::Xywh { y: _y, .. } => *_y = top,
            Rect::Ltrb { top: _top, .. } => *_top = top,
        }
    }
    #[inline(always)]
    pub fn update_x(&mut self, callback: impl FnOnce(&mut T)) {
        match self {
            Rect::Xywh { x, .. } => callback(x),
            Rect::Ltrb { left, .. } => callback(left),
        }
    }
    #[inline(always)]
    pub fn update_y(&mut self, callback: impl FnOnce(&mut T)) {
        match self {
            Rect::Xywh { y, .. } => callback(y),
            Rect::Ltrb { top, .. } => callback(top),
        }
    }
    #[inline(always)]
    pub fn update_left(&mut self, callback: impl FnOnce(&mut T)) {
        self.update_x(callback);
    }
    #[inline(always)]
    pub fn update_top(&mut self, callback: impl FnOnce(&mut T)) {
        self.update_y(callback);
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

    fn be_xywh(&mut self) {
        match self {
            Rect::Xywh { .. } => {}
            Rect::Ltrb {
                left,
                top,
                right,
                bottom,
            } => {
                *self = Rect::Xywh {
                    x: left.clone(),
                    y: top.clone(),
                    width: right.clone() - left.clone(),
                    height: bottom.clone() - top.clone(),
                };
            }
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
    #[inline(always)]
    pub fn set_width(&mut self, width: T) {
        match self {
            Rect::Xywh { width: _width, .. } => *_width = width,
            Rect::Ltrb {
                left, top, bottom, ..
            } => {
                *self = Rect::Xywh {
                    x: left.clone(),
                    y: top.clone(),
                    width,
                    height: bottom.clone() - top.clone(),
                };
            }
        }
    }
    #[inline(always)]
    pub fn set_height(&mut self, height: T) {
        match self {
            Rect::Xywh {
                height: _height, ..
            } => *_height = height,
            Rect::Ltrb {
                left, top, right, ..
            } => {
                *self = Rect::Xywh {
                    x: left.clone(),
                    y: top.clone(),
                    width: right.clone() - left.clone(),
                    height,
                };
            }
        }
    }
    #[inline(always)]
    pub fn update_width(&mut self, callback: impl FnOnce(&mut T)) {
        match self {
            Rect::Xywh { width, .. } => callback(width),
            Rect::Ltrb { .. } => {
                self.be_xywh();
                self.update_width(callback);
            }
        }
    }
    #[inline(always)]
    pub fn update_height(&mut self, callback: impl FnOnce(&mut T)) {
        match self {
            Rect::Xywh { height, .. } => callback(height),
            Rect::Ltrb { .. } => {
                self.be_xywh();
                self.update_height(callback);
            }
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
    fn be_ltrb(&mut self) {
        match self {
            Rect::Xywh {
                x,
                y,
                width,
                height,
            } => {
                *self = Rect::Ltrb {
                    left: x.clone(),
                    top: y.clone(),
                    right: x.clone() + width.clone(),
                    bottom: y.clone() + height.clone(),
                };
            }
            Rect::Ltrb { .. } => {}
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
    #[inline(always)]
    pub fn set_right(&mut self, right: T) {
        match self {
            Rect::Xywh { x, y, height, .. } => {
                *self = Rect::Ltrb {
                    left: x.clone(),
                    top: y.clone(),
                    right,
                    bottom: y.clone() + height.clone(),
                };
            }
            Rect::Ltrb { right: _right, .. } => {
                *_right = right;
            }
        }
    }
    #[inline(always)]
    pub fn set_bottom(&mut self, bottom: T) {
        match self {
            Rect::Xywh { x, y, width, .. } => {
                *self = Rect::Ltrb {
                    left: x.clone(),
                    top: y.clone(),
                    right: x.clone() + width.clone(),
                    bottom,
                };
            }
            Rect::Ltrb {
                bottom: _bottom, ..
            } => {
                *_bottom = bottom;
            }
        }
    }
    #[inline(always)]
    pub fn update_right(&mut self, callback: impl FnOnce(&mut T)) {
        match self {
            Rect::Ltrb { right, .. } => callback(right),
            Rect::Xywh { .. } => {
                self.be_ltrb();
                self.update_right(callback);
            }
        }
    }
    #[inline(always)]
    pub fn update_bottom(&mut self, callback: impl FnOnce(&mut T)) {
        match self {
            Rect::Ltrb { bottom, .. } => callback(bottom),
            Rect::Xywh { .. } => {
                self.be_ltrb();
                self.update_bottom(callback);
            }
        }
    }
}
impl<T> Rect<T>
where
    T: std::ops::Mul<f32, Output = T> + Clone,
{
    pub fn scale(&self, ratio: impl AsPrimitive<f32>) -> Self {
        let ratio = ratio.as_();
        match self {
            Rect::Xywh {
                x,
                y,
                width,
                height,
            } => Rect::Xywh {
                x: x.clone() * ratio,
                y: y.clone() * ratio,
                width: width.clone() * ratio,
                height: height.clone() * ratio,
            },
            Rect::Ltrb {
                left,
                top,
                right,
                bottom,
            } => Rect::Ltrb {
                left: left.clone() * ratio,
                top: top.clone() * ratio,
                right: right.clone() * ratio,
                bottom: bottom.clone() * ratio,
            },
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

impl<T> std::ops::Add<Xy<T>> for Rect<T>
where
    T: std::ops::Add<Output = T> + Clone,
{
    type Output = Rect<T>;
    fn add(self, rhs: Xy<T>) -> Self::Output {
        match self {
            Rect::Xywh {
                x,
                y,
                width,
                height,
            } => Rect::Xywh {
                x: x + rhs.x,
                y: y + rhs.y,
                width,
                height,
            },
            Rect::Ltrb {
                left,
                top,
                right,
                bottom,
            } => Rect::Ltrb {
                left: left + rhs.x.clone(),
                top: top + rhs.y.clone(),
                right: right + rhs.x,
                bottom: bottom + rhs.y,
            },
        }
    }
}

#[cfg(feature = "skia")]
impl From<Rect<Px>> for skia_safe::Rect {
    fn from(rect: Rect<Px>) -> Self {
        match rect {
            Rect::Xywh {
                x,
                y,
                width,
                height,
            } => skia_safe::Rect::new(
                x.as_f32(),
                y.as_f32(),
                x.as_f32() + width.as_f32(),
                y.as_f32() + height.as_f32(),
            ),
            Rect::Ltrb {
                left,
                top,
                right,
                bottom,
            } => skia_safe::Rect::new(left.into(), top.into(), right.into(), bottom.into()),
        }
    }
}

#[cfg(feature = "skia")]
impl<T> Into<Rect<T>> for skia_safe::Rect
where
    T: From<f32>,
{
    fn into(self) -> Rect<T> {
        Rect::Ltrb {
            left: self.left.into(),
            top: self.top.into(),
            right: self.right.into(),
            bottom: self.bottom.into(),
        }
    }
}

impl<T> Rect<T>
where
    T: Debug + rkyv::Archive,
    <T as rkyv::Archive>::Archived: Debug,
{
    pub fn map<U>(&self, f: impl Fn(&T) -> U) -> Rect<U>
    where
        U: Debug + rkyv::Archive,
        <U as rkyv::Archive>::Archived: Debug,
    {
        match self {
            Rect::Xywh {
                x,
                y,
                width,
                height,
            } => Rect::Xywh {
                x: f(x),
                y: f(y),
                width: f(width),
                height: f(height),
            },
            Rect::Ltrb {
                left,
                top,
                right,
                bottom,
            } => Rect::Ltrb {
                left: f(left),
                top: f(top),
                right: f(right),
                bottom: f(bottom),
            },
        }
    }
}
