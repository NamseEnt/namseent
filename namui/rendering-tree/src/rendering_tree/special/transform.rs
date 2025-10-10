use super::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq, State)]
pub struct TransformNode {
    pub matrix: TransformMatrix,
    pub rendering_tree: Box<RenderingTree>,
}

pub fn transform(matrix: TransformMatrix, rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }
    RenderingTree::Special(SpecialRenderingNode::Transform(TransformNode {
        matrix,
        rendering_tree: rendering_tree.into(),
    }))
}
