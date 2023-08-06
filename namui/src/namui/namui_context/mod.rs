use crate::*;

pub struct NamuiContext {}

impl NamuiContext {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub fn start<C: Component>(self, component: impl Fn() -> C) {
        crate::hooks::channel::init();

        let tree_ctx = crate::hooks::TreeContext::new();
        tree_ctx.start(component);
    }
}
