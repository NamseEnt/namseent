use super::*;

/// `OnTopNode` ignores clip and draw on top of other nodes.
#[type_derives]
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
    RenderingTree::Special(SpecialRenderingNode::OnTop(OnTopNode {
        rendering_tree: Box::new(rendering_tree),
    }))
}
