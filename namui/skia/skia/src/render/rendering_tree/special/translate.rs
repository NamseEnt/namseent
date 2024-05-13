use super::*;

#[type_derives()]
pub struct TranslateNode {
    pub x: Px,
    pub y: Px,
    pub rendering_tree: Box<RenderingTree>,
}

pub fn translate(x: Px, y: Px, rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }

    RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
        x,
        y,
        rendering_tree: rendering_tree.into(),
    }))
}
