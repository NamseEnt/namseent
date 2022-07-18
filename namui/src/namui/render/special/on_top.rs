use std::sync::Arc;

use super::SpecialRenderingNode;
use crate::RenderingTree;
use serde::Serialize;

/// `OnTopNode` ignores clip and draw on top of other nodes.
#[derive(Serialize, Clone, Debug)]
pub struct OnTopNode {
    pub(crate) rendering_tree: Arc<RenderingTree>,
}

/// `on_top` ignores clip and draw on top of other nodes.
pub fn on_top(rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::OnTop(OnTopNode {
        rendering_tree: Arc::new(rendering_tree),
    }))
}
