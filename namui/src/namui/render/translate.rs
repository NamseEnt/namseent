use super::{RenderingTree, SpecialRenderingNode};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct TranslateNode {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) rendering_tree: Vec<RenderingTree>,
}

pub fn translate(x: f32, y: f32, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
        x,
        y,
        rendering_tree: vec![rendering_tree],
    }))
}
