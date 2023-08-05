use crate::*;
use serde::Deserialize;
use std::str::FromStr;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[derive(serde::Deserialize, Debug)]
pub enum WebEvent {
    #[serde(deserialize_with = "deserialize_raw_mouse_event")]
    MouseDown {
        event: RawMouseEvent,
    },
    #[serde(deserialize_with = "deserialize_raw_mouse_event")]
    MouseMove {
        event: RawMouseEvent,
    },
    #[serde(deserialize_with = "deserialize_raw_mouse_event")]
    MouseUp {
        event: RawMouseEvent,
    },
    #[serde(deserialize_with = "deserialize_raw_wheel_event")]
    Wheel {
        event: RawWheelEvent,
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
    TextInputKeyDown {
        #[serde(deserialize_with = "deserialize_code")]
        code: Code,
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
        #[serde(rename = "isComposing")]
        is_composing: bool,
    },
}

fn deserialize_code<'de, D>(deserializer: D) -> Result<Code, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Code::from_str(s.as_str()).map_err(|err| serde::de::Error::custom("fail to deserialize code"))
}

fn deserialize_selection_direction<'de, D>(deserializer: D) -> Result<SelectionDirection, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    SelectionDirection::try_from(s.as_str())
        .map_err(|err| serde::de::Error::custom("fail to deserialize selection direction"))
}

fn deserialize_raw_mouse_event<'de, D>(deserializer: D) -> Result<RawMouseEvent, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Input {
        x: f32,
        y: f32,
        button: u16,
        buttons: u16,
    }

    let input = Input::deserialize(deserializer)?;
    Ok(RawMouseEvent {
        xy: Xy::new(input.x.px(), input.y.px()),
        pressing_buttons: system::mouse::event::get_pressing_buttons(input.buttons),
        button: Some(system::mouse::event::get_button(input.button)),
    })
}

fn deserialize_raw_wheel_event<'de, D>(deserializer: D) -> Result<RawWheelEvent, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Input {
        x: f32,
        y: f32,
        delta_x: f32,
        delta_y: f32,
    }

    let input = Input::deserialize(deserializer)?;
    Ok(RawWheelEvent {
        delta_xy: Xy::new(input.delta_x, input.delta_y),
        mouse_xy: Xy::new(input.x.px(), input.y.px()),
    })
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

pub(crate) fn wait_web_event() -> WebEvent {
    loop {
        let event = waitEvent();
        let event: Option<WebEvent> =
            serde_wasm_bindgen::from_value(event).expect("failed to parse web event");
        if let Some(event) = event {
            return event;
        }
    }
}

pub(crate) fn handle_web_event(rendering_tree: Option<&RenderingTree>) -> WebEvent {
    let web_event = wait_web_event();
    match &web_event {
        // &WebEvent::MouseDown {
        //     x,
        //     y,
        //     button,
        //     buttons,
        // } => {
        //     // rendering_tree.map(|rendering_tree| {
        //     //     rendering_tree.call_mouse_event(
        //     //         MouseEventType::Down,
        //     //         &RawMouseEvent {
        //     //             id: uuid(),
        //     //             xy: Xy::new(px(x as f32), px(y as f32)),
        //     //             pressing_buttons: crate::system::mouse::event::get_pressing_buttons(
        //     //                 buttons as u16,
        //     //             ),
        //     //             button: Some(crate::system::mouse::event::get_button(button as u16)),
        //     //         },
        //     //     )
        //     // });
        // }
        // &WebEvent::MouseMove {
        //     x,
        //     y,
        //     button,
        //     buttons,
        // } => {
        //     // rendering_tree.map(|rendering_tree| {
        //     //     rendering_tree.call_mouse_event(
        //     //         MouseEventType::Move,
        //     //         &RawMouseEvent {
        //     //             id: uuid(),
        //     //             xy: Xy::new(px(x as f32), px(y as f32)),
        //     //             pressing_buttons: crate::system::mouse::event::get_pressing_buttons(
        //     //                 buttons as u16,
        //     //             ),
        //     //             button: Some(crate::system::mouse::event::get_button(button as u16)),
        //     //         },
        //     //     )
        //     // });
        // }
        // &WebEvent::MouseUp {
        //     x,
        //     y,
        //     button,
        //     buttons,
        // } => {
        //     // rendering_tree.map(|rendering_tree| {
        //     //     rendering_tree.call_mouse_event(
        //     //         MouseEventType::Up,
        //     //         &RawMouseEvent {
        //     //             id: uuid(),
        //     //             xy: Xy::new(px(x as f32), px(y as f32)),
        //     //             pressing_buttons: crate::system::mouse::event::get_pressing_buttons(
        //     //                 buttons as u16,
        //     //             ),
        //     //             button: Some(crate::system::mouse::event::get_button(button as u16)),
        //     //         },
        //     //     )
        //     // });
        // }
        // &WebEvent::Wheel {
        //     x,
        //     y,
        //     delta_x,
        //     delta_y,
        // } => {
        //     // rendering_tree.map(|rendering_tree| {
        //     //     rendering_tree.call_wheel_event(&RawWheelEvent {
        //     //         id: crate::uuid(),
        //     //         delta_xy: Xy {
        //     //             x: delta_x as f32,
        //     //             y: delta_y as f32,
        //     //         },
        //     //         mouse_xy: Xy::new(px(x as f32), px(y as f32)),
        //     //     })
        //     // });
        // }
        &WebEvent::HashChange { .. } => {}
        &WebEvent::SelectionChange { .. } => {}
        &WebEvent::KeyDown { ref code } => {} // crate::keyboard::on_key_down(&code),
        &WebEvent::KeyUp { ref code } => {}   //crate::keyboard::on_key_up(&code),
        &WebEvent::Blur => crate::keyboard::reset_pressing_code_set(),
        &WebEvent::VisibilityChange => crate::keyboard::reset_pressing_code_set(),
        &WebEvent::Resize { width, height } => {
            system::screen::resize(Wh::new(px(width as f32), px(height as f32)));
        }
        &WebEvent::AsyncFunction { id } => {
            crate::system::web::on_async_function_executed(id);
        }
        &WebEvent::TextInputTextUpdated { .. } => {}
        &WebEvent::TextInputKeyDown { .. } => {}
        _ => {}
    }

    web_event
}
