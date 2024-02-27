use super::mechanics::{Acceleration, Meter, Speed};
use namui::prelude::*;
use num_traits::{Float, Signed};

pub const GRAVITY: Acceleration = Acceleration(9.8);

pub fn calculate_range_with_xz_angle(projectile_speed: Speed, xz_angle: Angle) -> Meter {
    xz_angle.sin() * xz_angle.cos() * 2.0 * (projectile_speed * (projectile_speed / GRAVITY))
}

pub fn calculate_launch_angle_with_distance(projectile_speed: Speed, distance: Meter) -> Angle {
    let double_start_velocity = projectile_speed.powi(2);
    Angle::Radian(
        f32::asin(GRAVITY.as_f32() * distance.as_f32() / double_start_velocity.as_f32()) / 2.0,
    )
}

pub fn calculate_launch_angle_with_flight_time(
    projectile_speed: Speed,
    flight_time: Duration,
) -> Angle {
    Angle::Radian(
        (GRAVITY * flight_time / projectile_speed / 2.0)
            .asin()
            .as_f32(),
    )
}

pub fn calculate_flight_time_with_launch_angle(
    projectile_speed: Speed,
    xz_angle: Angle,
) -> Duration {
    2.0 * projectile_speed * xz_angle.sin() / GRAVITY
}

// TODO: maybe use a math for this
pub struct AimMovingTarget {
    pub projectile_speed: Speed,
    pub start_xy: Xy<Meter>,
    pub target_xy: Xy<Meter>,
    pub target_speed: Speed,
    pub target_yaw: Angle,
}
impl AimMovingTarget {
    pub fn aim(&self) -> (Xy<f32>, Angle) {
        let Self {
            projectile_speed,
            start_xy,
            ..
        } = self;

        let max_flight_time = calculate_flight_time_with_launch_angle(*projectile_speed, 45.deg());

        let mut min = 0.sec();
        let mut max = max_flight_time;
        let mut mid = max_flight_time / 2;

        loop {
            let (target_xy, xz_angle, xy_diff) = self.aim_at(mid);
            if max - min < 0.3.sec() {
                break ((target_xy - start_xy).normalize().into_type(), xz_angle);
            }
            if xy_diff.is_positive() {
                max = mid;
                mid = (min + max) / 2;
            } else {
                min = mid;
                mid = (min + max) / 2;
            }
        }
    }
    fn aim_at(&self, flight_time: Duration) -> (Xy<Meter>, Angle, Meter) {
        let Self {
            projectile_speed,
            start_xy,
            target_xy,
            target_speed,
            target_yaw,
        } = self;

        let target_xy = *target_xy + target_yaw.as_xy() * Xy::single(*target_speed * flight_time);
        let distance_to_target = (target_xy - *start_xy).length();
        let xz_angle = calculate_launch_angle_with_flight_time(*projectile_speed, flight_time);
        let distance = calculate_range_with_xz_angle(*projectile_speed, xz_angle);
        let xy_diff = distance - distance_to_target;
        (target_xy, xz_angle, xy_diff)
    }
}
