use super::*;
use crate::text::LineTexts;
use std::{
    ops::Range,
    sync::atomic::{AtomicBool, Ordering},
};

pub struct TextInputKeyDownEvent {
    selection_start: usize,
    selection_end: usize,
    selection_direction: SelectionDirection,
    code: Code,
    is_composing: bool,
    text: String,
}

pub(crate) fn on_key_down(event: TextInputKeyDownEvent) {
    let atom = TEXT_INPUT_ATOM.get();
    let Some(last_focused_text_input) = &atom.last_focused_text_input else {return};

    let is_composing = event.is_composing;

    last_focused_text_input
        .props
        .event_handler
        .as_ref()
        .map(|event_handler| {
            event_handler.on_key_down.as_ref().map(|on_key_down| {
                let is_prevented_default = Arc::new(AtomicBool::new(false));

                let key_down_event = KeyDownEvent {
                    code: event.code,
                    is_prevented_default: is_prevented_default.clone(),
                    is_composing,
                };
                on_key_down.invoke(key_down_event);

                if is_prevented_default.load(Ordering::Relaxed) {
                    todo!()
                    // event.prevent_default();
                }
            })
        });

    // crate::event::send(text_input::Event::KeyDown {
    //     id: last_focused_text_input.id.clone(),
    //     code,
    // });
    handle_selection_change(&last_focused_text_input, event);
}

fn get_line_texts(props: &text_input::TextInput) -> Option<LineTexts> {
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

fn handle_selection_change(text_input: &TextInputCustomData, event: TextInputKeyDownEvent) {
    let key_in_interest = match event.code {
        Code::ArrowUp => KeyInInterest::ArrowUpDown(ArrowUpDown::Up),
        Code::ArrowDown => KeyInInterest::ArrowUpDown(ArrowUpDown::Down),
        Code::Home => KeyInInterest::HomeEnd(HomeEnd::Home),
        Code::End => KeyInInterest::HomeEnd(HomeEnd::End),
        _ => return,
    };

    let selection = get_selection_on_keyboard_down(&text_input.props, key_in_interest, &event);

    let Some(utf16_selection) = selection.as_utf16(&event.text) else {
        return;
    };

    let selection_direction = if utf16_selection.start <= utf16_selection.end {
        "forward"
    } else {
        "backward"
    };

    web::execute_function_sync(
        "
        textArea.setSelectionRange(
            selectionStart,
            selectionEnd,
            selectionDirection,
        )
    ",
    )
    .arg(
        "selectionStart",
        utf16_selection.start.min(utf16_selection.end) as u32,
    )
    .arg(
        "selectionEnd",
        utf16_selection.start.max(utf16_selection.end) as u32,
    )
    .arg("selectionDirection", selection_direction)
    .run::<()>();
}

fn get_selection_on_keyboard_down(
    props: &text_input::TextInput,
    key: KeyInInterest,
    event: &TextInputKeyDownEvent,
) -> Selection {
    let selection = super::get_input_element_selection(
        event.selection_direction,
        event.selection_start,
        event.selection_end,
        &event.text,
    );
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
