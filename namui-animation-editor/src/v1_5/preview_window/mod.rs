use namui::{
    animation::{Animate, KeyframeGraph, Layer},
    prelude::*,
    types::{PixelSize, Time, TimePerPixel},
};
use namui_prebuilt::{
    table::{fixed, ratio, vertical},
    *,
};
use std::{
    any::Any,
    sync::{Arc, RwLock},
};

pub(crate) struct PreviewWindow {}

pub(crate) struct Props<'a> {
    pub animation: &'a animation::Animation,
    pub playback_time: Time,
}

#[derive(Debug, Clone)]
enum Event {}

pub(crate) struct PreviewWindowContext {
    pub start_at: Time,
    pub time_per_pixel: TimePerPixel,
}

impl PreviewWindow {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn update(&mut self, _event: &dyn Any) {}
}

impl table::CellRender<Props<'_>> for PreviewWindow {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        namui::clip(
            namui::PathBuilder::new().add_rect(&LtrbRect {
                left: 0.0,
                top: 0.0,
                right: wh.width,
                bottom: wh.height,
            }),
            ClipOp::Intersect,
            namui::scale(
                wh.width / 1920.0,
                wh.height / 1080.0,
                props.animation.render(props.playback_time),
            ),
        )
    }
}
