use super::*;
use crate::namui::render::Matrix3x3;
use std::ops::{Add, Mul};

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
    /// velocity ratio * L / T = v0
    SquashAndStretch {
        velocity_ratio: f32,
    },
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
    fn interpolate(&self, next: &Self, time_ratio: f32, line: &ImageInterpolation) -> Self {
        match line {
            ImageInterpolation::AllLinear => Self {
                matrix: linear_interpolate(&self.matrix, &next.matrix, time_ratio),
                opacity: linear_interpolate(&self.opacity, &next.opacity, time_ratio),
            },
            &ImageInterpolation::SquashAndStretch { velocity_ratio } => {
                fn get_position_of_time_ratio(
                    time_ratio: f32,
                    velocity_ratio: f32,
                    length: Px,
                ) -> Px {
                    length
                        * ((1.0 - velocity_ratio) * time_ratio.powf(2.0)
                            + velocity_ratio * time_ratio)
                }

                let x = self.x()
                    + get_position_of_time_ratio(time_ratio, velocity_ratio, next.x() - self.x());

                let y = self.y()
                    + get_position_of_time_ratio(time_ratio, velocity_ratio, next.y() - self.y());

                // let velocity = {
                //     let length = crate::Xy {
                //         x: next.x() - self.x(),
                //         y: next.y() - self.y(),
                //     }
                //     .length();

                //     let v0 = velocity_ratio * length;
                //     let accel = 2.0 * length * (1.0 - velocity_ratio);

                //     let vt = v0 + accel * time_ratio;
                //     vt
                // };

                Self {
                    matrix: Matrix3x3::translate(x.as_f32(), y.as_f32()),
                    opacity: linear_interpolate(&self.opacity, &next.opacity, time_ratio),
                }
            }
        }
    }
}

fn linear_interpolate<'a, TValue>(a: &'a TValue, b: &'a TValue, time_ratio: f32) -> TValue
where
    TValue: 'a + Add<Output = TValue>,
    &'a TValue: Mul<f32, Output = TValue>,
{
    a * (1.0 - time_ratio) + b * time_ratio
}
