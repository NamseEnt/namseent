mod glyph_group;
mod line_texts;
mod measure_glyphs;
pub(crate) mod multiline_caret;
mod text_width;

use crate::{namui::skia::FontMetrics, *};
pub(crate) use glyph_group::*;
pub(crate) use line_texts::*;
pub(crate) use measure_glyphs::*;
use std::sync::Arc;
pub(crate) use text_width::*;

pub fn get_left_in_align(x: Px, align: TextAlign, width: Px) -> Px {
    match align {
        TextAlign::Left => x,
        TextAlign::Center => x - width / 2.0,
        TextAlign::Right => x - width,
    }
}
pub fn get_bottom_of_baseline(baseline: TextBaseline, font_metrics: FontMetrics) -> Px {
    match baseline {
        TextBaseline::Top => -font_metrics.ascent - font_metrics.descent,
        TextBaseline::Middle => (-font_metrics.ascent - font_metrics.descent) / 2.0,
        TextBaseline::Bottom => -font_metrics.descent,
    }
}
pub fn get_multiline_y_baseline_offset(
    baseline: TextBaseline,
    line_height: Px,
    line_texts_len: usize,
) -> Px {
    match baseline {
        TextBaseline::Top => 0.px(),
        TextBaseline::Middle => -line_height * 0.5 * (line_texts_len - 1),
        TextBaseline::Bottom => -line_height * (line_texts_len - 1),
    }
}
pub(crate) fn get_text_width_with_fonts(
    fonts: &Vec<Arc<Font>>,
    text: &str,
    paint: Arc<Paint>,
) -> Px {
    let groups = get_glyph_groups(text, fonts, paint);
    groups.iter().map(|group| group.width).sum()
}
