use super::*;

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct ClipNode {
    pub path: Path,
    pub clip_op: ClipOp,
    pub rendering_tree: Box<RenderingTree>,
}
