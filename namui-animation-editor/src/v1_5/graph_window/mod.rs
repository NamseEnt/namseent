use namui::{
    animation::{KeyframeGraph, Layer},
    prelude::*,
    types::{Degree, OneZero, PixelSize, Time, TimePerPixel},
};
use namui_prebuilt::{
    table::{fixed, ratio, vertical},
    *,
};
use std::{ops::Range, sync::Arc};
mod render;
use super::read_only_lock::ReadOnlyLock;
mod time_ruler;
mod update;

pub(crate) struct GraphWindow {
    id: String,
    context: GraphWindowContext,
    x_context: PropertyContext<PixelSize>,
    y_context: PropertyContext<PixelSize>,
    width_context: PropertyContext<PixelSize>,
    height_context: PropertyContext<PixelSize>,
    rotation_angle_context: PropertyContext<Degree>,
    opacity_context: PropertyContext<OneZero>,
    mouse_over_row: Option<MouseOverRow>,
    row_height: Option<f32>,
    animation: ReadOnlyLock<animation::Animation>,
    selected_point_address: Option<PointAddress>,
    dragging: Option<Dragging>,
}

pub(crate) struct Props<'a> {
    pub layer: Option<&'a namui::animation::Layer>,
    pub playback_time: Time,
}

#[derive(Debug, Clone)]
enum Dragging {
    Point(PointAddress),
    Background {
        property_name: PropertyName,
        last_mouse_local_xy: Xy<f32>,
    },
}

#[derive(Debug, Clone)]
struct MouseOverRow {
    property_name: PropertyName,
    mouse_local_xy: Xy<f32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PointAddress {
    layer_id: String,
    property_name: PropertyName,
    point_id: String,
}

#[derive(Debug, Clone)]
enum Event {
    GraphMouseMoveIn {
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    },
    GraphMouseMoveOut,
    GraphShiftMouseWheel {
        delta: PixelSize,
    },
    GraphAltMouseWheel {
        delta: PixelSize,
        mouse_local_xy: Xy<f32>,
    },
    GraphCtrlMouseWheel {
        delta: PixelSize,
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    },
    GraphMouseRightDown {
        layer_id: String,
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    },
    GraphMouseLeftDown {
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
    },
    RowHeightChange {
        row_height: f32,
    },
    TimelineTimeRulerClicked {
        click_position_in_time: Time,
    },
    GraphPointMouseDown {
        point_address: PointAddress,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PropertyName {
    X,
    Y,
    Width,
    Height,
    RotationAngle,
    Opacity,
}

pub(crate) struct GraphWindowContext {
    pub start_at: Time,
    pub time_per_pixel: TimePerPixel,
}

impl GraphWindow {
    pub(crate) fn new(animation: ReadOnlyLock<animation::Animation>) -> Self {
        Self {
            id: namui::nanoid(),
            context: GraphWindowContext {
                start_at: Time::zero(),
                time_per_pixel: Time::from_ms(50.0) / PixelSize::new(1.0),
            },
            x_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 10.0.into(),
                    pixel_size: 1.0.into(),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|&x| (x as f32).into())
                    .collect(),
                gradation_pixel_size_range: 15.0.into()..30.0.into(),
            },
            y_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 10.0.into(),
                    pixel_size: 1.0.into(),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|&x| (x as f32).into())
                    .collect(),
                gradation_pixel_size_range: 15.0.into()..30.0.into(),
            },
            width_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 10.0.into(),
                    pixel_size: 1.0.into(),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|&x| (x as f32).into())
                    .collect(),
                gradation_pixel_size_range: 15.0.into()..30.0.into(),
            },
            height_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 10.0.into(),
                    pixel_size: 1.0.into(),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|&x| (x as f32).into())
                    .collect(),
                gradation_pixel_size_range: 15.0.into()..30.0.into(),
            },
            rotation_angle_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 1.0.into(),
                    pixel_size: 1.0.into(),
                },
                gradation_value_candidates: [5, 10, 15, 30, 60, 90, 360]
                    .iter()
                    .map(|&x| (x as f32).into())
                    .collect(),
                gradation_pixel_size_range: 15.0.into()..30.0.into(),
            },
            opacity_context: PropertyContext {
                value_at_bottom: 0.0.into(),
                value_per_pixel: ValuePerPixel {
                    value: 0.01.into(),
                    pixel_size: 1.0.into(),
                },
                gradation_value_candidates: [0.1].iter().map(|&x| (x as f32).into()).collect(),
                gradation_pixel_size_range: 15.0.into()..30.0.into(),
            },
            mouse_over_row: None,
            row_height: None,
            animation,
            selected_point_address: None,
            dragging: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ValuePerPixel<TValue> {
    value: TValue,
    pixel_size: PixelSize,
}

impl<TValue: Into<f32> + From<f32>> std::ops::Mul<PixelSize> for ValuePerPixel<TValue> {
    type Output = TValue;

    fn mul(self, rhs: PixelSize) -> Self::Output {
        (self.value.into() * (rhs / self.pixel_size)).into()
    }
}
impl<TValue: Into<f32> + Copy> ValuePerPixel<TValue> {
    fn get_pixel_size(&self, value: TValue) -> PixelSize {
        let value: f32 = value.into();
        let self_value: f32 = self.value.into();
        (value / self_value) * self.pixel_size
    }
}

struct Context<'a, TValue> {
    start_at: Time,
    time_per_pixel: TimePerPixel,
    property_context: &'a PropertyContext<TValue>,
    mouse_local_xy: Option<Xy<f32>>,
    property_name: PropertyName,
    selected_point_id: Option<String>,
    layer: &'a Layer,
}

#[derive(Debug, Clone)]
struct PropertyContext<TValue> {
    value_per_pixel: ValuePerPixel<TValue>,
    value_at_bottom: TValue,
    gradation_value_candidates: Box<[TValue]>,
    gradation_pixel_size_range: Range<PixelSize>,
}
