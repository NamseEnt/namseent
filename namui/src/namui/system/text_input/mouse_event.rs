use super::*;
use crate::text::*;
use std::ops::Range;

pub(crate) fn on_mouse_down_in_before_attach_event_calls() {
    text_input_system_mutate(|text_input_system| {
        text_input_system.dragging_text_input = None;
    });
}

pub(crate) fn on_mouse_down_in_after_attach_event_calls() {
    if text_input_system().dragging_text_input.is_none() {
        web::execute_function_sync(
            "
            textArea.blur();
        ",
        )
        .run::<()>();

        // text_input_system_mutate(|text_input_system| {
        //     text_input_system.last_focused_text_input = None;
        // });

        TEXT_INPUT_ATOM.mutate(|x| {
            x.last_focused_text_input = None;
        });

        // if let Some(last_focused_text_input) = last_focused_text_input.as_ref() {
        //     crate::event::send(text_input::Event::Blur {
        //         id: last_focused_text_input.id.clone(),
        //     });
        // }
    }
}

pub(crate) fn on_mouse_down_in_at_attach_event_calls(
    local_xy: Xy<Px>,
    custom_data: &TextInputCustomData,
) {
    text_input_system_mutate(|text_input_system| {
        text_input_system.dragging_text_input = Some(custom_data.clone());
        // text_input_system.last_focused_text_input = Some(custom_data.clone());
    });
    {
        let custom_data = custom_data.clone();
        TEXT_INPUT_ATOM.mutate(move |x| {
            x.last_focused_text_input = Some(custom_data.clone());
        });
    }

    // if let Some(last_focused_text_input) = &*last_focused_text_input {
    //     if last_focused_text_input.id.ne(&custom_data.id) {
    //         crate::event::send(text_input::Event::Blur {
    //             id: last_focused_text_input.id.clone(),
    //         });
    //     }
    // }

    update_focus_with_mouse_movement(&custom_data, local_xy, false);
}
pub(crate) fn on_mouse_move(rendering_tree: &RenderingTree, mouse_xy: Xy<Px>) {
    let text_input_system = text_input_system();
    let Some(dragging_text_input) = &text_input_system.dragging_text_input else {return};

    let Some(custom_data) = find_text_input_by_id(rendering_tree, dragging_text_input.id) else {return};

    let local_xy = get_text_input_xy(rendering_tree, custom_data.id).unwrap();
    let mouse_local_xy = mouse_xy - local_xy;

    update_focus_with_mouse_movement(&custom_data, mouse_local_xy, true);
}
pub(crate) fn on_mouse_up() {
    text_input_system_mutate(|text_input_system| {
        text_input_system.dragging_text_input = None;
    });
}

fn update_focus_with_mouse_movement(
    custom_data: &TextInputCustomData,
    local_mouse_xy: Xy<Px>,
    is_mouse_move: bool,
) {
    let local_text_xy =
        local_mouse_xy - Xy::new(custom_data.props.text_x(), custom_data.props.text_y());

    let selection =
        get_selection_on_mouse_movement(&custom_data.props, local_text_xy, is_mouse_move);

    let selection_direction = match &selection {
        Selection::Range(range) => {
            if range.start <= range.end {
                "forward"
            } else {
                "backward"
            }
        }
        Selection::None => "none",
    };

    // prev: let utf16_selection = selection.as_utf16(input_element.value());
    let utf16_selection = selection.as_utf16(&custom_data.props.text);
    let selection_start = utf16_selection
        .as_ref()
        .map_or(0, |selection| selection.start.min(selection.end) as u32);
    let selection_end = utf16_selection
        .as_ref()
        .map_or(0, |selection| selection.start.max(selection.end) as u32);

    let width = custom_data.props.rect.width().as_f32();

    web::execute_function_sync(
        "
        textArea.style.width = `${width}px`;
        textArea.value = text;
        textArea.setSelectionRange(selectionStart, selectionEnd, selectionDirection);
        textArea.focus();
    ",
    )
    .arg("width", width)
    .arg("text", &custom_data.props.text)
    .arg("selectionStart", selection_start)
    .arg("selectionEnd", selection_end)
    .arg("selectionDirection", selection_direction)
    .run::<()>();
}

