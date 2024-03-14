mod draw_command;
mod rendering_tree;

use crate::*;

pub trait BoundingBox {
    fn xy_in(&self, xy: Xy<Px>) -> bool;
    fn bounding_box(&self) -> Option<Rect<Px>>;
}

impl BoundingBox for Path {
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        crate::system::skia::path_contains_xy(self, None, xy)
    }

    fn bounding_box(&self) -> Option<Rect<Px>> {
        crate::system::skia::path_bounding_box(self, None)
    }
}

impl<'a, T> BoundingBox for &'a T
where
    T: BoundingBox,
{
    fn xy_in(&self, xy: Xy<Px>) -> bool {
        T::xy_in(*self, xy)
    }

    fn bounding_box(&self) -> Option<Rect<Px>> {
        T::bounding_box(*self)
    }
}
