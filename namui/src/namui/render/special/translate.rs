use super::SpecialRenderingNode;
use crate::RenderingTree;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct TranslateNode {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) rendering_tree: Box<RenderingTree>,
}

pub fn translate(x: f32, y: f32, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Translate(TranslateNode {
        x,
        y,
        rendering_tree: Box::new(rendering_tree),
    }))
}
