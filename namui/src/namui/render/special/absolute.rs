use super::SpecialRenderingNode;
use crate::{Px, RenderingTree};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct AbsoluteNode {
    pub(crate) x: Px,
    pub(crate) y: Px,
    pub(crate) rendering_tree: Box<RenderingTree>,
}

pub fn absolute(x: Px, y: Px, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Absolute(AbsoluteNode {
        x,
        y,
        rendering_tree: Box::new(rendering_tree),
    }))
}
