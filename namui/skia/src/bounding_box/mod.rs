mod draw_command;
mod rendering_tree;

use crate::*;
use namui_type::*;

pub trait BoundingBox {
    fn bounding_box(self) -> Option<Rect<Px>>;
}

impl BoundingBox for &Path {
    fn bounding_box(self) -> Option<Rect<Px>> {
        NativeCalculate::path_bounding_box(self, None)
    }
}
