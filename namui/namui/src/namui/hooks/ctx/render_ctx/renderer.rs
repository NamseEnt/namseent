use super::*;

#[derive(Clone)]
pub(super) struct Renderer {
    pub(super) instance: Arc<ComponentInstance>,
}

impl Renderer {
    pub(super) fn render(&self, key_vec: KeyVec, component: impl Component) -> RenderingTree {
        let child_instance = self
            .instance
            .get_or_create_child_instance(key_vec, component.static_type_name());
        global_state::tree_ctx().render(component, child_instance)
    }
}
