use super::*;

#[derive(Debug, bincode::Decode, bincode::Encode, PartialEq, Clone, Hash, Eq)]
pub struct ClipNode {
    pub path: Path,
    pub clip_op: ClipOp,
    pub rendering_tree: Box<RenderingTree>,
}
