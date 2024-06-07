use super::*;

#[type_derives(Hash, Eq, -serde::Serialize, -serde::Deserialize)]
pub struct ClipNode {
    pub path: Path,
    pub clip_op: ClipOp,
    pub rendering_tree: Box<RenderingTree>,
}
