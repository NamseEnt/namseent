use super::*;

pub(crate) fn mount_visit<'a>(
    component: &'a (dyn Component + 'a),
    tree_ctx: TreeContext,
) -> TreeContext {
    let component_instance = {
        tree_ctx
            .get_last_component_instance(component.static_type_id())
            .unwrap_or_else(|| Arc::new(ComponentInstance::new(component)))
    };
    let render_ctx = RenderCtx::new(component_instance, tree_ctx);
    let done = component.render(&render_ctx);

    done.tree_ctx
}
