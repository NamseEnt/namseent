use super::SpecialRenderingNode;
use crate::{Px, RenderingTree};
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct TranslateNode {
    pub(crate) x: Px,
    pub(crate) y: Px,
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
}

pub fn translate(x: Px, y: Px, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
        x,
        y,
        rendering_tree: std::sync::Arc::new(rendering_tree),
    }))
}
