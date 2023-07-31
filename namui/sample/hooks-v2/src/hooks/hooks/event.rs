use super::*;

#[derive(Clone, Debug)]
pub struct EventCallback {
    pub(crate) component_id: usize,
    pub(crate) event: Arc<dyn Any + Send + Sync>,
}
unsafe impl Send for EventCallback {}
unsafe impl Sync for EventCallback {}

impl EventCallback {
    pub(crate) fn call(&self) {
        channel::send(channel::Item::EventCallback(self.clone()));
    }
}

#[derive(Clone)]
pub struct EventCallbackWithParam<Param> {
    pub(crate) component_id: usize,
    pub(crate) closure: Arc<dyn (Fn(Param) -> Option<Arc<(dyn Any + Send + Sync)>>) + Send + Sync>,
}
unsafe impl<Param> Send for EventCallbackWithParam<Param> {}
unsafe impl<Param> Sync for EventCallbackWithParam<Param> {}

impl<Param> Debug for EventCallbackWithParam<Param> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EventCallbackWithParam")
            .field("component_id", &self.component_id)
            .finish()
    }
}

impl<Param> EventCallbackWithParam<Param> {
    pub(crate) fn call(&self, param: Param) {
        let event = (self.closure)(param);

        if let Some(event) = event {
            channel::send(channel::Item::EventCallback(EventCallback {
                component_id: self.component_id,
                event,
            }))
        }
    }
}
