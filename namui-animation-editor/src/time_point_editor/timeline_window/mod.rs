use namui::{
    prelude::*,
    types::{Time, TimePerPixel},
};
use namui_prebuilt::{table::*, *};
mod render;
mod update;

pub struct TimelineWindow {
    start_at: Time,
    time_per_pixel: TimePerPixel,
}

impl TimelineWindow {
    pub fn new() -> Self {
        Self {
            start_at: Time::zero(),
            time_per_pixel: TimePerPixel::from_ms_per_pixel(100.0),
        }
    }
}

pub struct Props {
    pub wh: Wh<f32>,
}

enum Event {}

enum Dragging {}
