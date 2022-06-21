use namui::{animation::Layer, prelude::*, types::*};
use namui_prebuilt::{table::*, *};
mod render;
mod update;

pub struct TimelineWindow {
    start_at: Time,
    time_per_pixel: TimePerPixel,
    dragging: Option<Dragging>,
}

impl TimelineWindow {
    pub fn new() -> Self {
        Self {
            start_at: Time::zero(),
            time_per_pixel: TimePerPixel::from_ms_per_pixel(100.0),
            dragging: None,
        }
    }
}

pub(crate) struct Props<'a> {
    pub layers: &'a [Layer],
    pub wh: Wh<f32>,
    pub selected_layer_id: Option<String>,
}

enum Event {
    ShiftWheel { delta: f32 },
    AltWheel { delta: f32, anchor_xy: Xy<f32> },
    TimelineClicked { mouse_local_xy: Xy<f32> },
    TimelineMouseMoveIn { mouse_local_xy: Xy<f32> },
}

enum Dragging {
    Background { last_mouse_local_xy: Xy<f32> },
}
