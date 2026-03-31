use super::InitResult;
use crate::*;
use std::collections::HashSet;
use std::sync::atomic::AtomicU32;

pub(crate) fn init() -> InitResult {
    Ok(())
}


// --- Shared position tracking ---

/// 16 bit x, 16 bit y
static MOUSE_POSITION: AtomicU32 = AtomicU32::new(0);

pub fn position() -> Xy<Px> {
    let mouse_position = MOUSE_POSITION.load(std::sync::atomic::Ordering::SeqCst);
    Xy::new(
        px((mouse_position >> 16) as f32),
        px((mouse_position & 0xffff) as f32),
    )
}

pub(crate) fn mouse_position_u32() -> u32 {
    MOUSE_POSITION.load(std::sync::atomic::Ordering::SeqCst)
}

fn update_mouse_position(x: f32, y: f32) {
    MOUSE_POSITION.store(
        (x as u16 as u32) << 16 | y as u16 as u32,
        std::sync::atomic::Ordering::SeqCst,
    );
}

// --- Unified mouse event helpers ---

pub(crate) fn on_mouse_down(x: f32, y: f32, button: u8, buttons: u8) -> RawEvent {
    update_mouse_position(x, y);
    RawEvent::MouseDown {
        event: RawMouseEvent {
            xy: Xy::new(px(x), px(y)),
            pressing_buttons: buttons_from_bitmask(buttons),
            button: Some(button_from_u8(button)),
        },
    }
}

pub(crate) fn on_mouse_move(x: f32, y: f32, buttons: u8) -> RawEvent {
    update_mouse_position(x, y);
    RawEvent::MouseMove {
        event: RawMouseEvent {
            xy: Xy::new(px(x), px(y)),
            pressing_buttons: buttons_from_bitmask(buttons),
            button: None,
        },
    }
}

pub(crate) fn on_mouse_up(x: f32, y: f32, button: u8, buttons: u8) -> RawEvent {
    update_mouse_position(x, y);
    RawEvent::MouseUp {
        event: RawMouseEvent {
            xy: Xy::new(px(x), px(y)),
            pressing_buttons: buttons_from_bitmask(buttons),
            button: Some(button_from_u8(button)),
        },
    }
}

pub(crate) fn on_mouse_wheel(delta_x: f32, delta_y: f32, x: f32, y: f32) -> RawEvent {
    update_mouse_position(x, y);
    RawEvent::Wheel {
        event: RawWheelEvent {
            delta_xy: Xy::new(delta_x, delta_y),
            mouse_xy: Xy::new(px(x), px(y)),
        },
    }
}

/// Convert DOM MouseEvent.button to MouseButton.
/// 0=Left, 1=Middle, 2=Right
fn button_from_u8(button: u8) -> MouseButton {
    match button {
        0 => MouseButton::Left,
        1 => MouseButton::Middle,
        2 => MouseButton::Right,
        _ => MouseButton::Left,
    }
}

/// Convert DOM MouseEvent.buttons bitmask to HashSet<MouseButton>.
/// bit0=Left, bit1=Right, bit2=Middle
fn buttons_from_bitmask(buttons: u8) -> HashSet<MouseButton> {
    let mut set = HashSet::new();
    if buttons & (1 << 0) != 0 {
        set.insert(MouseButton::Left);
    }
    if buttons & (1 << 1) != 0 {
        set.insert(MouseButton::Right);
    }
    if buttons & (1 << 2) != 0 {
        set.insert(MouseButton::Middle);
    }
    set
}
