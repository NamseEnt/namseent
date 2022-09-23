use super::EditingTarget;
use crate::types::{Act, ActionTicket, AnimationHistory};
use namui::{
    animation::{Animation, Layer},
    prelude::*,
};
use namui_prebuilt::{table::*, *};
use playing_status::*;

mod playing_status;
mod render;
mod update;

pub struct TimelineWindow {
    animation_history: AnimationHistory,
    window_id: namui::Uuid,
    start_at: Time,
    time_per_px: TimePerPx,
    dragging: Option<Dragging>,
    playing_status: PlayingStatus,
}

impl TimelineWindow {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            animation_history,
            window_id: namui::uuid(),
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
    pub selected_layer_id: Option<Uuid>,
    pub editing_target: Option<EditingTarget>,
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
        selected_layer_id: Option<Uuid>,
    },
    TimelineMouseMoveIn {
        mouse_local_xy: Xy<Px>,
    },
    KeyframeMouseDown {
        point_id: namui::Uuid,
        anchor_xy: Xy<Px>,
        keyframe_time: Time,
        mouse_local_xy: Xy<Px>,
        layer_id: namui::Uuid,
    },
    MouseLeftDownOutOfEditingTargetButInWindow,
    TimelineDeleteKeyDown {
        selected_layer_id: Option<Uuid>,
        playback_time: Time,
    },
    TimelineSpaceKeyDown {
        selected_layer_id: Option<Uuid>,
        editing_target: Option<EditingTarget>,
    },
    LineMouseDown {
        point_id: namui::Uuid,
        layer_id: namui::Uuid,
    },
}

enum Dragging {
    Background { last_mouse_local_xy: Xy<Px> },
    Keyframe { action_ticket: ActionTicket },
}
