use super::*;

#[type_derives()]
pub struct ClipNode {
    pub path: Path,
    pub clip_op: ClipOp,
    pub rendering_tree: Box<RenderingTree>,
}
