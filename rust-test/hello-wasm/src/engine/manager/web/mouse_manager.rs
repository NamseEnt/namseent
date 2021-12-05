use std::sync::{Arc, RwLock};

use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;

use crate::engine::{self, engine_state::EngineState, Engine, EngineImpl, EngineInternal, Xy};

pub struct MouseManager {
    pub mouse_position: Arc<RwLock<Xy<i16>>>,
}

impl MouseManager {
    pub fn mouse_position(&self) -> Xy<i16> {
        (*self.mouse_position).read().unwrap().clone()
    }
    pub fn new(element: &HtmlElement) -> Self {
        let mouse_position = Arc::new(RwLock::new(Xy::<i16> { x: 0, y: 0 }));
        let mouse_manager = Self {
            mouse_position: mouse_position.clone(),
        };

        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut mouse_position = (*mouse_position).write().unwrap();

            mouse_position.x = event.client_x() as i16;
            mouse_position.y = event.client_y() as i16;

            Engine::log(format!("Click x: {}", mouse_position.x));

            EngineInternal::update_state(EngineState {
                mouse_position: mouse_position.clone(),
                ..*engine::state()
            });
        }) as Box<dyn FnMut(_)>);

        element
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        mouse_manager
    }
}
