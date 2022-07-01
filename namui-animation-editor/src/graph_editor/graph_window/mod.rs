use namui::{
    animation::{KeyframeGraph, Layer},
    math::num::{FromPrimitive, ToPrimitive},
    prelude::*,
    types::{Angle, OneZero, Percent, Px, Time, TimePerPx},
};
use namui_prebuilt::{
    table::{fixed, ratio, vertical},
    *,
};
use std::{
    fmt::Debug,
    ops::{Div, Range},
    sync::Arc,
};
mod render;
use crate::{
    time_ruler,
    types::{ActionTicket, AnimationHistory},
};
mod update;

pub(crate) struct GraphWindow {
    context: GraphWindowContext,
    x_context: PropertyContext<Px>,
    y_context: PropertyContext<Px>,
    width_context: PropertyContext<Percent>,
    height_context: PropertyContext<Percent>,
    rotation_angle_context: PropertyContext<Angle>,
    opacity_context: PropertyContext<OneZero>,
    mouse_over_row: Option<MouseOverRow>,
    animation_history: AnimationHistory,
    dragging: Option<Dragging>,
    selected_point_address: Option<PointAddress>,
}

pub(crate) struct Props<'a> {
    pub wh: Wh<f32>,
    pub layer: Option<&'a namui::animation::Layer>,
    pub playback_time: Time,
}

#[derive(Debug, Clone)]
enum Dragging {
    Point {
        ticket: ActionTicket,
    },
    Background {
        property_name: PropertyName,
        last_mouse_local_xy: Xy<f32>,
    },
}

#[derive(Debug, Clone)]
struct MouseOverRow {
    property_name: PropertyName,
    mouse_xy_in_row: Xy<f32>,
}

#[derive(Debug, Clone)]
enum Event {
    GraphMouseMoveIn {
        property_name: PropertyName,
        mouse_xy_in_row: Xy<f32>,
    },
    GraphMouseMoveOut,
    GraphShiftMouseWheel {
        delta: Px,
    },
    GraphAltMouseWheel {
        delta: Px,
        mouse_local_xy: Xy<f32>,
    },
    GraphCtrlMouseWheel {
        delta: Px,
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
        row_wh: Wh<f32>,
    },
    GraphMouseWheel {
        delta: Px,
        property_name: PropertyName,
    },
    GraphPointMouseDown {
        point_address: PointAddress,
        row_height: Px,
        y_in_row: Px,
    },
    GraphMouseLeftDown {
        property_name: PropertyName,
        mouse_local_xy: Xy<f32>,
    },
    KeyboardKeyDown {
        code: Code,
        row_height: f32,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointAddress {
    layer_id: String,
    property_name: PropertyName,
    point_id: String,
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
    pub time_per_px: TimePerPx,
}

impl GraphWindow {
    pub(crate) fn new(animation_history: AnimationHistory) -> Self {
        Self {
            context: GraphWindowContext {
                start_at: Time::Ms(0.0),
                time_per_px: Time::Ms(50.0) / Px::from(1.0_f32),
            },
            x_context: PropertyContext {
                px_zero_to_bottom: Px::from(-20.0_f32),
                value_per_px: ValuePerPx {
                    value: Px::from(10.0_f32),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|&x| Px::from(x as f32))
                    .collect(),
                gradation_px_range: Px::from(15.0_f32)..Px::from(30.0_f32),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 1.0,
                    max: 100.0,
                }),
            },
            y_context: PropertyContext {
                px_zero_to_bottom: Px::from(-20.0_f32),
                value_per_px: ValuePerPx {
                    value: Px::from(10.0_f32),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|&x| Px::from(x as f32))
                    .collect(),
                gradation_px_range: Px::from(15.0_f32)..Px::from(30.0_f32),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 1.0,
                    max: 100.0,
                }),
            },
            width_context: PropertyContext {
                px_zero_to_bottom: Px::from(-20.0_f32),
                value_per_px: ValuePerPx {
                    value: Percent::from_percent(5.0_f32),
                },
                gradation_value_candidates: [1, 2, 5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|x: &i32| Percent::from_percent(*x))
                    .collect(),
                gradation_px_range: Px::from(15.0_f32)..Px::from(30.0_f32),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 0.001,
                    max: 0.1,
                }),
            },
            height_context: PropertyContext {
                px_zero_to_bottom: Px::from(-20.0_f32),
                value_per_px: ValuePerPx {
                    value: Percent::from_percent(5.0_f32),
                },
                gradation_value_candidates: [1, 2, 5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|x: &i32| Percent::from_percent(*x))
                    .collect(),
                gradation_px_range: Px::from(15.0_f32)..Px::from(30.0_f32),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 0.001,
                    max: 0.1,
                }),
            },
            rotation_angle_context: PropertyContext {
                px_zero_to_bottom: Px::from(-20.0_f32),
                value_per_px: ValuePerPx {
                    value: Angle::Degree(1.0),
                },
                gradation_value_candidates: [5, 10, 15, 30, 60, 90, 360]
                    .iter()
                    .map(|&x| Angle::Degree(x as f32))
                    .collect(),
                gradation_px_range: Px::from(15.0_f32)..Px::from(30.0_f32),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 1.0,
                    max: 100.0,
                }),
            },
            opacity_context: PropertyContext {
                px_zero_to_bottom: Px::from(-20.0_f32),
                value_per_px: ValuePerPx {
                    value: OneZero::from(0.01_f32),
                },
                gradation_value_candidates: [0.1]
                    .iter()
                    .map(|&x| OneZero::from(x as f32))
                    .collect(),
                gradation_px_range: Px::from(15.0_f32)..Px::from(30.0_f32),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 0.005,
                    max: 0.1,
                }),
            },
            mouse_over_row: None,
            animation_history,
            dragging: None,
            selected_point_address: None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ValuePerPx<TValue> {
    value: TValue,
}

