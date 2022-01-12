use crate::app::types::{PixelSize, Time, TimePerPixel};
use namui::prelude::*;
mod gradations;
use gradations::*;
mod time_text;
use time_text::*;

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
        let gradation_gap_px =
            get_gradation_gap_px(PixelSize(20.0), PixelSize(100.0), props.time_per_pixel);
        let gradation_gap_time = gradation_gap_px * props.time_per_pixel;
        let big_gradation_gap_time = BIG_GRADATION_INTERVAL * gradation_gap_time;
        // NOTE: This code has been designed not to care about negative start_at.
        let gradation_start_at = -(props.start_at % big_gradation_gap_time);
        let gradation_start_px = gradation_start_at / props.time_per_pixel;

        translate(
            props.xywh.x,
            props.xywh.y,
            render![clip(
                PathBuilder::new().add_rect(
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
                    render_gradations(&GradationsProps {
                        wh: props.xywh.wh(),
                        start_px: gradation_start_px,
                        gap_px: gradation_gap_px,
                    }),
                ],
            )],
        )
    }
}

fn get_gradation_gap_px(min: PixelSize, max: PixelSize, time_per_pixel: TimePerPixel) -> PixelSize {
    [
        100, 250, 500, 1000, 5000, 10000, 30000, 60000, 300000, 600000, 1800000,
    ]
    .iter()
    .map(|&ms| Time::from_ms(ms as f32 / 5.0) / time_per_pixel)
    .find(|&px| min <= px && px <= max)
    .unwrap_or(max)
}
