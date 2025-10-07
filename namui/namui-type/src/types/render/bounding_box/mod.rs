mod draw_command;
mod rendering_tree;

use crate::*;

pub trait BoundingBox {
    fn bounding_box(self) -> Option<Rect<Px>>;
}

impl BoundingBox for &Path {
    fn bounding_box(self) -> Option<Rect<Px>> {
        SkCalculate::path_bounding_box(self, None)
    }
}
