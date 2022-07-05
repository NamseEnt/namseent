use namui::{animation::Animate, prelude::*, types::Time};
use std::any::Any;

pub struct PreviewWindow {}

pub struct Props<'a> {
    pub wh: Wh<f32>,
    pub animation: &'a animation::Animation,
    pub playback_time: Time,
}

#[derive(Debug, Clone)]
enum Event {}

impl PreviewWindow {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&mut self, _event: &dyn Any) {}
    pub fn render(&self, props: Props) -> RenderingTree {
        namui::clip(
            namui::PathBuilder::new().add_rect(&LtrbRect {
                left: 0.0,
                top: 0.0,
                right: props.wh.width,
                bottom: props.wh.height,
            }),
            ClipOp::Intersect,
            namui::scale(
                props.wh.width / 1920.0,
                props.wh.height / 1080.0,
                props.animation.render(props.playback_time),
            ),
        )
    }
}
