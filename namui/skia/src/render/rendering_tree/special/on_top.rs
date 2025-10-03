use super::*;

/// `OnTopNode` ignores clip and draw on top of other nodes.
#[derive(Debug, PartialEq, Clone, Hash, Eq, bincode::Encode, bincode::Decode)]
pub struct OnTopNode {
    pub rendering_tree: Box<RenderingTree>,
}

/// `on_top` ignores clip and draw on top of other nodes.
/// If you want to attach event to on_top, make sure that you put `attach_event` inside `on_top`.
/// ```ignore
/// // X - wrong
/// namui::on_top(render([])).attach_event(|_| {});
/// // O - right
/// namui::on_top(render([]).attach_event(|_| {}));
/// ```
pub fn on_top(rendering_tree: RenderingTree) -> RenderingTree {
    if rendering_tree == RenderingTree::Empty {
        return RenderingTree::Empty;
    }
    RenderingTree::Special(SpecialRenderingNode::OnTop(OnTopNode {
        rendering_tree: rendering_tree.into(),
    }))
}
