use crate::{system, MouseEventType, RawMouseEvent, RawWheelEvent, RenderingTree};
use namui_type::{px, uuid, Wh, Xy};
use serde::Deserialize;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(serde::Deserialize, Debug, Clone)]
pub enum WebEvent {
    MouseDown {
        x: usize,
        y: usize,
        button: usize,
        buttons: usize,
    },
    MouseMove {
        x: usize,
        y: usize,
        button: usize,
        buttons: usize,
    },
    MouseUp {
        x: usize,
        y: usize,
        button: usize,
        buttons: usize,
    },
    Wheel {
        x: usize,
        y: usize,
        delta_x: isize,
        delta_y: isize,
    },
    HashChange {
        new_url: String,
        old_url: String,
    },
    // Drop {
    //     dataTransfer: Option<web_sys::DataTransfer>,
    //     x: usize,
    //     y: usize,
    // },
    SelectionChange {
        #[serde(
            rename = "selectionDirection",
            deserialize_with = "deserialize_selection_direction"
        )]
        selection_direction: SelectionDirection,
        #[serde(rename = "selectionStart")]
        selection_start: usize,
        #[serde(rename = "selectionEnd")]
        selection_end: usize,
        text: String,
    },
    KeyDown {
        code: String,
    },
    KeyUp {
        code: String,
    },
    Blur,
    VisibilityChange,
    Resize {
        width: usize,
        height: usize,
    },
    AsyncFunction {
        id: usize,
    },
    TextInputTextUpdated {
        text: String,
    },
    TextInputKeyDown {
        code: String,
        text: String,
        #[serde(
            rename = "selectionDirection",
            deserialize_with = "deserialize_selection_direction"
        )]
        selection_direction: SelectionDirection,
        #[serde(rename = "selectionStart")]
        selection_start: usize,
        #[serde(rename = "selectionEnd")]
        selection_end: usize,
    },
}

fn deserialize_selection_direction<'de, D>(deserializer: D) -> Result<SelectionDirection, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    SelectionDirection::try_from(s.as_str())
        .map_err(|err| serde::de::Error::custom("fail to deserialize selection direction"))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelectionDirection {
    Forward,
    Backward,
    None,
}

impl TryFrom<&str> for SelectionDirection {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "forward" => Ok(SelectionDirection::Forward),
            "backward" => Ok(SelectionDirection::Backward),
            "none" => Ok(SelectionDirection::None),
            _ => Err(()),
        }
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = globalThis)]
    fn waitEvent() -> JsValue;
}

fn wait_web_event() -> Option<WebEvent> {
    let event = waitEvent();
    let event: Option<WebEvent> =
        serde_wasm_bindgen::from_value(event).expect("failed to parse web event");
    event
}

pub fn handle_web_event(rendering_tree: Option<&RenderingTree>) -> Option<WebEvent> {
    let Some(web_event) = wait_web_event() else {
        return None;
    };
    match &web_event {
        &WebEvent::MouseDown {
            x,
            y,
            button,
            buttons,
        } => {
            rendering_tree.map(|rendering_tree| {
                rendering_tree.call_mouse_event(
                    MouseEventType::Down,
                    &RawMouseEvent {
                        id: uuid(),
                        xy: Xy::new(px(x as f32), px(y as f32)),
                        pressing_buttons: crate::system::mouse::event::get_pressing_buttons(
                            buttons as u16,
                        ),
                        button: Some(crate::system::mouse::event::get_button(button as u16)),
                    },
                )
            });
        }
        &WebEvent::MouseMove {
            x,
            y,
            button,
            buttons,
        } => {
            rendering_tree.map(|rendering_tree| {
                rendering_tree.call_mouse_event(
                    MouseEventType::Move,
                    &RawMouseEvent {
                        id: uuid(),
                        xy: Xy::new(px(x as f32), px(y as f32)),
                        pressing_buttons: crate::system::mouse::event::get_pressing_buttons(
                            buttons as u16,
                        ),
                        button: Some(crate::system::mouse::event::get_button(button as u16)),
                    },
                )
            });
        }
        &WebEvent::MouseUp {
            x,
            y,
            button,
            buttons,
        } => {
            rendering_tree.map(|rendering_tree| {
                rendering_tree.call_mouse_event(
                    MouseEventType::Up,
                    &RawMouseEvent {
                        id: uuid(),
                        xy: Xy::new(px(x as f32), px(y as f32)),
                        pressing_buttons: crate::system::mouse::event::get_pressing_buttons(
                            buttons as u16,
                        ),
                        button: Some(crate::system::mouse::event::get_button(button as u16)),
                    },
                )
            });
        }
        &WebEvent::Wheel {
            x,
            y,
            delta_x,
            delta_y,
        } => {
            rendering_tree.map(|rendering_tree| {
                rendering_tree.call_wheel_event(&RawWheelEvent {
                    id: crate::uuid(),
                    delta_xy: Xy {
                        x: delta_x as f32,
                        y: delta_y as f32,
                    },
                    mouse_xy: Xy::new(px(x as f32), px(y as f32)),
                })
            });
        }
        &WebEvent::HashChange { .. } => {}
        &WebEvent::SelectionChange {
            selection_direction,
            selection_start,
            selection_end,
            ref text,
        } => crate::system::text_input::on_selection_change(
            selection_direction,
            selection_start,
            selection_end,
            text,
        ),
        &WebEvent::KeyDown { ref code } => crate::keyboard::on_key_down(&code),
        &WebEvent::KeyUp { ref code } => crate::keyboard::on_key_up(&code),
        &WebEvent::Blur => crate::keyboard::reset_pressing_code_set(),
        &WebEvent::VisibilityChange => crate::keyboard::reset_pressing_code_set(),
        &WebEvent::Resize { width, height } => {
            system::screen::resize(Wh::new(px(width as f32), px(height as f32)));
        }
        &WebEvent::AsyncFunction { id } => {
            crate::system::web::on_async_function_executed(id);
        }
        &WebEvent::TextInputTextUpdated { ref text } => {}
        &WebEvent::TextInputKeyDown {
            ref code,
            ref text,
            ref selection_direction,
            selection_end,
            selection_start,
        } => {}
    }

    Some(web_event)
}
