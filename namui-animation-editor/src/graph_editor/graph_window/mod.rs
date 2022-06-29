use namui::{
    animation::{KeyframeGraph, Layer},
    math::num::{FromPrimitive, ToPrimitive},
    prelude::*,
    types::{Degree, OneZero, Percent, PixelSize, Time, TimePerPixel},
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
    id: String,
    context: GraphWindowContext,
    x_context: PropertyContext<PixelSize>,
    y_context: PropertyContext<PixelSize>,
    width_context: PropertyContext<Percent>,
    height_context: PropertyContext<Percent>,
    rotation_angle_context: PropertyContext<Degree>,
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
        point_address: PointAddress,
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
    GraphMouseWheel {
        delta: PixelSize,
        property_name: PropertyName,
    },
    GraphPointMouseDown {
        point_address: PointAddress,
        row_height: PixelSize,
        y_in_row: PixelSize,
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
    pub time_per_pixel: TimePerPixel,
}

impl GraphWindow {
    pub(crate) fn new(animation_history: AnimationHistory) -> Self {
        Self {
            id: namui::nanoid(),
            context: GraphWindowContext {
                start_at: Time::zero(),
                time_per_pixel: Time::from_ms(50.0) / PixelSize::from(1.0),
            },
            x_context: PropertyContext {
                pixel_size_zero_to_bottom: PixelSize::from(-20.0),
                value_per_pixel: ValuePerPixel {
                    value: PixelSize::from(10.0),
                    pixel_size: PixelSize::from(1.0),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|&x| PixelSize::from(x as f32))
                    .collect(),
                gradation_pixel_size_range: PixelSize::from(15.0)..PixelSize::from(30.0),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 1.0,
                    max: 100.0,
                }),
            },
            y_context: PropertyContext {
                pixel_size_zero_to_bottom: PixelSize::from(-20.0),
                value_per_pixel: ValuePerPixel {
                    value: PixelSize::from(10.0),
                    pixel_size: PixelSize::from(1.0),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|&x| PixelSize::from(x as f32))
                    .collect(),
                gradation_pixel_size_range: PixelSize::from(15.0)..PixelSize::from(30.0),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 1.0,
                    max: 100.0,
                }),
            },
            width_context: PropertyContext {
                pixel_size_zero_to_bottom: PixelSize::from(-20.0),
                value_per_pixel: ValuePerPixel {
                    value: Percent::new(5.0_f32),
                    pixel_size: PixelSize::from(1.0),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|x: &i32| Percent::new(*x))
                    .collect(),
                gradation_pixel_size_range: PixelSize::from(15.0)..PixelSize::from(30.0),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 1.0,
                    max: 100.0,
                }),
            },
            height_context: PropertyContext {
                pixel_size_zero_to_bottom: PixelSize::from(-20.0),
                value_per_pixel: ValuePerPixel {
                    value: Percent::new(5.0_f32),
                    pixel_size: PixelSize::from(1.0),
                },
                gradation_value_candidates: [5, 10, 25, 50, 100, 200, 500]
                    .iter()
                    .map(|x: &i32| Percent::new(*x))
                    .collect(),
                gradation_pixel_size_range: PixelSize::from(15.0)..PixelSize::from(30.0),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 1.0,
                    max: 100.0,
                }),
            },
            rotation_angle_context: PropertyContext {
                pixel_size_zero_to_bottom: PixelSize::from(-20.0),
                value_per_pixel: ValuePerPixel {
                    value: Degree::from(1.0),
                    pixel_size: PixelSize::from(1.0),
                },
                gradation_value_candidates: [5, 10, 15, 30, 60, 90, 360]
                    .iter()
                    .map(|&x| Degree::from(x as f32))
                    .collect(),
                gradation_pixel_size_range: PixelSize::from(15.0)..PixelSize::from(30.0),
                zoom: Arc::new(F32BasedZoom {
                    step: 400.0,
                    min: 1.0,
                    max: 100.0,
                }),
            },
            opacity_context: PropertyContext {
                pixel_size_zero_to_bottom: PixelSize::from(-20.0),
                value_per_pixel: ValuePerPixel {
                    value: OneZero::from(0.01),
                    pixel_size: PixelSize::from(1.0),
                },
                gradation_value_candidates: [0.1]
                    .iter()
                    .map(|&x| OneZero::from(x as f32))
                    .collect(),
                gradation_pixel_size_range: PixelSize::from(15.0)..PixelSize::from(30.0),
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
struct ValuePerPixel<TValue> {
    value: TValue,
    pixel_size: PixelSize,
}

impl<TValue: ToPrimitive + FromPrimitive> std::ops::Mul<PixelSize> for ValuePerPixel<TValue> {
    type Output = TValue;

    fn mul(self, rhs: PixelSize) -> Self::Output {
        (&self).mul(rhs)
    }
}
impl<TValue: ToPrimitive + FromPrimitive> std::ops::Mul<PixelSize> for &'_ ValuePerPixel<TValue> {
    type Output = TValue;

    fn mul(self, rhs: PixelSize) -> Self::Output {
        TValue::from_f32(self.value.to_f32().unwrap() * (rhs / self.pixel_size)).unwrap()
    }
}
impl<TValue: ToPrimitive + Copy> ValuePerPixel<TValue> {
    fn get_pixel_size(&self, value: TValue) -> PixelSize {
        let value: f32 = value.to_f32().unwrap();
        let self_value: f32 = self.value.to_f32().unwrap();
        (value / self_value) * self.pixel_size
    }
}
impl<TValue: ToPrimitive> Div for ValuePerPixel<TValue> {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        (self.value.to_f32().unwrap() / Into::<f32>::into(self.pixel_size))
            / (rhs.value.to_f32().unwrap() / Into::<f32>::into(rhs.pixel_size))
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
    pixel_size_zero_to_bottom: PixelSize,
    gradation_value_candidates: Box<[TValue]>,
    gradation_pixel_size_range: Range<PixelSize>,
    zoom: Arc<dyn Zoom<TValue>>,
}

impl<TValue: ToPrimitive + FromPrimitive + Copy> PropertyContext<TValue> {
    fn get_value_on_y(&self, row_height: PixelSize, y: PixelSize) -> TValue {
        self.value_per_pixel * (row_height - y + self.pixel_size_zero_to_bottom)
    }
    fn get_value_at_bottom(&self, row_height: PixelSize) -> TValue {
        self.get_value_on_y(row_height, row_height)
    }
    fn get_value_at_top(&self, row_height: PixelSize) -> TValue {
        self.get_value_on_y(row_height, 0.0.into())
    }
}

// const STEP: f32 = 400.0;
// const MIN: f32 = 1.0;
// const MAX: f32 = 100.0;
trait Zoom<TValue> {
    fn zoom(&self, target: ValuePerPixel<TValue>, delta: f32) -> ValuePerPixel<TValue>;
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
    fn zoom(&self, target: ValuePerPixel<TValue>, delta: f32) -> ValuePerPixel<TValue> {
        let wheel = self.step
            * (target.value.to_f32().unwrap() / f32::from(target.pixel_size) / 10.0).log2();

        let next_wheel = wheel + delta;

        let zoomed = namui::math::num::clamp(
            10.0 * 2.0f32.powf(next_wheel / self.step),
            self.min,
            self.max,
        );

        ValuePerPixel {
            value: TValue::from_f32(zoomed).unwrap(),
            pixel_size: 1.0_f32.into(),
        }
    }
}
