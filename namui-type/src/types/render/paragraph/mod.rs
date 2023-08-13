mod caret;
mod paragraph;

use crate::*;
pub use caret::*;
pub use paragraph::*;
use std::fmt::Debug;

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
    paragraph_len: usize,
) -> Px {
    match baseline {
        TextBaseline::Top => 0.px(),
        TextBaseline::Middle => -line_height * 0.5 * (paragraph_len - 1),
        TextBaseline::Bottom => -line_height * (paragraph_len - 1),
    }
}

#[type_derives(Copy)]
pub enum NewLineBy {
    Wrap,
    /// `\n`
    LineFeed,
}

#[type_derives()]
pub struct Line {
    /// Should not have `\n`
    pub chars: Vec<char>,
    pub new_line_by: Option<NewLineBy>,
}

pub trait GroupGlyph: Debug {
    fn groups(&self, text: &str) -> Vec<GlyphGroup>;
    fn width(&self, text: &str) -> Px;
    fn widths(&self, text: &str) -> Vec<Px>;
    fn font_metrics(&self) -> FontMetrics;
}

impl GroupGlyph for &dyn GroupGlyph {
    fn groups(&self, text: &str) -> Vec<GlyphGroup> {
        (*self).groups(text)
    }

    fn width(&self, text: &str) -> Px {
        (*self).width(text)
    }

    fn widths(&self, text: &str) -> Vec<Px> {
        (*self).widths(text)
    }

    fn font_metrics(&self) -> FontMetrics {
        (*self).font_metrics()
    }
}

pub struct GlyphGroup {
    pub glyphs: Vec<Glyph>,
    pub font: Font,
    pub width: Px,
}

pub struct Glyph {
    pub id: usize,
    pub width: Px,
}
