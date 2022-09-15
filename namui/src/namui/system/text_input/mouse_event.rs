use super::*;

pub(crate) fn on_mouse_down_in_before_attach_event_calls() {
    *TEXT_INPUT_SYSTEM.dragging_text_input_id.lock().unwrap() = None;
}

pub(crate) fn on_mouse_down_in_after_attach_event_calls() {
    if TEXT_INPUT_SYSTEM
        .dragging_text_input_id
        .lock()
        .unwrap()
        .is_none()
    {
        let input_element = get_input_element();
        input_element.blur().unwrap();

        let mut last_focused_text_input_id =
            TEXT_INPUT_SYSTEM.last_focused_text_input_id.lock().unwrap();
        if let Some(id) = last_focused_text_input_id.as_ref() {
            crate::event::send(text_input::Event::Blur { id: id.clone() });
        }
        *last_focused_text_input_id = None;
    }
}

pub(crate) fn on_mouse_down_in_at_attach_event_calls(
    local_xy: Xy<Px>,
    custom_data: &TextInputCustomData,
) {
    let input_element = get_input_element();
    let mut last_focused_text_input_id =
        TEXT_INPUT_SYSTEM.last_focused_text_input_id.lock().unwrap();

    *TEXT_INPUT_SYSTEM.dragging_text_input_id.lock().unwrap() =
        Some(custom_data.text_input.id.clone());

    if let Some(last_focused_text_input_id) = &*last_focused_text_input_id {
        if last_focused_text_input_id.ne(&custom_data.text_input.id) {
            crate::event::send(text_input::Event::Blur {
                id: last_focused_text_input_id.clone(),
            });
        }
    }

    *last_focused_text_input_id = Some(custom_data.text_input.id.clone());

    update_focus_with_mouse_movement(&custom_data, input_element, local_xy, false);
}
pub(crate) fn on_mouse_move(namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
    let dragging_text_input_id = TEXT_INPUT_SYSTEM.dragging_text_input_id.lock().unwrap();
    if dragging_text_input_id.is_none() {
        return;
    }
    let dragging_text_input_id = dragging_text_input_id.as_ref().unwrap();

    let custom_data = find_text_input_by_id(&namui_context.rendering_tree, dragging_text_input_id);
    if custom_data.is_none() {
        return;
    }
    let custom_data = custom_data.unwrap();

    let local_xy =
        get_text_input_xy(&namui_context.rendering_tree, &custom_data.text_input.id).unwrap();
    let mouse_local_xy = raw_mouse_event.xy - local_xy;

    update_focus_with_mouse_movement(&custom_data, get_input_element(), mouse_local_xy, true);
}
pub(crate) fn on_mouse_up_in() {
    *TEXT_INPUT_SYSTEM.dragging_text_input_id.lock().unwrap() = None;
}

fn update_focus_with_mouse_movement(
    custom_data: &TextInputCustomData,
    input_element: HtmlTextAreaElement,
    local_mouse_xy: Xy<Px>,
    is_mouse_move: bool,
) {
    let local_text_xy =
        local_mouse_xy - Xy::new(custom_data.props.text_x(), custom_data.props.text_y());

    let selection = custom_data.text_input.get_selection_on_mouse_movement(
        &custom_data.props,
        local_text_xy,
        is_mouse_move,
    );

    let selection_direction = match &selection {
        Some(selection) => {
            if selection.start <= selection.end {
                "forward"
            } else {
                "backward"
            }
        }
        None => "none",
    };

    let width = custom_data.props.rect.width().as_f32();
    input_element
        .style()
        .set_property("width", &format!("{width}px"))
        .unwrap();

    input_element.set_value(&custom_data.props.text);
    input_element
        .set_selection_range_with_direction(
            selection
                .as_ref()
                .map_or(0, |selection| selection.start.min(selection.end) as u32),
            selection
                .as_ref()
                .map_or(0, |selection| selection.start.max(selection.end) as u32),
            selection_direction,
        )
        .unwrap();

    input_element.focus().unwrap();

    let event = text_input::Event::Focus {
        id: custom_data.text_input.id.clone(),
        selection,
    };
    crate::event::send(event);
}
