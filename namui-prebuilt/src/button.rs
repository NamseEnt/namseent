use crate::{simple_rect, typography::center_text};
use namui::prelude::*;
use std::sync::Arc;

pub fn text_button(
    xywh: XywhRect<f32>,
    text: &str,
    text_color: Color,
    stroke_color: Color,
    stroke_width: f32,
    fill_color: Color,
    on_mouse_down: impl Fn() + 'static,
) -> namui::RenderingTree {
    let on_mouse_down = Arc::new(on_mouse_down);
    translate(
        xywh.x,
        xywh.y,
        render([
            simple_rect(xywh.wh(), stroke_color, stroke_width, fill_color),
            center_text(xywh.wh(), text, text_color),
        ]),
    )
    .attach_event(|builder| {
        let on_mouse_down = on_mouse_down.clone();
        builder.on_mouse_down(move |_| {
            on_mouse_down();
        });
    })
}
