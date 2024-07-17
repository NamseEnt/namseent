use super::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct AbsoluteNode {
    pub x: Px,
    pub y: Px,
    pub rendering_tree: Box<RenderingTree>,
}

pub fn absolute(x: Px, y: Px, rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }
    RenderingTree::Special(SpecialRenderingNode::Absolute(AbsoluteNode {
        x,
        y,
        rendering_tree: rendering_tree.into(),
    }))
}
