use crate::*;

pub struct NamuiContext {}

impl NamuiContext {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub async fn start<C: Component>(self, component: impl Send + Sync + Fn() -> C + 'static) {
        crate::hooks::start(component).await;
    }
}
