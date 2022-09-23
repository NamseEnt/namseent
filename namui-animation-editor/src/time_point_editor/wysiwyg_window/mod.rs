use super::EditingTarget;
use crate::types::{ActionTicket, AnimationHistory};
use namui::{
    prelude::*,
    types::{Angle, Px, Time},
};
use namui_prebuilt::*;

mod render;
mod update;

pub struct WysiwygWindow {
    window_id: namui::Uuid,
    animation_history: AnimationHistory,
    real_left_top_xy: Xy<Px>,
    real_px_per_screen_px: f32,
    last_wh: Option<Wh<Px>>,
    dragging: Option<Dragging>,
}

impl WysiwygWindow {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            window_id: namui::uuid(),
            animation_history,
            real_left_top_xy: Xy::single(px(-50.0)),
            real_px_per_screen_px: 2.0,
            last_wh: None,
            dragging: None,
        }
    }
}

pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub playback_time: Time,
    pub animation: &'a animation::Animation,
    pub selected_layer_id: Option<Uuid>,
    pub editing_target: Option<EditingTarget>,
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum ResizeCircleLocation {
    LeftTop,
    Top,
    RightTop,
    Left,
    Right,
    LeftBottom,
    Bottom,
    RightBottom,
}

enum Dragging {
    Background { anchor_xy: Xy<Px> },
    ResizeCircle { ticket: ActionTicket },
    ImageBody { ticket: ActionTicket },
    Rotation { ticket: ActionTicket },
}

enum Event {
    BackgroundClicked {
        mouse_xy: Xy<Px>,
    },
    MouseMoveIn {
        mouse_local_xy: Xy<Px>,
    },
    ShiftWheel {
        delta: f32,
    },
    Wheel {
        delta: f32,
    },
    AltWheel {
        delta: f32,
        mouse_local_xy: Xy<Px>,
    },
    UpdateWh {
        wh: Wh<Px>,
    },
    SelectedLayerMouseDown {
        layer_id: namui::Uuid,
        anchor_xy: Xy<Px>,
        keyframe_point_id: namui::Uuid,
    },
    ResizeCircleMouseDown {
        layer_id: namui::Uuid,
        location: ResizeCircleLocation,
        anchor_xy: Xy<Px>,
        keyframe_point_id: namui::Uuid,
        rotation_angle: Angle,
    },
    RotationToolMouseDown {
        image_center_real_xy: Xy<Px>,
        mouse_local_xy: Xy<Px>,
        keyframe_point_id: namui::Uuid,
        layer_id: namui::Uuid,
    },
}
