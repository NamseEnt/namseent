use super::*;

#[type_derives]
pub struct TransformNode {
    pub matrix: Matrix3x3,
    pub rendering_tree: Box<RenderingTree>,
}

pub fn transform(matrix: Matrix3x3, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Transform(TransformNode {
        matrix,
        rendering_tree: Box::new(rendering_tree),
    }))
}
