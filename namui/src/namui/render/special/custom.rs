use super::SpecialRenderingNode;
use crate::RenderingTree;
use std::sync::Arc;

type CustomData = Arc<dyn std::any::Any>;

#[derive(Debug, Clone, serde::Serialize)]
pub struct CustomNode {
    pub(crate) rendering_tree: std::sync::Arc<RenderingTree>,
    #[serde(skip)]
    pub data: CustomData,
}

impl PartialEq for CustomNode {
    fn eq(&self, _other: &Self) -> bool {
        // TODO
        false
    }
}

impl RenderingTree {
    pub fn with_custom(self, data: impl std::any::Any) -> RenderingTree {
        RenderingTree::Special(SpecialRenderingNode::Custom(CustomNode {
            rendering_tree: std::sync::Arc::new(self),
            data: Arc::new(data),
        }))
    }
}
