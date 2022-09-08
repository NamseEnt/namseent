use super::*;

pub enum ArrowUpDown {
    Up,
    Down,
}

pub enum HomeEnd {
    Home,
    End,
}

pub enum KeyInInterest {
    ArrowUpDown(ArrowUpDown),
    HomeEnd(HomeEnd),
}

pub(crate) fn on_key_down(namui_context: &NamuiContext, raw_keyboard_event: &RawKeyboardEvent) {
    let key_in_interest = match raw_keyboard_event.code {
        Code::ArrowUp => KeyInInterest::ArrowUpDown(ArrowUpDown::Up),
        Code::ArrowDown => KeyInInterest::ArrowUpDown(ArrowUpDown::Down),
        Code::Home => KeyInInterest::HomeEnd(HomeEnd::Home),
        Code::End => KeyInInterest::HomeEnd(HomeEnd::End),
        _ => return,
    };

    let input_element = get_input_element();
    let last_focused_text_input_id = TEXT_INPUT_SYSTEM.last_focused_text_input_id.lock().unwrap();

    if last_focused_text_input_id.is_none() {
        return;
    }
    let last_focused_text_input_id = last_focused_text_input_id.as_ref().unwrap();

    let custom_data =
        find_text_input_by_id(&namui_context.rendering_tree, last_focused_text_input_id);

    if custom_data.is_none() {
        return;
    }
    let custom_data = custom_data.unwrap();

    let selection = custom_data
        .text_input
        .get_selection_on_keyboard_down(&custom_data.props, key_in_interest);
    if selection.is_none() {
        return;
    }
    let selection = selection.unwrap();

    let selection_direction = if selection.start <= selection.end {
        "forward"
    } else {
        "backward"
    };

    input_element
        .set_selection_range_with_direction(
            selection.start.min(selection.end) as u32,
            selection.start.max(selection.end) as u32,
            selection_direction,
        )
        .unwrap();
}
