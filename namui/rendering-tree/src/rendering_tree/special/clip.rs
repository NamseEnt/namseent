use super::*;

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, bincode::Encode)]
pub struct ClipNode {
    pub path: &'static Path,
    pub clip_op: ClipOp,
    pub rendering_tree: &'static RenderingTree,
}
