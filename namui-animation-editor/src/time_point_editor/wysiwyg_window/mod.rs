use namui::{prelude::*, types::Time};
use namui_prebuilt::{table::*, *};
mod render;
mod update;

pub struct WysiwygWindow {
    animation: crate::ReadOnlyLock<animation::Animation>,
    real_left_top_xy: Xy<f32>,
    real_pixel_size_per_screen_pixel_size: f32,
    last_wh: Option<Wh<f32>>,
    selected_layer_id: Option<String>,
    dragging: Option<Dragging>,
    mouse_local_xy: Option<Xy<f32>>,
}

impl WysiwygWindow {
    pub fn new(animation: crate::ReadOnlyLock<animation::Animation>) -> Self {
        Self {
            animation,
            real_left_top_xy: Xy { x: -5.0, y: -5.0 },
            real_pixel_size_per_screen_pixel_size: 2.0,
            last_wh: None,
            selected_layer_id: None,
            dragging: None,
            mouse_local_xy: None,
        }
    }
}

pub struct Props {
    pub wh: Wh<f32>,
    pub playback_time: Time,
}

#[derive(Clone, Copy, Debug)]
enum ResizeCircleLocation {
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
    Background {
        anchor_xy: Xy<f32>,
    },
    ResizeCircle {
        location: ResizeCircleLocation,
        anchor_xy: Xy<f32>,
        playback_time: Time,
    },
    ImageBody {
        anchor_xy: Xy<f32>,
        playback_time: Time,
    },
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
    LayerClicked {
        layer_id: String,
        anchor_xy: Xy<f32>,
        playback_time: Time,
    },
    ResizeCircleClicked {
        location: ResizeCircleLocation,
        anchor_xy: Xy<f32>,
        playback_time: Time,
    },
}
