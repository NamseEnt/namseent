use crate::{simple_rect, typography::center_text_full_height};
use namui::prelude::*;

fn attach_text_button_event<'a>(
    button: impl 'a + Component,
    mouse_buttons: impl IntoIterator<Item = MouseButton>,
    on_mouse_up_in: impl 'a + FnOnce(MouseEvent),
) -> impl 'a + Component {
    let mouse_buttons = mouse_buttons.into_iter().collect::<Vec<_>>();

    button.attach_event(move |event| {
        if let Event::MouseUp { event } = event {
            if !event.is_local_xy_in() {
                return;
            }
            let Some(button) = event.button else {
                return;
            };
            if mouse_buttons.contains(&button) {
                on_mouse_up_in(event);
            }
        }
    })
}

#[allow(clippy::too_many_arguments)]
pub fn text_button<'a>(
    rect: Rect<Px>,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    mouse_buttons: impl IntoIterator<Item = MouseButton>,
    on_mouse_up_in: impl 'a + FnOnce(MouseEvent),
) -> impl 'a + Component {
    attach_text_button_event(
        translate(
            rect.x(),
            rect.y(),
            render([
                center_text_full_height(rect.wh(), text, text_color),
                simple_rect(rect.wh(), stroke_color, stroke_width, fill_color),
            ]),
        ),
        mouse_buttons,
        on_mouse_up_in,
    )
}

#[allow(clippy::too_many_arguments)]
pub fn text_button_fit<'a>(
    height: Px,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    side_padding: Px,
    mouse_buttons: impl IntoIterator<Item = MouseButton>,
    on_mouse_up_in: impl 'a + FnOnce(MouseEvent),
) -> impl 'a + Component {
    let mouse_buttons = mouse_buttons.into_iter().collect::<Vec<_>>();
    let center_text = center_text_full_height(Wh::new(0.px(), height), text, text_color);
    let width = match center_text.bounding_box() {
        Some(bounding_box) => bounding_box.width(),
        None => return None,
    };

    Some(attach_text_button_event(
        render([
            translate(width / 2 + side_padding, 0.px(), center_text),
            simple_rect(
                Wh::new(width + side_padding * 2, height),
                stroke_color,
                stroke_width,
                fill_color,
            ),
        ]),
        mouse_buttons,
        on_mouse_up_in,
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn text_button_fit_align<'a>(
    wh: Wh<Px>,
    align: TextAlign,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    side_padding: Px,
    mouse_buttons: impl IntoIterator<Item = MouseButton>,
    on_mouse_up_in: impl 'a + FnOnce(MouseEvent),
) -> impl 'a + Component {
    let mouse_buttons = mouse_buttons.into_iter().collect::<Vec<_>>();
    let center_text = center_text_full_height(Wh::new(0.px(), wh.height), text, text_color);
    let center_text_width = match center_text.bounding_box() {
        Some(bounding_box) => bounding_box.width(),
        None => return None,
    };
    let center_text_x = (wh.width - center_text_width)
        * match align {
            TextAlign::Left => 0.0,
            TextAlign::Center => 0.5,
            TextAlign::Right => 1.0,
        };

    Some(attach_text_button_event(
        render([
            translate(center_text_x, 0.px(), center_text),
            translate(
                center_text_x - center_text_width / 2 - side_padding,
                0.px(),
                simple_rect(
                    Wh::new(center_text_width + side_padding * 2, wh.height),
                    stroke_color,
                    stroke_width,
                    fill_color,
                ),
            ),
        ]),
        mouse_buttons,
        on_mouse_up_in,
    ))
}

#[allow(clippy::too_many_arguments)]
pub fn body_text_button<'a>(
    rect: Rect<Px>,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    text_align: TextAlign,
    mouse_buttons: impl IntoIterator<Item = MouseButton>,
    on_mouse_up_in: impl 'a + FnOnce(MouseEvent),
) -> impl 'a + Component {
    attach_text_button_event(
        translate(
            rect.x(),
            rect.y(),
            render([
                match text_align {
                    TextAlign::Left => {
                        crate::typography::body::left(rect.wh().height, text, text_color)
                    }
                    TextAlign::Center => {
                        crate::typography::body::center(rect.wh(), text, text_color)
                    }
                    TextAlign::Right => crate::typography::body::right(rect.wh(), text, text_color),
                },
                simple_rect(rect.wh(), stroke_color, stroke_width, fill_color),
            ]),
        ),
        mouse_buttons,
        on_mouse_up_in,
    )
}
