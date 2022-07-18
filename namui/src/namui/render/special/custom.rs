use super::SpecialRenderingNode;
use crate::RenderingTree;
use serde::Serialize;
use std::sync::Arc;

type CustomData = Arc<dyn std::any::Any>;

#[derive(Serialize, Clone, Debug)]
pub struct CustomNode {
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
    #[serde(skip_serializing)]
    pub data: CustomData,
}

impl RenderingTree {
    pub fn with_custom(self, data: impl std::any::Any) -> RenderingTree {
        RenderingTree::Special(SpecialRenderingNode::Custom(CustomNode {
            rendering_tree: std::sync::Arc::new(self),
            data: Arc::new(data),
        }))
    }
}
