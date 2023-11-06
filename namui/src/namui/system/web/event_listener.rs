use crate::system::platform_utils::window;
use wasm_bindgen::{prelude::Closure, JsCast};

enum WebEvent {
    HashChange { event: web_sys::HashChangeEvent },
}

static mut LAST_WEB_EVENT: Option<WebEvent> = None;

pub(super) fn init() {
    window()
        .add_event_listener_with_callback(
            "hashchange",
            Closure::wrap(Box::new(move |event: web_sys::HashChangeEvent| {
                unsafe {
                    LAST_WEB_EVENT = Some(WebEvent::HashChange { event });
                }
                crate::hooks::render_and_draw();
                unsafe {
                    LAST_WEB_EVENT = None;
                }
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();
}

pub fn event_listener_hash_change(func: impl FnOnce(&web_sys::HashChangeEvent)) {
    unsafe {
        if let Some(WebEvent::HashChange { event }) = &LAST_WEB_EVENT {
            func(event);
        }
    }
}
