use super::*;

macro_rules! on_mouse {
    ($extern_name: ident, $event: ident) => {
        pub extern "C" fn $extern_name(
            x: u32,
            y: u32,
            mouse_event_button: u16,
            mouse_event_buttons: u16,
        ) {
            let xy = Xy::new(px(x as f32), px(y as f32));
            update_mouse_position(xy);
            let button = get_button(mouse_event_button);
            let pressing_buttons = get_pressing_buttons(mouse_event_buttons);

            crate::hooks::on_raw_event(RawEvent::$event {
                event: RawMouseEvent {
                    xy,
                    pressing_buttons,
                    button: Some(button),
                },
            });
        }
    };
}

on_mouse!(on_mouse_down, MouseDown);
on_mouse!(on_mouse_move, MouseMove);
on_mouse!(on_mouse_up, MouseUp);

pub extern "C" fn on_mouse_wheel(delta_x: f32, delta_y: f32, x: u32, y: u32) {
    let xy = Xy::new(px(x as f32), px(y as f32));
    update_mouse_position(xy);

    crate::hooks::on_raw_event(RawEvent::Wheel {
        event: RawWheelEvent {
            delta_xy: Xy::new(delta_x, delta_y),
            mouse_xy: xy,
        },
    });
}

fn update_mouse_position(xy: Xy<Px>) {
    let mut mouse_position = MOUSE_SYSTEM.mouse_position.write().unwrap();

    *mouse_position = xy;
}

fn get_pressing_buttons(mouse_event_buttons: u16) -> HashSet<crate::MouseButton> {
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
fn get_button(mouse_event_button: u16) -> crate::MouseButton {
    const MOUSE_BUTTON_CONVERTING_TUPLES: [(u16, crate::MouseButton); 3] = [
        (0, crate::MouseButton::Left),
        (1, crate::MouseButton::Middle),
        (2, crate::MouseButton::Right),
    ];

    MOUSE_BUTTON_CONVERTING_TUPLES
        .iter()
        .find_map(|(value, button)| (mouse_event_button == *value).then_some(*button))
        .unwrap()
}
