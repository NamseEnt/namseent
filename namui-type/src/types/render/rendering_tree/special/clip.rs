use super::*;

#[type_derives]
pub struct ClipNode {
    pub path: Path,
    pub clip_op: ClipOp,
    pub rendering_tree: Box<RenderingTree>,
}

pub fn clip(path: Path, clip_op: ClipOp, rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::Clip(ClipNode {
        path,
        clip_op,
        rendering_tree: Box::new(rendering_tree),
    }))
}
