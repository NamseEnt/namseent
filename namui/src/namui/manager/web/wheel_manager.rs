use crate::namui::{self, Xy};
use wasm_bindgen::{prelude::Closure, JsCast};

pub struct WheelManager {}

impl WheelManager {
    pub fn new() -> Self {
        let wheel_closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
            namui::event::send(namui::NamuiEvent::Wheel(Xy {
                x: event.delta_x() as f32,
                y: event.delta_y() as f32,
            }));
        }) as Box<dyn FnMut(_)>);

        let window = namui::window();
        let document = window.document().unwrap();
        document
            .add_event_listener_with_callback("wheel", wheel_closure.as_ref().unchecked_ref())
            .unwrap();
        wheel_closure.forget();

        Self {}
    }
}
