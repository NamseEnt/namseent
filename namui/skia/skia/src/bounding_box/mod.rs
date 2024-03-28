mod draw_command;
mod rendering_tree;

use crate::*;
use namui_type::*;

pub trait BoundingBox {
    fn bounding_box(self, calculator: &dyn SkCalculate) -> Option<Rect<Px>>;
}

impl BoundingBox for &Path {
    fn bounding_box(self, calculator: &dyn SkCalculate) -> Option<Rect<Px>> {
        calculator.path_bounding_box(self, None)
    }
}
