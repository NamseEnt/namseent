use super::*;

pub(crate) fn on_mouse_down_in(namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
    let input_element = get_input_element();
    let mut last_focused_text_input_id =
        TEXT_INPUT_SYSTEM.last_focused_text_input_id.lock().unwrap();

    let custom_data =
        find_front_text_input_on_mouse(&namui_context.rendering_tree, raw_mouse_event);

    *TEXT_INPUT_SYSTEM.dragging_text_input_id.lock().unwrap() = custom_data
        .as_ref()
        .map(|custom_data| custom_data.text_input.id.clone())
        .clone();

    if let Some(last_focused_text_input_id) = &*last_focused_text_input_id {
        let is_last_focused_text_input_not_clicked = custom_data
            .as_ref()
            .and_then(|custom_data| {
                last_focused_text_input_id
                    .eq(&custom_data.text_input.id)
                    .then(|| ())
            })
            .is_none();
        if is_last_focused_text_input_not_clicked {
            crate::event::send(text_input::Event::Blur(text_input::Blur {
                id: last_focused_text_input_id.clone(),
            }));
        }
    }

    *last_focused_text_input_id = custom_data
        .as_ref()
        .map(|custom_data| custom_data.text_input.id.clone());

    if custom_data.is_none() {
        input_element.blur().unwrap();
        return;
    }
    let custom_data = custom_data.unwrap();

    update_focus_with_mouse_movement(
        &custom_data,
        namui_context,
        input_element,
        raw_mouse_event.xy,
        false,
    );
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

    update_focus_with_mouse_movement(
        &custom_data,
        namui_context,
        get_input_element(),
        raw_mouse_event.xy,
        true,
    );
}
pub(crate) fn on_mouse_up_in() {
    *TEXT_INPUT_SYSTEM.dragging_text_input_id.lock().unwrap() = None;
}

fn update_focus_with_mouse_movement(
    custom_data: &TextInputCustomData,
    namui_context: &NamuiContext,
    input_element: HtmlTextAreaElement,
    mouse_xy: Xy<Px>,
    is_mouse_move: bool,
) {
    let local_xy = get_text_input_xy(&namui_context.rendering_tree, &custom_data.text_input.id)
        .unwrap()
        + Xy::new(custom_data.props.text_x(), custom_data.props.text_y());
    let mouse_local_xy = mouse_xy - local_xy;

    let selection = custom_data.text_input.get_selection_on_mouse_movement(
        &custom_data.props,
        mouse_local_xy,
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

    let event = text_input::Event::Focus(Focus {
        id: custom_data.text_input.id.clone(),
        selection,
    });
    crate::event::send(event);
}
