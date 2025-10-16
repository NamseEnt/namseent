use super::*;

pub(crate) struct InternalRoot {
    root_component: RootComponent,
}
impl InternalRoot {
    pub(crate) fn new(root_component: RootComponent) -> Self {
        Self { root_component }
    }
}

impl Component for &InternalRoot {
    fn render(self, ctx: &RenderCtx) {
        // TODO: Add global systems like fire-and-forget particle.
        ctx.add(self.root_component);
    }
}
