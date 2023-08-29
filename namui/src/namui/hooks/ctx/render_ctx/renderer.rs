use super::*;

#[derive(Clone)]
pub(super) struct Renderer {
    pub(super) instance: Arc<ComponentInstance>,
    pub(super) updated_sigs: HashSet<SigId>,
    pub(super) tree_ctx: TreeContext,
}

impl Renderer {
    pub(super) fn render(
        &self,
        key_vec: KeyVec,
        component: impl Component,
        matrix: Matrix3x3,
        clippings: Vec<Clipping>,
    ) -> RenderingTree {
        let child_instance = self
            .instance
            .get_or_create_child_instance(key_vec, component.static_type_name());
        self.tree_ctx.render(
            component,
            child_instance,
            self.updated_sigs.clone(),
            matrix,
            clippings,
        )
    }

    pub(super) fn spawn_render_ctx(
        &self,
        key_vec: KeyVec,
        component_type_name: &'static str,
        matrix: Matrix3x3,
        clippings: Vec<Clipping>,
    ) -> RenderCtx {
        let child_instance = self
            .instance
            .get_or_create_child_instance(key_vec, component_type_name);
        self.tree_ctx
            .spawn_render_ctx(child_instance, self.updated_sigs.clone(), matrix, clippings)
    }
}
