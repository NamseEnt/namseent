use crate::app::cropper::{
    render_back_button::render_back_button, render_save_button::render_save_button,
};
use namui::prelude::*;

pub fn render_app_bar(wh: Wh<Px>) -> RenderingTree {
    const MARGIN: Px = px(8.0);
    let inner_height = wh.height - 2.0 * MARGIN;
    let button_wh = Wh {
        width: px(128.0),
        height: inner_height,
    };

    render([
        render_background(wh),
        translate(MARGIN, MARGIN, render_back_button(button_wh)),
        translate(
            wh.width - MARGIN - button_wh.width,
            MARGIN,
            render_save_button(button_wh),
        ),
    ])
}

fn render_background(wh: Wh<Px>) -> RenderingTree {
    namui::rect(RectParam {
        rect: Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: wh.width,
            height: wh.height,
        },
        style: RectStyle {
            stroke: None,
            fill: Some(RectFill {
                color: Color::from_u8(44, 62, 80, 255),
            }),
            round: None,
        },
    })
}
