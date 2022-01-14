use crate::namui::{self, namui_state::NamuiState, render::MouseCursor, NamuiInternal, Xy};
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

        let mouse_down_mouse_position = mouse_position.clone();
        let mouse_down_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut mouse_position = mouse_down_mouse_position.write().unwrap();

            mouse_position.x = event.client_x() as i16;
            mouse_position.y = event.client_y() as i16;

            NamuiInternal::update_state(NamuiState {
                mouse_position: mouse_position.clone(),
                ..*namui::state()
            });

            namui::event::send(namui::NamuiEvent::MouseDown(Xy {
                x: mouse_position.x as f32,
                y: mouse_position.y as f32,
            }));
        }) as Box<dyn FnMut(_)>);

        element
            .add_event_listener_with_callback(
                "mousedown",
                mouse_down_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        mouse_down_closure.forget();

        let mouse_up_mouse_position = mouse_position.clone();
        let mouse_up_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut mouse_position = mouse_up_mouse_position.write().unwrap();

            mouse_position.x = event.client_x() as i16;
            mouse_position.y = event.client_y() as i16;

            NamuiInternal::update_state(NamuiState {
                mouse_position: mouse_position.clone(),
                ..*namui::state()
            });

            namui::event::send(namui::NamuiEvent::MouseUp(Xy {
                x: mouse_position.x as f32,
                y: mouse_position.y as f32,
            }));
        }) as Box<dyn FnMut(_)>);

        element
            .add_event_listener_with_callback("mouseup", mouse_up_closure.as_ref().unchecked_ref())
            .unwrap();
        mouse_up_closure.forget();

        let mouse_move_mouse_position = mouse_position.clone();
        let mouse_move_closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let mut mouse_position = (*mouse_move_mouse_position).write().unwrap();

            mouse_position.x = event.client_x() as i16;
            mouse_position.y = event.client_y() as i16;

            NamuiInternal::update_state(NamuiState {
                mouse_position: mouse_position.clone(),
                ..*namui::state()
            });

            namui::event::send(namui::NamuiEvent::MouseMove(Xy {
                x: mouse_position.x as f32,
                y: mouse_position.y as f32,
            }));
        }) as Box<dyn FnMut(_)>);

        element
            .add_event_listener_with_callback(
                "mousemove",
                mouse_move_closure.as_ref().unchecked_ref(),
            )
            .unwrap();
        mouse_move_closure.forget();

        mouse_manager
    }
    pub fn set_mouse_cursor(&self, cursor: MouseCursor) {
        let element = namui::window().document().unwrap().body().unwrap();
        element
            .style()
            .set_property("cursor", &cursor.to_css_cursor_value())
            .unwrap();
    }
}
impl MouseCursor {
    pub fn to_css_cursor_value(&self) -> &str {
        match self {
            Self::Default => "default",
            Self::TopBottomResize => "ns-resize",
            Self::LeftRightResize => "ew-resize",
            Self::LeftTopRightBottomResize => "nwse-resize",
            Self::RightTopLeftBottomResize => "nesw-resize",
            Self::Text => "text",
            Self::Grab => "grab",
            Self::Move => "move",
        }
    }
}
