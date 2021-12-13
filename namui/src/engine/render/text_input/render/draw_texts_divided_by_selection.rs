use crate::{
    engine::{self, get_text_width, RenderingTree, TextInput},
    render,
};

pub(crate) fn draw_texts_divided_by_selection(text_input: &TextInput) -> RenderingTree {
    let is_not_divided_by_selection = text_input
        .selection
        .map_or(true, |selection| selection.start == selection.end);

    if is_not_divided_by_selection {
        return engine::text(engine::TextParam {
            x: match text_input.text_align {
                engine::TextAlign::Left => 0.0,
                engine::TextAlign::Center => text_input.width / 2.0,
                engine::TextAlign::Right => text_input.width,
            },
            y: 0.0,
            align: text_input.text_align,
            baseline: text_input.text_baseline,
            text: text_input.text.clone(),
            font_type: text_input.font_type,
            style: text_input.text_style,
        });
    };

    let selection = text_input.selection.unwrap();

    let (left_selection_index, right_selection_index) = if selection.start < selection.end {
        (selection.start, selection.end)
    } else {
        (selection.end, selection.start)
    };

    let (left_text_string, selected_text_string, right_text_string) = (
        &text_input.text[..left_selection_index],
        &text_input.text[left_selection_index..right_selection_index],
        &text_input.text[right_selection_index..],
    );

    let result = get_text_xs(
        text_input,
        left_text_string,
        selected_text_string,
        right_text_string,
    );
    if result.is_none() {
        return RenderingTree::Empty;
    };

    let (left_text_x, selected_text_x, right_text_x) = result.unwrap();

    let left_text_text_param = engine::TextParam {
        x: left_text_x,
        y: 0.0,
        align: text_input.text_align,
        baseline: text_input.text_baseline,
        text: left_text_string.to_string(),
        font_type: text_input.font_type,
        style: text_input.text_style,
    };

    let selected_text_text_param = engine::TextParam {
        x: selected_text_x,
        text: selected_text_string.to_string(),
        style: engine::TextStyle {
            color: engine::Color::WHITE,
            ..left_text_text_param.style
        },
        ..left_text_text_param
    };
    let right_text_text_param = engine::TextParam {
        x: right_text_x,
        text: right_text_string.to_string(),
        ..left_text_text_param
    };

    let left_text = engine::text(left_text_text_param);
    let selected_text = engine::text(selected_text_text_param);
    let right_text = engine::text(right_text_text_param);

    return render![left_text, selected_text, right_text];
}

fn get_text_xs(
    text_input: &TextInput,
    left_text_string: &str,
    selected_text_string: &str,
    right_text_string: &str,
) -> Option<(f32, f32, f32)> {
    let font = engine::managers()
        .font_manager
        .get_font(&text_input.font_type);

    if font.is_none() {
        return None;
    }
    let font = font.unwrap();

    let drop_shadow_x = text_input.text_style.drop_shadow.map(|shadow| shadow.x);

    let (left_text_width, selected_text_width, right_text_width) = (
        get_text_width(&font, left_text_string, drop_shadow_x),
        get_text_width(&font, selected_text_string, drop_shadow_x),
        get_text_width(&font, right_text_string, drop_shadow_x),
    );

    match text_input.text_align {
        engine::TextAlign::Left => {
            Some((0.0, left_text_width, left_text_width + selected_text_width))
        }
        engine::TextAlign::Center => {
            let center = text_input.width / 2.0;
            let total_width = left_text_width + selected_text_width + right_text_width;

            Some((
                center - total_width / 2.0 + left_text_width / 2.0,
                center - total_width / 2.0 + left_text_width + selected_text_width / 2.0,
                center + total_width / 2.0 - right_text_width / 2.0,
            ))
        }
        engine::TextAlign::Right => Some((
            text_input.width,
            text_input.width - right_text_width,
            text_input.width - right_text_width - selected_text_width,
        )),
    }
}
