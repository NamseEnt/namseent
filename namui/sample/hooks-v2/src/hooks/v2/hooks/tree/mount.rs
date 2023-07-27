use super::*;

pub(crate) fn mount_visit(component: &dyn Component) -> ComponentTree {
    let component_instance = Arc::new(ComponentInstance::new(component));

    hooks::ctx::set_up_before_render(ContextFor::Mount, component_instance);
    let done: RenderDone = component.render();

    done.component_tree
}
