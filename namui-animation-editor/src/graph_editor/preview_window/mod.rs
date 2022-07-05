use namui::{animation::Animate, prelude::*, types::Time};
use std::any::Any;

pub struct PreviewWindow {}

pub struct Props<'a> {
    pub wh: Wh<Px>,
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
            namui::PathBuilder::new().add_rect(Rect::from_xy_wh(Xy::single(px(0.0)), props.wh)),
            ClipOp::Intersect,
            namui::scale(
                props.wh.width / px(1920.0),
                props.wh.height / px(1080.0),
                props.animation.render(props.playback_time),
            ),
        )
    }
}
