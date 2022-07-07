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
