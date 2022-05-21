use super::SpecialRenderingNode;
use crate::{namui::render::Matrix3x3, RenderingTree};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct RotateNode {
    pub(crate) ccw_radian: f32,
    pub(crate) rendering_tree: Box<RenderingTree>,
}

pub fn rotate(ccw_radian: f32, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Rotate(RotateNode {
        ccw_radian,
        rendering_tree: Box::new(rendering_tree),
    }))
}

impl RotateNode {
    pub(crate) fn get_matrix(&self) -> Matrix3x3 {
        let sin = self.ccw_radian.sin();
        let cos = self.ccw_radian.cos();

        Matrix3x3::new(cos, sin, 0.0, -sin, cos, 0.0, 0.0, 0.0, 1.0)
    }
    pub(crate) fn get_counter_wise_matrix(&self) -> Matrix3x3 {
        let sin = self.ccw_radian.sin();
        let cos = self.ccw_radian.cos();

        Matrix3x3::new(cos, -sin, 0.0, sin, cos, 0.0, 0.0, 0.0, 1.0)
    }
}
