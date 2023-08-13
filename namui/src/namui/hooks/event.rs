// use crate::*;
// use std::sync::{Arc, Mutex, OnceLock};

// #[derive(Debug)]
// pub(crate) enum RenderEvent<'a> {
//     Mount,
//     Event { event: Event<'a> },
//     ChannelEvents { channel_events: Vec<Item> },
// }
// unsafe impl Send for RenderEvent<'_> {}
// unsafe impl Sync for RenderEvent<'_> {}

// pub(crate) static RENDER_EVENT: OnceLock<Mutex<Arc<RenderEvent>>> = OnceLock::new();

// pub(crate) fn init_render_event(event: RenderEvent) {
//     RENDER_EVENT.set(Mutex::new(Arc::new(event))).unwrap();
// }

// pub(crate) fn set_render_event(web_event: RenderEvent) {
//     *RENDER_EVENT.get().unwrap().lock().unwrap() = Arc::new(web_event);
// }

// pub(crate) fn get_render_event() -> Arc<RenderEvent> {
//     RENDER_EVENT.get().unwrap().lock().unwrap().clone()
// }

// pub(crate) fn with_web_event(web_event: impl FnOnce(&WebEvent)) {
//     if let RenderEvent::WebEvent { web_event: event } = get_render_event().as_ref() {
//         web_event(event);
//     }
// }
