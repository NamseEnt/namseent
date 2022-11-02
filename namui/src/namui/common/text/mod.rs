mod glyph_group;
mod line_texts;
mod multiline_caret;

use crate::{namui::skia::FontMetrics, *};
pub(crate) use glyph_group::*;
pub(crate) use line_texts::*;
use std::{collections::VecDeque, sync::Arc};

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
pub fn get_fallback_fonts(font_size: IntPx) -> VecDeque<Arc<Font>> {
    crate::typeface::get_fallback_font_typefaces()
        .map(|typeface| crate::font::get_font_of_typeface(typeface.clone(), font_size))
        .collect()
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
    paint: &Arc<Paint>,
) -> Px {
    let groups = get_glyph_groups(text, fonts, paint);
    groups.iter().map(|group| group.width).sum()
}
