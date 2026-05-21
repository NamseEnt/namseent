use super::*;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, bincode::Encode)]
pub struct TransformNode {
    pub matrix: TransformMatrix,
    pub rendering_tree: &'static RenderingTree,
}

pub fn transform(matrix: TransformMatrix, rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }
    RenderingTree::Special(SpecialRenderingNode::Transform(TransformNode {
        matrix,
        rendering_tree: arena_alloc(rendering_tree),
    }))
}
