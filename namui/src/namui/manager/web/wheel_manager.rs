use crate::namui::{self, Xy};
use wasm_bindgen::{prelude::Closure, JsCast};

pub struct WheelManager {}

impl WheelManager {
    pub fn new() -> Self {
        namui::window()
            .document()
            .unwrap()
            .add_event_listener_with_callback_and_add_event_listener_options(
                "wheel",
                Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
                    if event.ctrl_key() {
                        event.prevent_default()
                    }

                    namui::event::send(namui::NamuiEvent::Wheel(Xy {
                        x: event.delta_x() as f32,
                        y: event.delta_y() as f32,
                    }));
                }) as Box<dyn FnMut(_)>)
                .into_js_value()
                .unchecked_ref(),
                web_sys::AddEventListenerOptions::new().passive(false),
            )
            .unwrap();

        Self {}
    }
}
