use super::*;

#[derive(Clone)]
pub(super) struct Renderer {
    pub(crate) instance: Rc<ComponentInstance>,
    pub(super) updated_sigs: Vec<SigId>,
}

impl Renderer {
    pub(super) fn render(
        &self,
        key_vec: KeyVec,
        component: impl Component,
        matrix: Matrix3x3,
        clippings: Vec<Clipping>,
        raw_event: RawEventContainer,
    ) -> RenderingTree {
        let child_instance = self
            .instance
            .get_or_create_child_instance(key_vec, component.static_type_name());
        tree_ctx().render(
            component,
            child_instance,
            self.updated_sigs.clone(),
            matrix,
            clippings,
            raw_event,
        )
    }
}
