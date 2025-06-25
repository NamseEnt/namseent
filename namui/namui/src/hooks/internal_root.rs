use super::*;

pub(crate) struct InternalRoot<Root: Component + Clone> {
    root_component: Root,
    particle_fire_and_forget_systems: namui_particle::FireAndForgetSystems,
}
impl<Root: Component + Clone> InternalRoot<Root> {
    pub(crate) fn new(root_component: Root) -> Self {
        Self {
            root_component,
            particle_fire_and_forget_systems: namui_particle::FireAndForgetSystems,
        }
    }
}

impl<Root: Component + Clone> Component for &InternalRoot<Root> {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(&self.particle_fire_and_forget_systems);
        ctx.add(self.root_component.clone());
    }
}
