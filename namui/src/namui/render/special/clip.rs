use super::SpecialRenderingNode;
use crate::{namui::ClipOp, PathBuilder};
use crate::{RenderingTree, Xy};
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

impl ClipNode {
    pub(crate) fn is_clip_in(&self, xy: Xy<f32>) -> bool {
        let path = self.path_builder.build();

        let clip_bounding_box = path.get_bounding_box();

        match self.clip_op {
            ClipOp::Intersect => match clip_bounding_box {
                Some(clip_bounding_box) => clip_bounding_box.is_xy_inside(xy),
                None => false,
            },
            ClipOp::Difference => match clip_bounding_box {
                Some(clip_bounding_box) => clip_bounding_box.is_xy_outside(xy),
                None => true,
            },
        }
    }
}
