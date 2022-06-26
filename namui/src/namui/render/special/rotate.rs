use super::SpecialRenderingNode;
use crate::{namui::render::Matrix3x3, RenderingTree};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct RotateNode {
    pub(crate) radian: f32,
    pub(crate) rendering_tree: Box<RenderingTree>,
}

/// radian is in **cw** direction.
pub fn rotate(radian: f32, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Rotate(RotateNode {
        radian,
        rendering_tree: Box::new(rendering_tree),
    }))
}

impl RotateNode {
    pub(crate) fn get_matrix(&self) -> Matrix3x3 {
        let sin = self.radian.sin();
        let cos = self.radian.cos();

        Matrix3x3::new(cos, -sin, 0.0, sin, cos, 0.0, 0.0, 0.0, 1.0)
    }
    pub(crate) fn get_counter_wise_matrix(&self) -> Matrix3x3 {
        let sin = self.radian.sin();
        let cos = self.radian.cos();

        Matrix3x3::new(cos, sin, 0.0, -sin, cos, 0.0, 0.0, 0.0, 1.0)
    }
}
