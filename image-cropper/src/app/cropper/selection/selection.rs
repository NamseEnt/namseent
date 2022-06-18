use super::RectSelection;
use namui::{LtrbRect, RenderingTree, Xy, XywhRect};

#[derive(Clone)]
pub enum Selection {
    RectSelection(RectSelection),
}
impl Selection {
    pub fn render(&self, scale: f32) -> RenderingTree {
        match self {
            Selection::RectSelection(selection) => selection.render(scale),
        }
    }

    fn get_polygon(&self) -> Vec<Xy<f32>> {
        match self {
            Selection::RectSelection(selection) => selection.get_polygon(),
        }
    }

    pub fn get_bounding_box(&self) -> Option<XywhRect<f32>> {
        let polygon = self.get_polygon();
        polygon
            .first()
            .and_then(|first_point| {
                Some(LtrbRect {
                    left: first_point.x,
                    top: first_point.y,
                    right: first_point.x,
                    bottom: first_point.y,
                })
            })
            .and_then(|initial_bounding_box| {
                let bounding_box = polygon.into_iter().fold(
                    initial_bounding_box,
                    |bounding_box: LtrbRect, point| LtrbRect {
                        left: bounding_box.left.min(point.x),
                        top: bounding_box.top.min(point.y),
                        right: bounding_box.right.max(point.x),
                        bottom: bounding_box.bottom.max(point.y),
                    },
                );
                Some(XywhRect {
                    x: bounding_box.left,
                    y: bounding_box.top,
                    width: bounding_box.right - bounding_box.left,
                    height: bounding_box.bottom - bounding_box.top,
                })
            })
    }
}
pub trait SelectionTrait {
    fn render(&self, scale: f32) -> RenderingTree;
    fn get_polygon(&self) -> Vec<Xy<f32>>;
}
