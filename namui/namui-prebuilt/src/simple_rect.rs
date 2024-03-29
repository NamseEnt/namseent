use namui::*;

pub fn simple_rect(
    wh: Wh<Px>,
    stroke_color: Color,
    stroke_width: Px,
    fill_color: Color,
) -> RenderingTree {
    namui::rect(RectParam {
        rect: Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: wh.width,
            height: wh.height,
        },
        style: RectStyle {
            stroke: Some(RectStroke {
                color: stroke_color,
                width: stroke_width,
                border_position: BorderPosition::Inside,
            }),
            fill: Some(RectFill { color: fill_color }),
            ..Default::default()
        },
    })
}

pub fn transparent_rect(wh: Wh<Px>) -> RenderingTree {
    simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT)
}
