use crate::editor::types::{Time, TimePerPixel};
use namui::prelude::*;

pub struct TimeRuler {}
pub struct TimeRulerProps {
    pub xywh: XywhRect<f32>,
    pub start_at: Time,
    pub time_per_pixel: TimePerPixel,
}

impl TimeRuler {
    pub fn new() -> Self {
        TimeRuler {}
    }
}

impl Entity for TimeRuler {
    type Props = TimeRulerProps;
    fn update(&mut self, event: &dyn std::any::Any) {}
    fn render(&self, props: &Self::Props) -> RenderingTree {
        translate(
            props.xywh.x,
            props.xywh.y,
            render![clip(
                Path::new().add_rect(
                    &XywhRect {
                        x: 0.0,
                        y: 0.0,
                        width: props.xywh.width,
                        height: props.xywh.height,
                    }
                    .into_ltrb(),
                ),
                ClipOp::Intersect,
                render![
                    rect(RectParam {
                        x: 0.0,
                        y: 0.0,
                        width: props.xywh.width,
                        height: props.xywh.height,
                        style: RectStyle {
                            stroke: Some(RectStroke {
                                border_position: BorderPosition::Inside,
                                color: Color::BLACK,
                                width: 1.0,
                            }),
                            fill: Some(RectFill {
                                color: Color::WHITE,
                            }),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    // TODO : TimeTexts
                    // TODO : Gradations
                ],
            )],
        )
    }
}
