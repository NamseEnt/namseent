use super::MOUSE_SYSTEM;
use crate::*;
use winit::{dpi::PhysicalPosition, event::ElementState};

pub(crate) fn on_winit_mouse_input(state: ElementState, button: crate::MouseButton) -> RawEvent {
    update_pressing_button(state, button);

    let mouse_xy = { *MOUSE_SYSTEM.mouse_position.read().unwrap() };

    let event = RawMouseEvent {
        xy: mouse_xy,
        pressing_buttons: get_pressing_buttons(),
        button: Some(button),
    };

    match state {
        ElementState::Pressed => RawEvent::MouseDown { event },
        ElementState::Released => RawEvent::MouseUp { event },
    }
}

pub(crate) fn on_winit_mouse_wheel(delta: winit::event::MouseScrollDelta) -> RawEvent {
    let mouse_xy = { *MOUSE_SYSTEM.mouse_position.read().unwrap() };

    RawEvent::Wheel {
        event: RawWheelEvent {
            delta_xy: match delta {
                winit::event::MouseScrollDelta::LineDelta(x, y) => Xy::new(x, y),
                winit::event::MouseScrollDelta::PixelDelta(delta) => {
                    Xy::new(delta.x as f32, delta.y as f32)
                }
            },
            mouse_xy,
        },
    }
}

pub(crate) fn on_winit_cursor_moved(position: PhysicalPosition<f64>) -> RawEvent {
    let mouse_xy = Xy::new((position.x as f32).px(), (position.y as f32).px());
    update_mouse_position(mouse_xy);

    RawEvent::MouseMove {
        event: RawMouseEvent {
            xy: mouse_xy,
            pressing_buttons: get_pressing_buttons(),
            button: None,
        },
    }
}

fn get_pressing_buttons() -> std::collections::HashSet<MouseButton> {
    MOUSE_SYSTEM.pressing_buttons.read().unwrap().clone()
}

fn update_mouse_position(mouse_xy: Xy<Px>) {
    *MOUSE_SYSTEM.mouse_position.write().unwrap() = mouse_xy;
}

fn update_pressing_button(state: ElementState, button: crate::MouseButton) {
    let mut pressing_buttons = MOUSE_SYSTEM.pressing_buttons.write().unwrap();
    match state {
        ElementState::Pressed => {
            pressing_buttons.insert(button);
        }
        ElementState::Released => {
            pressing_buttons.remove(&button);
        }
    }
}
