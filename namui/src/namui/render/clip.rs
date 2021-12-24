use super::{RenderingTree, SpecialRenderingNode};
use crate::namui::{ClipOp, Path};
use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct Clip {
    pub(crate) path: Path,
    pub(crate) clip_op: ClipOp,
    pub(crate) rendering_tree: Vec<RenderingTree>,
}

pub fn clip(path: Path, clip_op: ClipOp, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Clip(Clip {
        path,
        clip_op,
        rendering_tree: vec![rendering_tree],
    }))
}
