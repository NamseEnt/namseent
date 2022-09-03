use super::Props;
use crate::{
    draw::text::get_line_height,
    namui::{self, TextInput},
    *,
};
use std::ops::Range;

impl TextInput {
    pub(crate) fn get_selection_on_mouse_movement(
        &self,
        props: &Props,
        click_local_xy: Xy<Px>,
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

        let is_dragging = is_shift_key_pressed || is_dragging_by_mouse;

        Some(get_one_click_selection(
            &props.text_param,
            &font,
            click_local_xy,
            is_dragging,
            &self.selection,
        ))
    }
}

fn get_one_click_selection(
    text_param: &TextParam,
    font: &namui::Font,
    click_local_xy: Xy<Px>,
    is_dragging: bool,
    last_selection: &Option<Range<usize>>,
) -> Range<usize> {
    let selection_index_of_xy = get_selection_index_of_x(font, text_param, click_local_xy);

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

/// TODO: Calculate Baseline
fn get_selection_index_of_x(
    font: &namui::Font,
    text_param: &TextParam,
    click_local_xy: Xy<Px>,
) -> usize {
    let text = &text_param.text;
    if click_local_xy.y <= 0.px() {
        return 0;
    }

    let line_height = get_line_height(font.size);
    let line_index = (click_local_xy.y / line_height).floor() as usize;
    let line_max_index = text.lines().count() - 1;

    if line_index > line_max_index {
        return text.chars().count();
    }

    let index_before_line = {
        let mut index = 0;
        for line_with_newline in text.split_inclusive("\n").take(line_index) {
            index += line_with_newline.chars().count();
        }
        index
    };

    let line_text = text.lines().nth(line_index).unwrap();

    let glyph_ids = font.get_glyph_ids(line_text);
    let glyph_widths = font.get_glyph_widths(glyph_ids, None);

    let line_width = glyph_widths.iter().sum::<Px>();

    let aligned_x = match text_param.align {
        namui::TextAlign::Left => click_local_xy.x,
        namui::TextAlign::Center => click_local_xy.x + line_width / 2.0,
        namui::TextAlign::Right => click_local_xy.x + line_width,
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

    index_before_line + index
}
