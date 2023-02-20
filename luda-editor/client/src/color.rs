use namui::Color;

pub const BACKGROUND: Color = Color::grayscale_u8(32);
pub const STROKE_NORMAL: Color = Color::grayscale_u8(204);
pub const STROKE_DISABLED: Color = Color::grayscale_u8(128);
pub const STROKE_HOVER: Color = Color::grayscale_u8(230);
pub const STROKE_SELECTED: Color = Color::grayscale_u8(255);

pub const STROKE_FOCUS: Color = Color::from_u8(155, 109, 255, 255);

pub const fn stroke_color(is_selected: bool, is_focused: bool) -> Color {
    if is_selected && is_focused {
        STROKE_FOCUS
    } else if is_selected {
        STROKE_SELECTED
    } else {
        STROKE_NORMAL
    }
}
