use namui::{math::num::pow::Pow, prelude::*};

pub const GRAVITY: Px = px(40.0);

pub fn range(start_velocity: Per<Px, Duration>, xz_angle: Angle) -> Px {
    let start_velocity = start_velocity * 1.sec();
    start_velocity * start_velocity.as_f32() * xz_angle.sin() * xz_angle.cos() * 2.0
        / GRAVITY.as_f32()
}

pub fn calculate_launch_angle(start_velocity: Per<Px, Duration>, distance: Px) -> Angle {
    let double_start_velocity = (start_velocity * 1.sec()).as_f32().pow(2);
    Angle::Radian(f32::asin(GRAVITY.as_f32() * distance.as_f32() / double_start_velocity) / 2.0)
}
