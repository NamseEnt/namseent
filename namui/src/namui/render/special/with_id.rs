use super::SpecialRenderingNode;
use crate::RenderingTree;
use serde::Serialize;

#[derive(Serialize, Clone, Debug, PartialEq)]
pub struct WithIdNode {
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
    pub(crate) id: crate::Uuid,
}

impl RenderingTree {
    pub fn with_id(self, id: crate::Uuid) -> RenderingTree {
        RenderingTree::Special(SpecialRenderingNode::WithId(WithIdNode {
            rendering_tree: std::sync::Arc::new(self),
            id,
        }))
    }
}
