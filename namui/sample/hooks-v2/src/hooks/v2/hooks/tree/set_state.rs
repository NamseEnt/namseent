use super::*;

pub(crate) fn set_state_visit(
    component: &dyn Component,
    component_tree: ComponentTree,
    set_state_item: SetStateItem,
    updated_signals: Arc<Mutex<HashSet<SignalId>>>,
) -> ComponentTree {
    // TODO: This can be optimized by remembering the route to the component, without bfs.

    hooks::ctx::set_up_before_render(
        ContextFor::SetState {
            set_state_item,
            updated_signals,
            children: component_tree.children,
        },
        component_tree.component_instance,
    );
    let done: RenderDone = component.render();

    done.component_tree
}
