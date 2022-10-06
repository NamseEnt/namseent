use super::*;
use crate::namui::render::Matrix3x3;
use std::{
    f32::consts::PI,
    ops::{Add, Mul},
};

pub type ImageKeyframeGraph = KeyframeGraph<ImageKeyframe, ImageInterpolation>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageKeyframe {
    pub matrix: Matrix3x3,
    pub opacity: OneZero,
}

impl ImageKeyframe {
    pub fn x(&self) -> Px {
        self.matrix.index_0_2().px()
    }
    pub fn y(&self) -> Px {
        self.matrix.index_1_2().px()
    }
    pub fn rotation_angle(&self) -> Angle {
        let radian = self.matrix.index_1_0().atan2(self.matrix.index_0_0());
        Angle::Radian(radian)
    }
    pub fn width_percent(&self) -> Percent {
        Percent::from(self.matrix.index_0_0())
    }
    pub fn height_percent(&self) -> Percent {
        Percent::from(self.matrix.index_1_1())
    }
    pub fn set_x(&mut self, x: Px) {
        self.matrix.set_index_0_2(x.as_f32());
    }
    pub fn set_y(&mut self, y: Px) {
        self.matrix.set_index_1_2(y.as_f32());
    }
    pub fn set_rotation_angle(&mut self, angle: Angle) {
        let radian = angle.as_radians();
        self.matrix.set_index_0_0(radian.cos());
        self.matrix.set_index_0_1(-radian.sin());
        self.matrix.set_index_1_0(radian.sin());
        self.matrix.set_index_1_1(radian.cos());
    }
    pub fn set_width_percent(&mut self, width_percent: Percent) {
        self.matrix.set_index_0_0(width_percent.as_f32());
    }
    pub fn set_height_percent(&mut self, height_percent: Percent) {
        self.matrix.set_index_1_1(height_percent.as_f32());
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    serde::Serialize,
    serde::Deserialize,
    strum_macros::EnumIter,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
)]
pub enum ImageInterpolation {
    AllLinear,
    SquashAndStretch { frame_per_second: f32 },
}

impl ImageInterpolation {
    pub fn iter() -> impl Iterator<Item = ImageInterpolation> {
        fn generic_iter<E>() -> impl Iterator<Item = E>
        where
            E: strum::IntoEnumIterator,
        {
            E::iter()
        }
        generic_iter::<ImageInterpolation>()
    }
}

impl KeyframeValue<ImageInterpolation> for ImageKeyframe {
    fn interpolate(&self, next: &Self, context: &InterpolationContext<ImageInterpolation>) -> Self {
        match context.line {
            ImageInterpolation::AllLinear => Self {
                matrix: linear_interpolate(&self.matrix, &next.matrix, context.time_ratio),
                opacity: linear_interpolate(&self.opacity, &next.opacity, context.time_ratio),
            },
            &ImageInterpolation::SquashAndStretch { frame_per_second } => {
                let time_ratio = get_time_ratio_in_fps(&context, frame_per_second);
                fn get_position_of_time_ratio(time_ratio: f32, length: Px) -> Px {
                    length / 2.0 * (1.0 - (PI * time_ratio).cos())
                }
                let vector = crate::Xy {
                    x: next.x() - self.x(),
                    y: next.y() - self.y(),
                };

                let x = self.x() + get_position_of_time_ratio(time_ratio, vector.x);

                let y = self.y() + get_position_of_time_ratio(time_ratio, vector.y);

                let angle = vector.atan2();

                let mut matrix = Matrix3x3::identity();

                matrix.rotate(-angle);

                let velocity = {
                    let length = vector.length();

                    let vt = length * (PI / 2.0) * (PI * time_ratio).sin();
                    vt
                };
                let sx = (velocity.as_f32() / 1000.0).min(1.25).max(1.0);
                let sy = 1.0 / sx;
                matrix.scale(sx, sy);

                matrix.rotate(angle);

                matrix.translate(x.as_f32(), y.as_f32());

                Self {
                    matrix,
                    opacity: linear_interpolate(&self.opacity, &next.opacity, time_ratio),
                }
            }
        }
    }
}

fn get_time_ratio_in_fps(context: &InterpolationContext<ImageInterpolation>, fps: f32) -> f32 {
    let time_ratio_per_frame = 1.0 / (context.duration.as_seconds() * fps);
    let rest = context.time_ratio % time_ratio_per_frame;
    let time_ratio = context.time_ratio - rest;
    time_ratio
}

fn linear_interpolate<'a, TValue>(a: &'a TValue, b: &'a TValue, time_ratio: f32) -> TValue
where
    TValue: 'a + Add<Output = TValue>,
    &'a TValue: Mul<f32, Output = TValue>,
{
    a * (1.0 - time_ratio) + b * time_ratio
}