fn get_selection_on_mouse_movement(
    props: &TextInput,
    click_local_xy: Xy<Px>,
    is_dragging_by_mouse: bool,
) -> Selection {
    let font = crate::font::get_font(props.font_type);

    if font.is_none() {
        return Selection::None;
    };
    let font = font.unwrap();
    let fonts = crate::font::with_fallbacks(font);

    let is_shift_key_pressed =
        crate::keyboard::any_code_press([crate::Code::ShiftLeft, crate::Code::ShiftRight]);

    let paint = get_text_paint(props.style.text.color).build();

    let line_texts = LineTexts::new(
        &props.text,
        fonts.clone(),
        paint.clone(),
        Some(props.rect.width()),
    );

    // const continouslyFastClickCount: number;

    // if (continouslyFastClickCount >= 3) {
    //   return getMoreTripleClickSelection({ text });
    // }
    // if (continouslyFastClickCount === 2) {
    //   return getDoubleClickSelection({ text, font, x: localX });
    // }

    let is_dragging = is_shift_key_pressed || is_dragging_by_mouse;

    let selection = super::get_input_element_selection_sync();

    Selection::Range(get_one_click_selection(
        props,
        &fonts,
        &line_texts,
        click_local_xy,
        is_dragging,
        &selection,
        paint,
    ))
}

fn get_one_click_selection(
    text_input_props: &TextInput,
    fonts: &Vec<Arc<Font>>,
    line_texts: &LineTexts,
    click_local_xy: Xy<Px>,
    is_dragging: bool,
    last_selection: &Selection,
    paint: Arc<Paint>,
) -> Range<usize> {
    let selection_index_of_xy =
        get_selection_index_of_xy(text_input_props, fonts, line_texts, click_local_xy, paint);

    let start = match last_selection {
        Selection::Range(range) => {
            if !is_dragging {
                selection_index_of_xy
            } else {
                range.start
            }
        }
        Selection::None => selection_index_of_xy,
    };

    start..selection_index_of_xy
}

fn get_selection_index_of_xy(
    text_input_props: &TextInput,
    fonts: &Vec<Arc<Font>>,
    line_texts: &LineTexts,
    click_local_xy: Xy<Px>,
    paint: Arc<Paint>,
) -> usize {
    let line_len = line_texts.line_len();
    if line_len == 0 {
        return 0;
    }

    let line_index = {
        let line_height = text_input_props.line_height_px();

        let top_y = click_local_xy.y
            + line_height
                * match text_input_props.text_baseline {
                    TextBaseline::Top => 0.0,
                    TextBaseline::Middle => line_len as f32 / 2.0,
                    TextBaseline::Bottom => line_len as f32,
                };

        let line_index = if top_y <= 0.px() {
            0
        } else {
            (top_y / line_height).floor() as usize
        };

        let line_max_index = line_len - 1;
        line_index.min(line_max_index)
    };

    let str_index_before_line = line_texts.char_index_before_line(line_index);

    let line_text = line_texts.iter_str().nth(line_index).unwrap();

    let glyph_widths = get_text_widths(&line_text, &fonts, paint);

    let line_width = glyph_widths.iter().sum::<Px>();

    let aligned_x = match text_input_props.text_align {
        TextAlign::Left => click_local_xy.x,
        TextAlign::Center => click_local_xy.x + line_width / 2.0,
        TextAlign::Right => click_local_xy.x + line_width,
    };

    let mut left = px(0.0);
    let index = glyph_widths
        .iter()
        .position(|width| {
            let center = left + width / 2.0;
            if aligned_x < center {
                return true;
            }
            left += *width;
            return false;
        })
        .unwrap_or(line_text.chars().count());

    str_index_before_line + index
}
