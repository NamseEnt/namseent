use super::Props;
use crate::{system::text_input::KeyInInterest, text::*, *};
use std::ops::Range;

impl TextInput {
    pub(crate) fn get_selection_on_keyboard_down(
        &self,
        props: &Props,
        key: KeyInInterest,
    ) -> Option<Range<usize>> {
        if self.selection.is_none() {
            return None;
        }
        let selection = self.selection.as_ref().unwrap();

        let font = crate::font::get_font(props.font_type);
        if font.is_none() {
            return None;
        };
        let font = font.unwrap();

        let is_shift_key_pressed =
            crate::keyboard::any_code_press([namui::Code::ShiftLeft, namui::Code::ShiftRight]);

        let fonts = std::iter::once(font.clone())
            .chain(std::iter::once_with(|| get_fallback_fonts(font.size)).flatten())
            .collect::<Vec<_>>();

        let paint = get_text_paint(props.text_style.color).build();

        let line_texts = LineTexts::new(&props.text, &fonts, &paint, Some(props.rect.width()));

        let is_dragging = is_shift_key_pressed;

        let next_selection_end =
            get_caret_index_after_apply_key_movement(key, line_texts, selection);

        match is_dragging {
            true => Some(selection.start..next_selection_end),
            false => Some(next_selection_end..next_selection_end),
        }
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
