use super::InitResult;
use crate::*;
use std::collections::HashSet;
use std::sync::atomic::AtomicU32;

pub(crate) fn init() -> InitResult {
    Ok(())
}

pub fn set_mouse_cursor(_cursor: &MouseCursor) {
    todo!()
}

pub fn position() -> Xy<Px> {
    let mouse_position = MOUSE_POSITION.load(std::sync::atomic::Ordering::SeqCst);
    Xy::new(
        px((mouse_position >> 16) as f32),
        px((mouse_position & 0xffff) as f32),
    )
}

fn update_mouse_position(x: u16, y: u16) {
    MOUSE_POSITION.store(
        (x as u32) << 16 | y as u32,
        std::sync::atomic::Ordering::SeqCst,
    );
}

pub(crate) fn mouse_position_u32() -> u32 {
    MOUSE_POSITION.load(std::sync::atomic::Ordering::SeqCst)
}

/// 16 bit x, 16 bit y
static MOUSE_POSITION: AtomicU32 = AtomicU32::new(0);

macro_rules! on_mouse {
    ($extern_name: ident, $event: ident) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn $extern_name(
            x: u16,
            y: u16,
            mouse_event_button: u8,
            mouse_event_buttons: u8,
        ) -> u64 {
            update_mouse_position(x, y);
            let button = get_button(mouse_event_button);
            let pressing_buttons = get_pressing_buttons(mouse_event_buttons);

            crate::on_event(RawEvent::$event {
                event: RawMouseEvent {
                    xy: Xy::new(px(x as f32), px(y as f32)),
                    pressing_buttons,
                    button: Some(button),
                },
            })
        }
    };
}

on_mouse!(_on_mouse_down, MouseDown);
on_mouse!(_on_mouse_move, MouseMove);
on_mouse!(_on_mouse_up, MouseUp);

#[unsafe(no_mangle)]
pub extern "C" fn _on_mouse_wheel(delta_x: f32, delta_y: f32, x: u16, y: u16) -> u64 {
    let xy = Xy::new(px(x as f32), px(y as f32));
    update_mouse_position(x, y);

    crate::on_event(RawEvent::Wheel {
        event: RawWheelEvent {
            delta_xy: Xy::new(delta_x, delta_y),
            mouse_xy: xy,
        },
    })
}

fn get_pressing_buttons(mouse_event_buttons: u8) -> HashSet<crate::MouseButton> {
    const MOUSE_BUTTONS_CONVERTING_TUPLES: [(u8, crate::MouseButton); 3] = [
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
fn get_button(mouse_event_button: u8) -> crate::MouseButton {
    const MOUSE_BUTTON_CONVERTING_TUPLES: [(u8, crate::MouseButton); 3] = [
        (0, crate::MouseButton::Left),
        (1, crate::MouseButton::Middle),
        (2, crate::MouseButton::Right),
    ];

    MOUSE_BUTTON_CONVERTING_TUPLES
        .iter()
        .find_map(|(value, button)| (mouse_event_button == *value).then_some(*button))
        .unwrap()
}
