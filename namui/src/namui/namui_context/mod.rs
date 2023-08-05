use crate::*;

pub struct NamuiContext {}

impl NamuiContext {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub fn start(self, component: impl Component) {
        crate::hooks::channel::init();

        let tree_ctx = crate::hooks::TreeContext::new();
        tree_ctx.start(component);
    }
}
