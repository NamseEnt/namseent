use super::SpecialRenderingNode;
use crate::{Px, RenderingTree};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct TranslateNode {
    pub(crate) x: Px,
    pub(crate) y: Px,
    pub(crate) rendering_tree: Box<RenderingTree>,
}

pub fn translate(x: Px, y: Px, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
        x,
        y,
        rendering_tree: Box::new(rendering_tree),
    }))
}
