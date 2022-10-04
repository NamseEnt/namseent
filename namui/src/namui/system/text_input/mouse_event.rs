use super::*;
use crate::text::{get_fallback_fonts, get_line_height, LineTexts};
use std::ops::Range;

pub(crate) fn on_mouse_down_in_before_attach_event_calls() {
    *TEXT_INPUT_SYSTEM.dragging_text_input.lock().unwrap() = None;
}

pub(crate) fn on_mouse_down_in_after_attach_event_calls() {
    if TEXT_INPUT_SYSTEM
        .dragging_text_input
        .lock()
        .unwrap()
        .is_none()
    {
        let input_element = get_input_element();
        input_element.blur().unwrap();

        let mut last_focused_text_input = TEXT_INPUT_SYSTEM.last_focused_text_input.lock().unwrap();
        if let Some(last_focused_text_input) = last_focused_text_input.as_ref() {
            crate::event::send(text_input::Event::Blur {
                id: last_focused_text_input.id.clone(),
            });
        }
        *last_focused_text_input = None;
    }
}

pub(crate) fn on_mouse_down_in_at_attach_event_calls(
    local_xy: Xy<Px>,
    custom_data: &TextInputCustomData,
) {
    let input_element = get_input_element();
    let mut last_focused_text_input = TEXT_INPUT_SYSTEM.last_focused_text_input.lock().unwrap();

    *TEXT_INPUT_SYSTEM.dragging_text_input.lock().unwrap() = Some(custom_data.clone());

    if let Some(last_focused_text_input) = &*last_focused_text_input {
        if last_focused_text_input.id.ne(&custom_data.id) {
            crate::event::send(text_input::Event::Blur {
                id: last_focused_text_input.id.clone(),
            });
        }
    }

    *last_focused_text_input = Some(custom_data.clone());

    update_focus_with_mouse_movement(&custom_data, input_element, local_xy, false);
}
pub(crate) fn on_mouse_move(namui_context: &NamuiContext, raw_mouse_event: &RawMouseEvent) {
    let dragging_text_input = TEXT_INPUT_SYSTEM.dragging_text_input.lock().unwrap();
    if dragging_text_input.is_none() {
        return;
    }
    let dragging_text_input = dragging_text_input.as_ref().unwrap();

    let custom_data = find_text_input_by_id(&namui_context.rendering_tree, dragging_text_input.id);
    if custom_data.is_none() {
        return;
    }
    let custom_data = custom_data.unwrap();

    let local_xy = get_text_input_xy(&namui_context.rendering_tree, custom_data.id).unwrap();
    let mouse_local_xy = raw_mouse_event.xy - local_xy;

    update_focus_with_mouse_movement(&custom_data, get_input_element(), mouse_local_xy, true);
}
pub(crate) fn on_mouse_up_in() {
    *TEXT_INPUT_SYSTEM.dragging_text_input.lock().unwrap() = None;
}

fn update_focus_with_mouse_movement(
    custom_data: &TextInputCustomData,
    input_element: HtmlTextAreaElement,
    local_mouse_xy: Xy<Px>,
    is_mouse_move: bool,
) {
    let local_text_xy =
        local_mouse_xy - Xy::new(custom_data.props.text_x(), custom_data.props.text_y());

    let selection = get_selection_on_mouse_movement(
        &input_element,
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
        id: custom_data.id.clone(),
        selection,
    };
    crate::event::send(event);
}

fn get_selection_on_mouse_movement(
    input_element: &HtmlTextAreaElement,
    props: &Props,
    click_local_xy: Xy<Px>,
    is_dragging_by_mouse: bool,
) -> Option<Range<usize>> {
    let font = crate::font::get_font(props.font_type);

    if font.is_none() {
        return None;
    };
    let font = font.unwrap();

    let is_shift_key_pressed =
        crate::keyboard::any_code_press([crate::Code::ShiftLeft, crate::Code::ShiftRight]);

    let fonts = std::iter::once(font.clone())
        .chain(std::iter::once_with(|| get_fallback_fonts(font.size)).flatten())
        .collect::<Vec<_>>();

    let paint = get_text_paint(props.text_style.color).build();

    let line_texts = LineTexts::new(&props.text, &fonts, &paint, Some(props.rect.width()));

    // const continouslyFastClickCount: number;

    // if (continouslyFastClickCount >= 3) {
    //   return getMoreTripleClickSelection({ text });
    // }
    // if (continouslyFastClickCount === 2) {
    //   return getDoubleClickSelection({ text, font, x: localX });
    // }

    let is_dragging = is_shift_key_pressed || is_dragging_by_mouse;

    let selection = super::get_input_element_selection(input_element);

    Some(get_one_click_selection(
        props.text_align,
        props.text_baseline,
        &font,
        &line_texts,
        click_local_xy,
        is_dragging,
        &selection,
    ))
}

fn get_one_click_selection(
    text_align: TextAlign,
    text_baseline: TextBaseline,
    font: &Font,
    line_texts: &LineTexts,
    click_local_xy: Xy<Px>,
    is_dragging: bool,
    last_selection: &Option<Range<usize>>,
) -> Range<usize> {
    let selection_index_of_xy =
        get_selection_index_of_xy(text_align, text_baseline, font, line_texts, click_local_xy);

    let start = match last_selection {
        Some(last_selection) => {
            if !is_dragging {
                selection_index_of_xy
            } else {
                last_selection.start
            }
        }
        None => selection_index_of_xy,
    };

    start..selection_index_of_xy
}

fn get_selection_index_of_xy(
    text_align: TextAlign,
    text_baseline: TextBaseline,
    font: &Font,
    line_texts: &LineTexts,
    click_local_xy: Xy<Px>,
) -> usize {
    let line_len = line_texts.line_len();
    if line_len == 0 {
        return 0;
    }

    let line_index = {
        let line_height = get_line_height(font.size);

        let top_y = click_local_xy.y
            + line_height
                * match text_baseline {
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

    let glyph_ids = font.get_glyph_ids(&line_text);
    let glyph_widths = font.get_glyph_widths(glyph_ids, None);

    let line_width = glyph_widths.iter().sum::<Px>();

    let aligned_x = match text_align {
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
