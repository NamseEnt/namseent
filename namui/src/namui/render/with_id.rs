use super::{RenderingTree, SpecialRenderingNode};
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct WithIdNode {
    pub(crate) rendering_tree: Vec<RenderingTree>,
    pub(crate) id: String,
}

impl RenderingTree {
    pub fn with_id(self, id: &str) -> RenderingTree {
        RenderingTree::Special(SpecialRenderingNode::WithId(WithIdNode {
            rendering_tree: vec![self],
            id: id.to_string(),
        }))
    }
}
