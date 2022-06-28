use super::*;
use crate::namui::{self, render::MouseCursor, Xy};
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use wasm_bindgen::{prelude::Closure, JsCast};

struct MouseSystem {
    mouse_position: Arc<RwLock<Xy<i16>>>,
}

lazy_static::lazy_static! {
    static ref MOUSE_SYSTEM: Arc<MouseSystem> = Arc::new(MouseSystem::new());
}

pub(crate) async fn init() -> InitResult {
    lazy_static::initialize(&MOUSE_SYSTEM);
    Ok(())
}

impl MouseSystem {
    fn new() -> Self {
        prevent_context_menu_open();

        let canvas_element = canvas_element();

        let mouse_position = Arc::new(RwLock::new(Xy::<i16> { x: 0, y: 0 }));
        let mouse = Self {
            mouse_position: mouse_position.clone(),
        };

        canvas_element
            .add_event_listener_with_callback(
                "mousedown",
                Closure::wrap({
                    let mouse_position = mouse_position.clone();
                    Box::new(move |event: web_sys::MouseEvent| {
                        event.prevent_default(); // NOTE: Text input needs this to prevent selection updates.
                        let mut mouse_position = mouse_position.write().unwrap();

                        mouse_position.x = event.client_x() as i16;
                        mouse_position.y = event.client_y() as i16;

                        let button = get_button(&event);
                        namui::event::send(namui::NamuiEvent::MouseDown(crate::RawMouseEvent {
                            id: format!(
                                "mousedown-{:?}-{:?}-{}",
                                button,
                                crate::now(),
                                crate::nanoid()
                            ),
                            xy: Xy {
                                x: mouse_position.x as f32,
                                y: mouse_position.y as f32,
                            },
                            pressing_buttons: get_pressing_buttons(&event),
                            button: Some(button),
                        }));
                    }) as Box<dyn FnMut(_)>
                })
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        canvas_element
            .add_event_listener_with_callback(
                "mouseup",
                Closure::wrap({
                    let mouse_position = mouse_position.clone();
                    Box::new(move |event: web_sys::MouseEvent| {
                        let mut mouse_position = mouse_position.write().unwrap();

                        mouse_position.x = event.client_x() as i16;
                        mouse_position.y = event.client_y() as i16;

                        let button = get_button(&event);
                        namui::event::send(namui::NamuiEvent::MouseUp(crate::RawMouseEvent {
                            id: format!(
                                "mouseup-{:?}-{:?}-{}",
                                button,
                                crate::now(),
                                crate::nanoid()
                            ),
                            xy: Xy {
                                x: mouse_position.x as f32,
                                y: mouse_position.y as f32,
                            },
                            pressing_buttons: get_pressing_buttons(&event),
                            button: Some(button),
                        }));
                    }) as Box<dyn FnMut(_)>
                })
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        canvas_element
            .add_event_listener_with_callback(
                "mousemove",
                Closure::wrap({
                    let mouse_position = mouse_position.clone();
                    Box::new(move |event: web_sys::MouseEvent| {
                        let mut mouse_position = mouse_position.write().unwrap();

                        mouse_position.x = event.client_x() as i16;
                        mouse_position.y = event.client_y() as i16;

                        namui::event::send(namui::NamuiEvent::MouseMove(crate::RawMouseEvent {
                            id: format!("mousemove-{:?}-{}", crate::now(), crate::nanoid()),
                            xy: Xy {
                                x: mouse_position.x as f32,
                                y: mouse_position.y as f32,
                            },
                            pressing_buttons: get_pressing_buttons(&event),
                            button: None,
                        }));
                    }) as Box<dyn FnMut(_)>
                })
                .into_js_value()
                .unchecked_ref(),
            )
            .unwrap();

        mouse
    }
}

pub fn set_mouse_cursor(cursor: &MouseCursor) {
    let element = document().body().unwrap();
    element
        .style()
        .set_property("cursor", &cursor.to_css_cursor_value())
        .unwrap();
}

pub fn mouse_position() -> Xy<i16> {
    MOUSE_SYSTEM.mouse_position.read().unwrap().clone()
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
            Self::Pointer => "pointer",
            MouseCursor::Custom(_) => "none",
        }
    }
}

fn get_pressing_buttons(mouse_event: &web_sys::MouseEvent) -> HashSet<crate::MouseButton> {
    let mouse_event_buttons = mouse_event.buttons();

    const MOUSE_BUTTONS_CONVERTING_TUPLES: [(u16, crate::MouseButton); 3] = [
        (1 << 0, crate::MouseButton::Left),
        (1 << 1, crate::MouseButton::Right),
        (1 << 2, crate::MouseButton::Middle),
    ];

    HashSet::from_iter(
        MOUSE_BUTTONS_CONVERTING_TUPLES
            .iter()
            .filter_map(|(bit, button)| {
                if mouse_event_buttons & bit != 0 {
                    Some(*button)
                } else {
                    None
                }
            }),
    )
}
fn get_button(mouse_event: &web_sys::MouseEvent) -> crate::MouseButton {
    let mouse_event_button = mouse_event.button() as u16;

    const MOUSE_BUTTON_CONVERTING_TUPLES: [(u16, crate::MouseButton); 3] = [
        (0, crate::MouseButton::Left),
        (1, crate::MouseButton::Middle),
        (2, crate::MouseButton::Right),
    ];

    MOUSE_BUTTON_CONVERTING_TUPLES
        .iter()
        .find_map(|(value, button)| (mouse_event_button == *value).then(|| *button))
        .unwrap()
}

fn prevent_context_menu_open() {
    document().set_oncontextmenu(Some(
        Closure::wrap({
            Box::new(move |event: web_sys::MouseEvent| {
                event.prevent_default();
            }) as Box<dyn FnMut(_)>
        })
        .into_js_value()
        .unchecked_ref(),
    ));
}
