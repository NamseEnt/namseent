use crate::{engine::*, render};
use chrono::Duration;

pub struct DurationPerPixel {
    duration: Duration,
    pixels: i32,
}
impl DurationPerPixel {
    pub fn new(duration: Duration, pixels: i32) -> Self {
        DurationPerPixel { duration, pixels }
    }
}
impl std::ops::Mul<i32> for DurationPerPixel {
    type Output = Duration;
    fn mul(self, pixels: i32) -> Self::Output {
        self.duration * pixels / self.pixels
    }
}

pub struct TimeRuler {}
pub struct TimeRulerProps {
    pub xywh: XywhRect<f32>,
    pub start: Duration,
    pub duration_per_pixel: DurationPerPixel,
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
                    XywhRect {
                        x: 0.0,
                        y: 0.0,
                        width: props.xywh.width,
                        height: props.xywh.height,
                    }
                    .into_ltrb(),
                ),
                ClipOp::Intersect,
                render![rect(RectParam {
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
                })]
            )],
        )
    }
}
