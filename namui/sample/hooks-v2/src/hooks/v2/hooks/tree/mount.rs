use super::*;

pub(crate) fn mount_visit<'a>(component: &'a (dyn Component + 'a), tree_ctx: TreeContext)
//  -> ComponentTree
{
    let component_instance = Arc::new(ComponentInstance::new(component));
    let render_ctx = RenderCtx::new(ContextFor::Mount, component_instance, tree_ctx);
    component.render(&render_ctx);

    // done.component_tree
}
