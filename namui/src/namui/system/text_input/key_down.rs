use super::*;
use crate::text::LineTexts;
use std::{
    ops::Range,
    sync::atomic::{AtomicBool, Ordering},
};

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

pub(crate) fn on_key_down(code: Code, event: web_sys::KeyboardEvent) {
    let input_element = get_input_element();
    let last_focused_text_input = TEXT_INPUT_SYSTEM.last_focused_text_input.lock().unwrap();

    if last_focused_text_input.is_none() {
        return;
    }
    let is_composing = event.is_composing();
    let last_focused_text_input = last_focused_text_input.as_ref().unwrap();

    last_focused_text_input
        .props
        .event_handler
        .as_ref()
        .map(|event_handler| {
            event_handler.on_key_down.as_ref().map(|on_key_down| {
                let is_prevented_default = Arc::new(AtomicBool::new(false));

                let key_down_event = KeyDownEvent {
                    code,
                    is_prevented_default: is_prevented_default.clone(),
                    is_composing,
                };
                on_key_down.invoke(key_down_event);

                if is_prevented_default.load(Ordering::Relaxed) {
                    event.prevent_default();
                }
            })
        });

    crate::event::send(text_input::Event::KeyDown {
        id: last_focused_text_input.id.clone(),
        code,
    });
    handle_selection_change(&last_focused_text_input, input_element, code);
}

fn get_line_texts(props: &text_input::Props) -> Option<LineTexts> {
    let font = crate::font::get_font(props.font_type)?;
    let fonts = crate::font::with_fallbacks(font);
    let paint = get_text_paint(props.style.text.color).build();
    Some(LineTexts::new(
        &props.text,
        fonts,
        paint.clone(),
        Some(props.rect.width()),
    ))
}

fn handle_selection_change(
    text_input: &TextInputCustomData,
    input_element: HtmlTextAreaElement,
    code: Code,
) {
    let key_in_interest = match code {
        Code::ArrowUp => KeyInInterest::ArrowUpDown(ArrowUpDown::Up),
        Code::ArrowDown => KeyInInterest::ArrowUpDown(ArrowUpDown::Down),
        Code::Home => KeyInInterest::HomeEnd(HomeEnd::Home),
        Code::End => KeyInInterest::HomeEnd(HomeEnd::End),
        _ => return,
    };

    let selection =
        get_selection_on_keyboard_down(&input_element, &text_input.props, key_in_interest);

    let Some(utf16_selection) = selection.as_utf16(input_element.value()) else {
        return;
    };

    let selection_direction = if utf16_selection.start <= utf16_selection.end {
        "forward"
    } else {
        "backward"
    };

    input_element
        .set_selection_range_with_direction(
            utf16_selection.start.min(utf16_selection.end) as u32,
            utf16_selection.start.max(utf16_selection.end) as u32,
            selection_direction,
        )
        .unwrap();
}

fn get_selection_on_keyboard_down(
    input_element: &HtmlTextAreaElement,
    props: &text_input::Props,
    key: KeyInInterest,
) -> Selection {
    let selection = super::get_input_element_selection(input_element);
    let Selection::Range(range) = selection else {
        return Selection::None;
    };

    let Some(line_texts) = get_line_texts(props) else {
        return Selection::None;
    };

    let next_selection_end = get_caret_index_after_apply_key_movement(key, line_texts, &range);

    let is_shift_key_pressed =
        crate::keyboard::any_code_press([crate::Code::ShiftLeft, crate::Code::ShiftRight]);
    let is_dragging = is_shift_key_pressed;

    match is_dragging {
        true => Selection::Range(range.start..next_selection_end),
        false => Selection::Range(next_selection_end..next_selection_end),
    }
}

fn get_caret_index_after_apply_key_movement(
    key: KeyInInterest,
    line_texts: LineTexts,
    selection: &Range<usize>,
) -> usize {
    let multiline_caret = line_texts.into_multiline_caret(selection.end);

    let caret_after_move = multiline_caret.get_caret_on_key(key);

    let next_selection_end = caret_after_move.to_selection_index();
    next_selection_end
}
