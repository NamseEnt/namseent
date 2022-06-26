use crate::types::{ActionTicket, AnimationHistory};
use namui::{
    prelude::*,
    types::{Radian, Time},
};
use namui_prebuilt::*;
mod render;
mod update;

pub struct WysiwygWindow {
    window_id: String,
    animation_history: AnimationHistory,
    real_left_top_xy: Xy<f32>,
    real_pixel_size_per_screen_pixel_size: f32,
    last_wh: Option<Wh<f32>>,
    dragging: Option<Dragging>,
}

impl WysiwygWindow {
    pub fn new(animation_history: AnimationHistory) -> Self {
        Self {
            window_id: namui::nanoid(),
            animation_history,
            real_left_top_xy: Xy { x: -5.0, y: -5.0 },
            real_pixel_size_per_screen_pixel_size: 2.0,
            last_wh: None,
            dragging: None,
        }
    }
}

pub struct Props<'a> {
    pub wh: Wh<f32>,
    pub playback_time: Time,
    pub animation: &'a animation::Animation,
    pub selected_layer_id: Option<String>,
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
    Background { anchor_xy: Xy<f32> },
    ResizeCircle { ticket: ActionTicket },
    ImageBody { ticket: ActionTicket },
    Rotation { ticket: ActionTicket },
}

enum Event {
    BackgroundClicked {
        mouse_xy: Xy<f32>,
    },
    MouseMoveIn {
        mouse_local_xy: Xy<f32>,
    },
    ShiftWheel {
        delta: f32,
    },
    Wheel {
        delta: f32,
    },
    AltWheel {
        delta: f32,
        mouse_local_xy: Xy<f32>,
    },
    UpdateWh {
        wh: Wh<f32>,
    },
    SelectedLayerMouseDown {
        layer_id: String,
        anchor_xy: Xy<f32>,
        playback_time: Time,
    },
    ResizeCircleMouseDown {
        layer_id: String,
        location: ResizeCircleLocation,
        anchor_xy: Xy<f32>,
        playback_time: Time,
        rotation_radian: Radian,
    },
    RotationToolMouseDown {
        image_center_real_xy: Xy<f32>,
        mouse_local_xy: Xy<f32>,
        playback_time: Time,
        layer_id: String,
    },
}
