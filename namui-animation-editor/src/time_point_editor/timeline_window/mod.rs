use crate::types::{Act, ActionTicket, AnimationHistory};
use namui::{
    animation::{Animation, KeyframeGraph, KeyframeValue, Layer},
    prelude::*,
    types::*,
};
use namui_prebuilt::{table::*, *};
mod render;
mod update;

pub struct TimelineWindow {
    animation_history: AnimationHistory,
    window_id: String,
    start_at: Time,
    time_per_pixel: TimePerPixel,
    dragging: Option<Dragging>,
    playback_time: Time,
}

impl TimelineWindow {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            animation_history,
            window_id: namui::nanoid(),
            start_at: Time::from_ms(-1000.0),
            time_per_pixel: TimePerPixel::from_ms_per_pixel(10.0),
            dragging: None,
            playback_time: Time::zero(),
        }
    }
    pub fn get_playback_time(&self) -> Time {
        self.playback_time
    }
}

pub(crate) struct Props<'a> {
    pub layers: &'a [Layer],
    pub wh: Wh<f32>,
    pub selected_layer_id: Option<String>,
}

pub(super) enum Event {
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
        selected_layer_id: Option<String>,
    },
    TimelineMouseMoveIn {
        mouse_local_xy: Xy<f32>,
    },
    KeyframeMouseDown {
        point_ids: Vec<String>,
        anchor_xy: Xy<f32>,
        keyframe_time: Time,
        mouse_local_xy: Xy<f32>,
        layer_id: String,
    },
    TimelineDeleteKeyDown {
        selected_layer_id: Option<String>,
        playback_time: Time,
    },
}

enum Dragging {
    Background { last_mouse_local_xy: Xy<f32> },
    Keyframe { action_ticket: ActionTicket },
}

fn get_time_and_id<T: KeyframeValue + Clone>(graph: &KeyframeGraph<T>) -> Vec<(Time, String)> {
    graph
        .get_points_with_lines()
        .iter()
        .map(|(point, _)| (point.time, point.id().to_string()))
        .collect()
}

fn get_all_time_and_ids(layer: &Layer) -> Vec<(Time, String)> {
    [
        get_time_and_id(&layer.image.x),
        get_time_and_id(&layer.image.y),
        get_time_and_id(&layer.image.width_percent),
        get_time_and_id(&layer.image.height_percent),
        get_time_and_id(&layer.image.rotation_angle),
        get_time_and_id(&layer.image.opacity),
    ]
    .concat()
}
