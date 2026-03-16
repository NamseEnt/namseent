use super::MOUSE_SYSTEM;
use crate::*;

// --- Primitive-type based functions (no winit dependency) ---

pub(crate) fn on_mouse_move(x: f32, y: f32) -> RawEvent {
    let mouse_xy = Xy::new(x.px(), y.px());
    update_mouse_position(mouse_xy);

    RawEvent::MouseMove {
        event: RawMouseEvent {
            xy: mouse_xy,
            pressing_buttons: get_pressing_buttons(),
            button: None,
        },
    }
}

pub(crate) fn on_mouse_input(is_down: bool, button: crate::MouseButton) -> RawEvent {
    update_pressing_button(is_down, button);

    let mouse_xy = { *MOUSE_SYSTEM.mouse_position.read().unwrap() };

    let event = RawMouseEvent {
        xy: mouse_xy,
        pressing_buttons: get_pressing_buttons(),
        button: Some(button),
    };

    if is_down {
        RawEvent::MouseDown { event }
    } else {
        RawEvent::MouseUp { event }
    }
}

pub(crate) fn on_mouse_wheel(delta_x: f32, delta_y: f32) -> RawEvent {
    let mouse_xy = { *MOUSE_SYSTEM.mouse_position.read().unwrap() };

    RawEvent::Wheel {
        event: RawWheelEvent {
            delta_xy: Xy::new(delta_x, delta_y),
            mouse_xy,
        },
    }
}

// --- Internal helpers ---

fn get_pressing_buttons() -> std::collections::HashSet<MouseButton> {
    MOUSE_SYSTEM.pressing_buttons.read().unwrap().clone()
}

fn update_mouse_position(mouse_xy: Xy<Px>) {
    *MOUSE_SYSTEM.mouse_position.write().unwrap() = mouse_xy;
    super::update_mouse_position_atomic(mouse_xy.x.as_f32() as u16, mouse_xy.y.as_f32() as u16);
}

fn update_pressing_button(is_down: bool, button: crate::MouseButton) {
    let mut pressing_buttons = MOUSE_SYSTEM.pressing_buttons.write().unwrap();
    if is_down {
        pressing_buttons.insert(button);
    } else {
        pressing_buttons.remove(&button);
    }
}
