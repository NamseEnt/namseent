use super::*;
use std::ops::{Add, Mul};

pub type ImageKeyframeGraph = KeyframeGraph<ImageKeyframe, ImageInterpolation>;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageKeyframe {
    pub x: Px,
    pub y: Px,
    pub width_percent: Percent,
    pub height_percent: Percent,
    pub rotation_angle: Angle,
    pub opacity: OneZero,
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
                x: linear_interpolate(&self.x, &next.x, time_ratio),
                y: linear_interpolate(&self.y, &next.y, time_ratio),
                width_percent: linear_interpolate(
                    &self.width_percent,
                    &next.width_percent,
                    time_ratio,
                ),
                height_percent: linear_interpolate(
                    &self.height_percent,
                    &next.height_percent,
                    time_ratio,
                ),
                rotation_angle: linear_interpolate(
                    &self.rotation_angle,
                    &next.rotation_angle,
                    time_ratio,
                ),
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

                let x = self.x
                    + get_position_of_time_ratio(time_ratio, velocity_ratio, next.x - self.x);

                let y = self.y
                    + get_position_of_time_ratio(time_ratio, velocity_ratio, next.y - self.y);

                Self {
                    x,
                    y,
                    width_percent: linear_interpolate(
                        &self.width_percent,
                        &next.width_percent,
                        time_ratio,
                    ),
                    height_percent: linear_interpolate(
                        &self.height_percent,
                        &next.height_percent,
                        time_ratio,
                    ),
                    rotation_angle: linear_interpolate(
                        &self.rotation_angle,
                        &next.rotation_angle,
                        time_ratio,
                    ),
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