impl<TValue: ToPrimitive + FromPrimitive> std::ops::Mul<Px> for ValuePerPx<TValue> {
    type Output = TValue;

    fn mul(self, rhs: Px) -> Self::Output {
        (&self).mul(rhs)
    }
}
impl<TValue: ToPrimitive + FromPrimitive> std::ops::Mul<Px> for &'_ ValuePerPx<TValue> {
    type Output = TValue;

    fn mul(self, rhs: Px) -> Self::Output {
        TValue::from_f32(self.value.to_f32().unwrap() * (rhs / Px::from(1.0f32))).unwrap()
    }
}
impl<TValue: ToPrimitive + Copy> ValuePerPx<TValue> {
    fn get_px(&self, value: TValue) -> Px {
        let value: f32 = value.to_f32().unwrap();
        let self_value: f32 = self.value.to_f32().unwrap();
        Px::from(value / self_value)
    }
}
impl<TValue: ToPrimitive> Div for ValuePerPx<TValue> {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.value.to_f32().unwrap() / rhs.value.to_f32().unwrap()
    }
}

struct Context<'a, TValue> {
    start_at: Time,
    time_per_px: TimePerPx,
    property_context: &'a PropertyContext<TValue>,
    mouse_local_xy: Option<Xy<f32>>,
    property_name: PropertyName,
    selected_point_id: Option<String>,
    layer: &'a Layer,
}

#[derive(Debug, Clone)]
struct PropertyContext<TValue> {
    value_per_px: ValuePerPx<TValue>,
    px_zero_to_bottom: Px,
    gradation_value_candidates: Box<[TValue]>,
    gradation_px_range: Range<Px>,
    zoom: Arc<dyn Zoom<TValue>>,
}

impl<TValue: ToPrimitive + FromPrimitive + Copy> PropertyContext<TValue> {
    fn get_value_on_y(&self, row_height: Px, y: Px) -> TValue {
        self.value_per_px * (row_height - y + self.px_zero_to_bottom)
    }
    fn get_value_at_bottom(&self, row_height: Px) -> TValue {
        self.get_value_on_y(row_height, row_height)
    }
    fn get_value_at_top(&self, row_height: Px) -> TValue {
        self.get_value_on_y(row_height, 0.0.into())
    }
}

// const STEP: f32 = 400.0;
// const MIN: f32 = 1.0;
// const MAX: f32 = 100.0;
trait Zoom<TValue> {
    fn zoom(&self, target: ValuePerPx<TValue>, delta: f32) -> ValuePerPx<TValue>;
}

impl<TValue> Debug for dyn Zoom<TValue> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Zoom")
    }
}

struct F32BasedZoom {
    pub step: f32,
    pub min: f32,
    pub max: f32,
}

impl<TValue: ToPrimitive + FromPrimitive + Copy> Zoom<TValue> for F32BasedZoom {
    fn zoom(&self, target: ValuePerPx<TValue>, delta: f32) -> ValuePerPx<TValue> {
        let wheel = self.step * (target.value.to_f32().unwrap() / 10.0).log2();

        let next_wheel = wheel + delta;

        let zoomed = namui::math::num::clamp(
            10.0 * 2.0f32.powf(next_wheel / self.step),
            self.min,
            self.max,
        );
        namui::log!("wheel: {}, delta: {}, zoomed: {}", wheel, delta, zoomed);
        ValuePerPx {
            value: TValue::from_f32(zoomed).unwrap(),
        }
    }
}
