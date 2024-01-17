use super::*;

pub(crate) fn set_up_event_handler() {
    prevent_context_menu_open();

    let document = document();

    document
        .add_event_listener_with_callback(
            "mousedown",
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                event.prevent_default(); // NOTE: Text input needs this to prevent selection updates.
                update_mouse_position(&event);

                let button = get_button(&event);
                let mouse_position = { *MOUSE_SYSTEM.mouse_position.read().unwrap() };

                crate::hooks::on_raw_event(RawEvent::MouseDown {
                    event: RawMouseEvent {
                        xy: mouse_position,
                        pressing_buttons: get_pressing_buttons(&event),
                        button: Some(button),
                        prevent_default: Box::new(move || {
                            event.prevent_default();
                        }),
                    },
                });
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    document
        .add_event_listener_with_callback(
            "mousemove",
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                update_mouse_position(&event);

                let button = get_button(&event);
                let mouse_position = { *MOUSE_SYSTEM.mouse_position.read().unwrap() };

                crate::hooks::on_raw_event(RawEvent::MouseMove {
                    event: RawMouseEvent {
                        xy: mouse_position,
                        pressing_buttons: get_pressing_buttons(&event),
                        button: Some(button),
                        prevent_default: Box::new(move || {
                            event.prevent_default();
                        }),
                    },
                });
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    document
        .add_event_listener_with_callback(
            "mouseup",
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                update_mouse_position(&event);

                let button = get_button(&event);
                let mouse_position = { *MOUSE_SYSTEM.mouse_position.read().unwrap() };

                crate::hooks::on_raw_event(RawEvent::MouseUp {
                    event: RawMouseEvent {
                        xy: mouse_position,
                        pressing_buttons: get_pressing_buttons(&event),
                        button: Some(button),
                        prevent_default: Box::new(move || {
                            event.prevent_default();
                        }),
                    },
                });
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    document
        .add_event_listener_with_callback_and_add_event_listener_options(
            "wheel",
            Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
                update_mouse_position(&event);

                if event.ctrl_key() {
                    event.prevent_default()
                }

                let mouse_position = { *MOUSE_SYSTEM.mouse_position.read().unwrap() };
                crate::hooks::on_raw_event(RawEvent::Wheel {
                    event: RawWheelEvent {
                        delta_xy: Xy {
                            x: event.delta_x() as f32,
                            y: event.delta_y() as f32,
                        },
                        mouse_xy: mouse_position,
                    },
                });
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
            web_sys::AddEventListenerOptions::new().passive(false),
        )
        .unwrap();
}

fn update_mouse_position(event: &web_sys::MouseEvent) {
    let mut mouse_position = MOUSE_SYSTEM.mouse_position.write().unwrap();

    mouse_position.x = px(event.client_x() as f32);
    mouse_position.y = px(event.client_y() as f32);
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
        .find_map(|(value, button)| (mouse_event_button == *value).then_some(*button))
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
