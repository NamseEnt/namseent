use super::SpecialRenderingNode;
use crate::{namui::render::Matrix3x3, RenderingTree};

#[derive(Debug, Clone)]
pub struct TransformNode {
    pub(crate) matrix: Matrix3x3,
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
}

pub fn transform(matrix: Matrix3x3, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Transform(TransformNode {
        matrix,
        rendering_tree: std::sync::Arc::new(rendering_tree),
    }))
}
