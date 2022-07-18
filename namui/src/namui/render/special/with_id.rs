use super::SpecialRenderingNode;
use crate::RenderingTree;
use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct WithIdNode {
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
    pub(crate) id: String,
}

impl RenderingTree {
    pub fn with_id(self, id: &str) -> RenderingTree {
        RenderingTree::Special(SpecialRenderingNode::WithId(WithIdNode {
            rendering_tree: std::sync::Arc::new(self),
            id: id.to_string(),
        }))
    }
}
