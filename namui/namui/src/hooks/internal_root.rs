use super::*;

pub(crate) struct InternalRoot<Root: Component + Clone> {
    root_component: Root,
}
impl<Root: Component + Clone> InternalRoot<Root> {
    pub(crate) fn new(root_component: Root) -> Self {
        Self { root_component }
    }
}

impl<Root: Component + Clone> Component for &InternalRoot<Root> {
    fn render(self, ctx: &RenderCtx) {
        // TODO: Add global systems like fire-and-forget particle.
        ctx.add(self.root_component.clone());
    }
}
