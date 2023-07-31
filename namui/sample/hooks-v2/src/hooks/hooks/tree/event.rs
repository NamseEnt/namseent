use super::*;

pub(crate) fn event_visit(
    component: &dyn Component,
    component_tree: ComponentTree,
    event_callback: EventCallback,
) -> ComponentTree {
    // TODO: This can be optimized by remembering the route to the component, without bfs.

    hooks::ctx::set_up_before_render(
        ContextFor::Event {
            event_callback,
            children_tree: component_tree.children,
        },
        component_tree.component_instance,
    );
    let done: RenderDone = component.render();

    done.component_tree
}
