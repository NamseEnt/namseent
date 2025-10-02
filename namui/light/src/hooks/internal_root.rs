use super::*;

pub(crate) struct InternalRoot<Root: Fn(&RenderCtx) + Send + Sync + 'static> {
    root_component: Root,
    particle_fire_and_forget_systems: namui_particle::FireAndForgetSystems,
}
impl<Root: Fn(&RenderCtx) + Send + Sync + 'static> InternalRoot<Root> {
    pub(crate) fn new(root_component: Root) -> Self {
        Self {
            root_component,
            particle_fire_and_forget_systems: namui_particle::FireAndForgetSystems,
        }
    }
}

impl<Root: Fn(&RenderCtx) + Send + Sync + 'static> Component for &InternalRoot<Root> {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(&self.particle_fire_and_forget_systems);
        ctx.add(&self.root_component);
    }
}
