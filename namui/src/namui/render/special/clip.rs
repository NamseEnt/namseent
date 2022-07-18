use super::SpecialRenderingNode;
use crate::{namui::ClipOp, *};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct ClipNode {
    pub(crate) path_builder: PathBuilder,
    pub(crate) clip_op: ClipOp,
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
}

pub fn clip(
    path_builder: PathBuilder,
    clip_op: ClipOp,
    rendering_tree: RenderingTree,
) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
        path_builder,
        clip_op,
        rendering_tree: std::sync::Arc::new(rendering_tree),
    }))
}

impl ClipNode {
    pub(crate) fn is_clip_in(&self, xy: Xy<Px>) -> bool {
        let path = self.path_builder.build();

        let path_contains = path.contains(xy);

        match self.clip_op {
            ClipOp::Intersect => path_contains,
            ClipOp::Difference => !path_contains,
        }
    }
}
