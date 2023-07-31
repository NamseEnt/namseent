use super::SpecialRenderingNode;
use crate::{namui::render::Matrix3x3, Angle, RenderingTree};

#[derive(Debug, Clone, serde::Serialize)]
pub struct RotateNode {
    pub(crate) angle: Angle,
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
}

/// angle is in **cw** direction.
pub fn rotate(angle: Angle, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Rotate(RotateNode {
        angle,
        rendering_tree: std::sync::Arc::new(rendering_tree),
    }))
}

impl RotateNode {
    pub(crate) fn get_matrix(&self) -> Matrix3x3 {
        let sin = self.angle.sin();
        let cos = self.angle.cos();

        Matrix3x3::new(cos, -sin, 0.0, sin, cos, 0.0, 0.0, 0.0, 1.0)
    }
    pub(crate) fn get_counter_wise_matrix(&self) -> Matrix3x3 {
        let sin = self.angle.sin();
        let cos = self.angle.cos();

        Matrix3x3::new(cos, sin, 0.0, -sin, cos, 0.0, 0.0, 0.0, 1.0)
    }
}
