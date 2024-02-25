use super::mechanics::{Acceleration, Meter, Speed};
use namui::{math::num::pow::Pow, prelude::*};

pub const GRAVITY: Acceleration = Acceleration(9.8);

pub fn range(start_speed: Speed, xz_angle: Angle) -> Meter {
    xz_angle.sin() * xz_angle.cos() * 2.0 * (start_speed * (start_speed / GRAVITY))
}

pub fn calculate_launch_angle(start_speed: Speed, distance: Meter) -> Angle {
    let double_start_velocity = (start_speed * 1.sec()).as_f32().pow(2);
    Angle::Radian(f32::asin(GRAVITY.as_f32() * distance.as_f32() / double_start_velocity) / 2.0)
}
