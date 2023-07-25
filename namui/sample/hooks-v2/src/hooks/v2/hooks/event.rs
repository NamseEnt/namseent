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
