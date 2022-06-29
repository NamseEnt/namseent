use namui::prelude::*;

pub fn simple_rect(
    wh: Wh<f32>,
    stroke_color: Color,
    stroke_width: f32,
    fill_color: Color,
) -> RenderingTree {
    namui::rect(RectParam {
        x: 0.0,
        y: 0.0,
        width: wh.width,
        height: wh.height,
        style: RectStyle {
            stroke: Some(RectStroke {
                color: stroke_color,
                width: stroke_width,
                border_position: BorderPosition::Inside,
            }),
            fill: Some(RectFill { color: fill_color }),
            ..Default::default()
        },
        ..Default::default()
    })
}
