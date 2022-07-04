use super::Props;
use crate::{
    namui::{self, get_text_width_internal, TextInput},
    *,
};
use std::ops::Range;

impl TextInput {
    pub(crate) fn get_selection_on_mouse_movement(
        &self,
        props: &Props,
        click_local_x: Px,
        is_dragging_by_mouse: bool,
    ) -> Option<Range<usize>> {
        let (font, is_shift_key_pressed) = {
            let font = crate::font::get_font(props.text_param.font_type);

            let is_shift_key_pressed =
                crate::keyboard::any_code_press([namui::Code::ShiftLeft, namui::Code::ShiftRight]);

            (font, is_shift_key_pressed)
        };
        if font.is_none() {
            return None;
        };
        let font = font.unwrap();

        // const continouslyFastClickCount: number;

        // if (continouslyFastClickCount >= 3) {
        //   return getMoreTripleClickSelection({ text });
        // }
        // if (continouslyFastClickCount === 2) {
        //   return getDoubleClickSelection({ text, font, x: localX });
        // }

        let text_width = get_text_width_internal(
            &font,
            &props.text_param.text,
            props.text_param.style.drop_shadow.map(|shadow| shadow.x),
        );

        let aligned_x = match props.text_param.align {
            namui::TextAlign::Left => click_local_x - props.text_param.x,
            namui::TextAlign::Center => click_local_x - props.text_param.x + text_width / 2.0,
            namui::TextAlign::Right => click_local_x - props.text_param.x + text_width,
        };

        let is_dragging = is_shift_key_pressed || is_dragging_by_mouse;

        Some(get_one_click_selection(
            &props.text_param.text,
            &font,
            aligned_x,
            is_dragging,
            &self.selection,
        ))
    }
}

fn get_one_click_selection(
    text: &str,
    font: &namui::Font,
    local_x: Px,
    is_dragging: bool,
    last_selection: &Option<Range<usize>>,
) -> Range<usize> {
    let selection_index_of_x = get_selection_index_of_x(font, text, local_x);

    let start = match last_selection {
        Some(last_selection) => {
            if !is_dragging {
                selection_index_of_x
            } else {
                last_selection.start
            }
        }
        None => selection_index_of_x,
    };

    start..selection_index_of_x
}

fn get_selection_index_of_x(font: &namui::Font, text: &str, local_x: Px) -> usize {
    let glyph_ids = font.get_glyph_ids(text);
    let glyph_widths = font.get_glyph_widths(glyph_ids, None);

    let mut left = px(0.0);
    let index = glyph_widths.iter().position(|width| {
        let center = left + width / 2.0;
        if local_x < center {
            return true;
        }
        left += *width;
        return false;
    });

    return index.unwrap_or(text.len());
}
