use std::sync::Arc;

use super::SpecialRenderingNode;
use crate::RenderingTree;

/// `OnTopNode` ignores clip and draw on top of other nodes.
#[derive(Debug, Clone)]
pub struct OnTopNode {
    pub(crate) rendering_tree: Arc<RenderingTree>,
}

/// `on_top` ignores clip and draw on top of other nodes.
/// If you want to attach event to on_top, make sure that you put `attach_event` inside `on_top`.
/// ```rust
/// // X - wrong
/// namui::on_top(render([])).attach_event(|_| {});
/// // O - right
/// namui::on_top(render([]).attach_event(|_| {}));
/// ```
pub fn on_top(rendering_tree: RenderingTree) -> RenderingTree {
    RenderingTree::Special(SpecialRenderingNode::OnTop(OnTopNode {
        rendering_tree: Arc::new(rendering_tree),
    }))
}
