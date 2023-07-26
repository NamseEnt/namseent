use super::*;

pub(crate) fn mount_visit(component: &dyn Component) -> ComponentTree {
    let component_instance = Arc::new(ComponentInstance::new(
        new_component_id(),
        component.static_type_id(),
        component.static_type_name(),
    ));

    hooks::ctx::set_up_before_render(ContextFor::Mount, component_instance);
    let done: RenderDone = component.render();
    hooks::ctx::clear_up_before_render();

    done.component_tree
}
