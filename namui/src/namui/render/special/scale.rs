use super::SpecialRenderingNode;
use crate::{namui::render::Matrix3x3, RenderingTree};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ScaleNode {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
}

pub fn scale(x: f32, y: f32, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Scale(ScaleNode {
        x,
        y,
        rendering_tree: std::sync::Arc::new(rendering_tree),
    }))
}
impl ScaleNode {
    pub(crate) fn get_matrix(&self) -> Matrix3x3 {
        Matrix3x3::new(self.x, 0.0, 0.0, 0.0, self.y, 0.0, 0.0, 0.0, 1.0)
    }
}
