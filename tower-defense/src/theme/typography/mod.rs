#![allow(dead_code)]
#![allow(clippy::extra_unused_type_parameters)]

mod builder;
mod inline_box;
mod layout;
mod renderer;
mod style;
mod token;

use super::palette;
pub use builder::*;
use namui::*;

pub const HEADLINE_FONT_NAME: &str = "NotoSansKR-Bold";
pub const PARAGRAPH_FONT_NAME: &str = "NotoSansKR-Regular";

pub const HEADLINE_FONT_SIZE_LARGE: IntPx = int_px(24);
pub const HEADLINE_FONT_SIZE_MEDIUM: IntPx = int_px(20);
pub const HEADLINE_FONT_SIZE_SMALL: IntPx = int_px(16);

pub const PARAGRAPH_FONT_SIZE_LARGE: IntPx = int_px(16);
pub const PARAGRAPH_FONT_SIZE_MEDIUM: IntPx = int_px(14);
pub const PARAGRAPH_FONT_SIZE_SMALL: IntPx = int_px(12);

pub const DEFAULT_TEXT_STYLE: TextStyle = TextStyle {
    border: None,
    drop_shadow: None,
    color: palette::ON_SURFACE,
    background: None,
    line_height_percent: percent(130.0),
    underline: None,
};

pub enum FontSize {
    Large,
    Medium,
    Small,
    Custom { size: Px },
}

struct RichTextBuilder {}
