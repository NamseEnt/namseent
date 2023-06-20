use super::*;

pub(crate) fn set_up_event_handler() {
    prevent_context_menu_open();

    let canvas_element = canvas_element();

    canvas_element
        .add_event_listener_with_callback(
            "mousedown",
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                event.prevent_default(); // NOTE: Text input needs this to prevent selection updates.
                update_mouse_position(&event);

                crate::system::text_input::on_mouse_down_in_before_attach_event_calls();

                let button = get_button(&event);
                let rendering_tree = render::last_rendering_tree();
                let mouse_position = { MOUSE_SYSTEM.mouse_position.read().unwrap().clone() };

                rendering_tree.call_mouse_event(
                    MouseEventType::Down,
                    &RawMouseEvent {
                        id: crate::uuid(),
                        xy: mouse_position,
                        pressing_buttons: get_pressing_buttons(&event),
                        button: Some(button),
                    },
                );

                crate::system::text_input::on_mouse_down_in_after_attach_event_calls();
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    canvas_element
        .add_event_listener_with_callback(
            "mousemove",
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                update_mouse_position(&event);
                let rendering_tree = render::last_rendering_tree();
                crate::system::text_input::on_mouse_move(&rendering_tree);

                let button = get_button(&event);
                let mouse_position = { MOUSE_SYSTEM.mouse_position.read().unwrap().clone() };

                rendering_tree.call_mouse_event(
                    MouseEventType::Move,
                    &RawMouseEvent {
                        id: crate::uuid(),
                        xy: mouse_position,
                        pressing_buttons: get_pressing_buttons(&event),
                        button: Some(button),
                    },
                )
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    canvas_element
        .add_event_listener_with_callback(
            "mouseup",
            Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
                update_mouse_position(&event);
                crate::system::text_input::on_mouse_up_in();

                let button = get_button(&event);
                let mouse_position = { MOUSE_SYSTEM.mouse_position.read().unwrap().clone() };
                let rendering_tree = render::last_rendering_tree();

                rendering_tree.call_mouse_event(
                    MouseEventType::Up,
                    &RawMouseEvent {
                        id: crate::uuid(),
                        xy: mouse_position,
                        pressing_buttons: get_pressing_buttons(&event),
                        button: Some(button),
                    },
                )
            }) as Box<dyn FnMut(_)>)
            .into_js_value()
            .unchecked_ref(),
        )
        .unwrap();

    canvas_element
        .add_event_listener_with_callback_and_add_event_listener_options(
            "wheel",
            Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
                update_mouse_position(&event);
                if event.ctrl_key() {
                    event.prevent_default()
                }
                let mouse_position = { MOUSE_SYSTEM.mouse_position.read().unwrap().clone() };
                let rendering_tree = render::last_rendering_tree();
                rendering_tree.call_wheel_event(&RawWheelEvent {
                    id: namui::uuid(),
                    delta_xy: Xy {
                        x: event.delta_x() as f32,
                        y: event.delta_y() as f32,
                    },
                    mouse_xy: mouse_position,
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
