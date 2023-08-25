mod draw_command;
mod rendering_tree;

use crate::*;
pub(crate) use rendering_tree::Visit;

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
