use super::*;

pub(crate) fn post_render(root_rendering_tree: &RenderingTree) {
    update_focused_text_input(root_rendering_tree);
    update_input_element_text();
}

fn update_focused_text_input(root_rendering_tree: &RenderingTree) {
    let next_focused_text_input_id = {
        TEXT_INPUT_SYSTEM
            .focus_requested_text_input_id
            .lock()
            .unwrap()
            .take()
            .or_else(|| {
                TEXT_INPUT_SYSTEM
                    .last_focused_text_input
                    .lock()
                    .unwrap()
                    .as_ref()
                    .map(|text_input| text_input.id.clone())
            })
    };

    if next_focused_text_input_id.is_none() {
        return;
    }
    let next_focused_text_input_id = next_focused_text_input_id.unwrap();

    let custom_data = find_text_input_by_id(root_rendering_tree, &next_focused_text_input_id);

    *TEXT_INPUT_SYSTEM.last_focused_text_input.lock().unwrap() = custom_data;
}

fn update_input_element_text() {
    TEXT_INPUT_SYSTEM
        .last_focused_text_input
        .lock()
        .unwrap()
        .as_ref()
        .map(|last_focused_text_input| {
            let input_element = get_input_element();
            let text = input_element.value();
            if text != last_focused_text_input.props.text {
                input_element.set_value(&last_focused_text_input.props.text);
            }
        });
}
