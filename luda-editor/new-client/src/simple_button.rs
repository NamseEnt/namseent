use super::*;

pub fn simple_button<Text: AsRef<str>, OnMouseUpIn: FnOnce(MouseEvent)>(
    wh: Wh<Px>,
    text: Text,
    on_mouse_up_in: OnMouseUpIn,
) -> button::TextButton<Text, OnMouseUpIn> {
    button::TextButton {
        rect: Rect::zero_wh(wh),
        text,
        text_color: Color::WHITE,
        stroke_color: Color::WHITE,
        stroke_width: 1.px(),
        fill_color: Color::TRANSPARENT,
        mouse_buttons: vec![MouseButton::Left],
        on_mouse_up_in,
    }
}

pub fn simple_toggle_button<Text: AsRef<str>, OnMouseUpIn: FnOnce(MouseEvent)>(
    wh: Wh<Px>,
    text: Text,
    is_on: bool,
    on_mouse_up_in: OnMouseUpIn,
) -> button::TextButton<Text, OnMouseUpIn> {
    button::TextButton {
        rect: Rect::zero_wh(wh),
        text,
        text_color: Color::WHITE,
        stroke_color: Color::WHITE,
        stroke_width: 1.px(),
        fill_color: if is_on {
            Color::BLUE
        } else {
            Color::TRANSPARENT
        },
        mouse_buttons: vec![MouseButton::Left],
        on_mouse_up_in,
    }
}
