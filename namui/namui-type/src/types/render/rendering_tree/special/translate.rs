use super::*;

#[type_derives(-serde::Deserialize)]
pub struct TranslateNode {
    pub x: Px,
    pub y: Px,
    pub rendering_tree: Box<RenderingTree>,
}

pub fn translate(x: Px, y: Px, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
        x,
        y,
        rendering_tree: Box::new(rendering_tree),
    }))
}
