use crate::namui::{self, namui_state::NamuiState, Namui, NamuiImpl, NamuiInternal, Xy};
use std::sync::{Arc, RwLock};
use wasm_bindgen::{prelude::Closure, JsCast};
use web_sys::HtmlElement;

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

            NamuiInternal::update_state(NamuiState {
                mouse_position: mouse_position.clone(),
                ..*namui::state()
            });

            namui::event::send(Box::new(namui::NamuiEvent::MoveClick(Xy {
                x: mouse_position.x as f32,
                y: mouse_position.y as f32,
            })));
        }) as Box<dyn FnMut(_)>);

        element
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())
            .unwrap();
        closure.forget();

        mouse_manager
    }
}
