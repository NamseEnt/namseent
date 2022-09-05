use super::*;
use crate::{text::*, *};

/// TODO: Calculate Baseline
pub(crate) fn get_selection_index_of_x(
    text_align: TextAlign,
    font: &Font,
    line_texts: &LineTexts,
    click_local_xy: Xy<Px>,
) -> usize {
    if click_local_xy.y <= 0.px() {
        return 0;
    }

    let line_height = get_line_height(font.size);
    let line_index = (click_local_xy.y / line_height).floor() as usize;
    let line_max_index = line_texts.line_len() - 1;

    if line_index > line_max_index {
        return line_texts.chars_len();
    }

    let str_index_before_line = line_texts.char_index_before_line(line_index);

    let line_text = line_texts.iter_str().nth(line_index).unwrap();

    let glyph_ids = font.get_glyph_ids(&line_text);
    let glyph_widths = font.get_glyph_widths(glyph_ids, None);

    let line_width = glyph_widths.iter().sum::<Px>();

    let aligned_x = match text_align {
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

    str_index_before_line + index
}
