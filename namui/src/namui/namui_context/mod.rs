use crate::*;

pub struct NamuiContext {}

impl NamuiContext {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub fn start(self, component: &dyn crate::Component) {
        crate::hooks::channel::init();

        let mut tree_ctx = crate::hooks::TreeContext::new();
        loop {
            tree_ctx = mount_visit(component, tree_ctx);
            tree_ctx.before_re_render();
        }
    }
}
