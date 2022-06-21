use namui::{animation::Layer, prelude::*, types::*};
use namui_prebuilt::{table::*, *};
mod render;
mod update;

pub struct TimelineWindow {
    animation: crate::ReadOnlyLock<animation::Animation>,
    id: String,
    start_at: Time,
    time_per_pixel: TimePerPixel,
    dragging: Option<Dragging>,
    pub selected_layer_id: Option<String>,
    selected_point_ids: Option<Vec<String>>,
}

impl TimelineWindow {
    pub fn new(animation: crate::ReadOnlyLock<animation::Animation>) -> Self {
        Self {
            animation,
            id: namui::nanoid(),
            start_at: Time::zero(),
            time_per_pixel: TimePerPixel::from_ms_per_pixel(100.0),
            dragging: None,
            selected_layer_id: None,
            selected_point_ids: None,
        }
    }
}

pub(crate) struct Props<'a> {
    pub layers: &'a [Layer],
    pub wh: Wh<f32>,
    pub playback_time: Time,
}

enum Event {
    ShiftWheel {
        delta: f32,
    },
    AltWheel {
        delta: f32,
        anchor_xy: Xy<f32>,
    },
    TimelineLeftMouseDown {
        mouse_local_xy: Xy<f32>,
    },
    TimelineRightMouseDown {
        mouse_local_xy: Xy<f32>,
    },
    TimelineMouseMoveIn {
        mouse_local_xy: Xy<f32>,
    },
    KeyframeMouseDown {
        point_ids: Vec<String>,
        anchor_xy: Xy<f32>,
    },
}

enum Dragging {
    Background {
        last_mouse_local_xy: Xy<f32>,
    },
    Keyframe {
        point_ids: Vec<String>,
        anchor_xy: Xy<f32>,
    },
}
