use super::*;

pub(crate) fn post_render(root_rendering_tree: &RenderingTree) {
    update_focused_text_input(root_rendering_tree);
    update_input_element_text();
}

fn update_focused_text_input(root_rendering_tree: &RenderingTree) {
    let last_focused_text_input_id = TEXT_INPUT_ATOM
        .get()
        .last_focused_text_input
        .as_ref()
        .map(|text_input| text_input.id.clone());

    let next_focused_text_input_id = text_input_system_mutate(|text_input_system| {
        text_input_system
            .focus_requested_text_input_id
            .take()
            .or_else(|| last_focused_text_input_id)
    });

    if next_focused_text_input_id.is_none() {
        return;
    }
    let next_focused_text_input_id = next_focused_text_input_id.unwrap();

    let custom_data = find_text_input_by_id(root_rendering_tree, next_focused_text_input_id);

    // let is_focused_text_input_id_changed =
    //     last_focused_text_input_id != Some(next_focused_text_input_id);
    // if is_focused_text_input_id_changed {
    //     if let Some(last_focused_text_input_id) = last_focused_text_input_id {
    //         crate::event::send(text_input::Event::Blur {
    //             id: last_focused_text_input_id,
    //         });
    //     }

    //     let next_focused_text_input_exists = custom_data.is_some();
    //     if next_focused_text_input_exists {
    //         crate::event::send(text_input::Event::Focus {
    //             id: next_focused_text_input_id,
    //         });
    //     }
    // }

    // text_input_system_mutate(|text_input_system| {
    //     text_input_system.last_focused_text_input = custom_data;
    // });

    TEXT_INPUT_ATOM.mutate(|x| {
        x.last_focused_text_input = custom_data;
    });
}

fn update_input_element_text() {
    let atom = TEXT_INPUT_ATOM.get();
    let Some(last_focused_text_input) = atom
        .last_focused_text_input
        .as_ref() else {
            return;
        };

    let text: String = web::execute_function_sync(
        "
        return textArea.value;
    ",
    )
    .run();

    if text != last_focused_text_input.props.text {
        web::execute_function_sync(
            "
            textArea.value = text;
        ",
        )
        .arg(text, &last_focused_text_input.props.text)
        .run::<()>();
    }
}
