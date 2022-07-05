use crate::types::{Act, ActionTicket, AnimationHistory};
use namui::{
    animation::{Animation, KeyframeGraph, KeyframeValue, Layer},
    prelude::*,
};
use namui_prebuilt::{table::*, *};
mod playing_status;
use playing_status::*;
mod render;
mod update;

pub struct TimelineWindow {
    animation_history: AnimationHistory,
    window_id: String,
    start_at: Time,
    time_per_px: TimePerPx,
    dragging: Option<Dragging>,
    playing_status: PlayingStatus,
}

impl TimelineWindow {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            animation_history,
            window_id: namui::nanoid(),
            start_at: Time::Ms(-1000.0),
            time_per_px: Time::Ms(10.0) / Px::from(1.0_f32),
            dragging: None,
            playing_status: PlayingStatus::new(),
        }
    }
    pub fn get_playback_time(&self) -> Time {
        self.playing_status.get_playback_time()
    }
    pub fn set_playback_time(&mut self, time: Time) {
        self.playing_status.set_playback_time(time);
    }
}

pub(crate) struct Props<'a> {
    pub layers: &'a [Layer],
    pub wh: Wh<Px>,
    pub selected_layer_id: Option<String>,
}

pub(super) enum Event {
    ShiftWheel {
        delta: f32,
    },
    AltWheel {
        delta: f32,
        anchor_xy: Xy<Px>,
    },
    TimelineLeftMouseDown {
        mouse_local_xy: Xy<Px>,
    },
    TimelineRightMouseDown {
        mouse_local_xy: Xy<Px>,
        selected_layer_id: Option<String>,
    },
    TimelineMouseMoveIn {
        mouse_local_xy: Xy<Px>,
    },
    KeyframeMouseDown {
        point_ids: Vec<String>,
        anchor_xy: Xy<Px>,
        keyframe_time: Time,
        mouse_local_xy: Xy<Px>,
        layer_id: String,
    },
    TimelineDeleteKeyDown {
        selected_layer_id: Option<String>,
        playback_time: Time,
    },
    TimelineSpaceKeyDown,
}

enum Dragging {
    Background { last_mouse_local_xy: Xy<Px> },
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
