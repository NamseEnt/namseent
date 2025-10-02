use namui_skia::BoundingBox;
use namui_type::{Px, Rect};

pub fn bounding_box(target: impl BoundingBox) -> Option<Rect<Px>> {
    target.bounding_box()
}
