use super::SpecialRenderingNode;
use crate::RenderingTree;
use crate::{namui::ClipOp, PathBuilder};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ClipNode {
    pub(crate) path_builder: PathBuilder,
    pub(crate) clip_op: ClipOp,
    pub(crate) rendering_tree: Box<RenderingTree>,
}

pub fn clip(
    path_builder: PathBuilder,
    clip_op: ClipOp,
    rendering_tree: RenderingTree,
) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
        path_builder,
        clip_op,
        rendering_tree: Box::new(rendering_tree),
    }))
}
