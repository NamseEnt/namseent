use crate::{simple_rect, typography::center_text};
use namui::prelude::*;
use std::sync::Arc;

pub fn text_button(
    rect: Rect<Px>,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    on_mouse_down_in: impl Fn() + 'static,
) -> namui::RenderingTree {
    let on_mouse_down_in = Arc::new(on_mouse_down_in);
    translate(
        rect.x(),
        rect.y(),
        render([
            simple_rect(rect.wh(), stroke_color, stroke_width, fill_color),
            center_text(rect.wh(), text, text_color),
        ]),
    )
    .attach_event(|builder| {
        let on_mouse_down_in = on_mouse_down_in.clone();
        builder.on_mouse_down_in(move |_| {
            on_mouse_down_in();
        });
    })
}

pub fn text_button_fit(
    height: Px,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    side_padding: Px,
    on_mouse_down_in: impl Fn() + 'static,
) -> namui::RenderingTree {
    let center_text = center_text(Wh::new(0.px(), height), text, text_color);
    let width = match center_text.get_bounding_box() {
        Some(bounding_box) => bounding_box.width(),
        None => return RenderingTree::Empty,
    };

    let on_mouse_down_in = Arc::new(on_mouse_down_in);

    render([
        simple_rect(
            Wh::new(width + 2 * side_padding, height),
            stroke_color,
            stroke_width,
            fill_color,
        ),
        translate(width / 2 + side_padding, 0.px(), center_text),
    ])
    .attach_event(|builder| {
        let on_mouse_down_in = on_mouse_down_in.clone();
        builder.on_mouse_down_in(move |_| {
            on_mouse_down_in();
        });
    })
}

pub fn body_text_button(
    rect: Rect<Px>,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
    text_align: TextAlign,
    on_mouse_down_in: impl Fn() + 'static,
) -> namui::RenderingTree {
    let on_mouse_down_in = Arc::new(on_mouse_down_in);
    translate(
        rect.x(),
        rect.y(),
        render([
            simple_rect(rect.wh(), stroke_color, stroke_width, fill_color),
            match text_align {
                TextAlign::Left => {
                    crate::typography::body::left(rect.wh().height, text, text_color)
                }
                TextAlign::Center => crate::typography::body::center(rect.wh(), text, text_color),
                TextAlign::Right => crate::typography::body::right(rect.wh(), text, text_color),
            },
        ]),
    )
    .attach_event(|builder| {
        let on_mouse_down_in = on_mouse_down_in.clone();
        builder.on_mouse_down_in(move |_| {
            on_mouse_down_in();
        });
    })
}
