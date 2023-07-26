use super::*;

pub(crate) fn event_visit(
    component: &dyn Component,
    component_tree: ComponentTree,
    event_callback: EventCallback,
) -> ComponentTree {
    hooks::ctx::set_up_before_render(
        ContextFor::Event { event_callback },
        component_tree.component_instance.clone(),
    );
    let done: RenderDone = component.render();
    hooks::ctx::clear_up_before_render();

    done.component_tree
}
