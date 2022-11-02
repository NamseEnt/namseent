use super::*;
use crate::text::{get_fallback_fonts, LineTexts};
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
                on_key_down(key_down_event);

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

fn get_selection_on_keyboard_down(
    input_element: &HtmlTextAreaElement,
    props: &text_input::Props,
    key: KeyInInterest,
) -> text_input::Selection {
    let selection = super::get_input_element_selection(input_element);
    if selection.is_none() {
        return None;
    }
    let selection = selection.as_ref().unwrap();

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

    let paint = get_text_paint(props.style.text.color).build();

    let line_texts = LineTexts::new(&props.text, &fonts, &paint, Some(props.rect.width()));

    let is_dragging = is_shift_key_pressed;

    let next_selection_end = get_caret_index_after_apply_key_movement(key, line_texts, selection);

    match is_dragging {
        true => Some(selection.start..next_selection_end),
        false => Some(next_selection_end..next_selection_end),
    }
}

fn get_caret_index_after_apply_key_movement(
    key: KeyInInterest,
    line_texts: LineTexts,
    selection: &Range<usize>,
) -> usize {
    let multiline_caret = line_texts.get_multiline_caret(selection.end);

    let caret_after_move = multiline_caret.get_caret_on_key(key);

    let next_selection_end = caret_after_move.to_selection_index();
    next_selection_end
}
