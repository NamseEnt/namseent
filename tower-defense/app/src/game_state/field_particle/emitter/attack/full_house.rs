use super::common::ember;
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{NewAttackParticle, common},
};
use namui::*;
use rand::Rng;
use rand::seq::SliceRandom;

const ORANGE: [f32; 3] = [1.0, 0.52, 0.12];
const PALE_BROWN: [f32; 3] = [0.62, 0.44, 0.3];
const SKY_ANGLE: f32 = -std::f32::consts::FRAC_PI_2;
const INITIAL_PROJECTILE_COUNT: usize = 8;
const TRAIL_SEGMENT_COUNT: usize = 12;
const IMPACT_FLASH_COUNT: usize = 12;
const SMOKE_ANGLE_VARIATION: f32 = std::f32::consts::PI / 12.0;
const LAUNCH_INTERVAL: Duration = Duration::from_millis(5);

pub(super) fn spawn(
    start_xy: (f32, f32),
    center_xy: (f32, f32),
    attack_range_radius: f32,
    created_at: Instant,
) {
    let mut rng = rand::thread_rng();
    let impact_count = (attack_range_radius.max(0.0) * 2.0)
        .ceil()
        .max(INITIAL_PROJECTILE_COUNT as f32) as usize;
    let mut radial_bins = (0..impact_count).collect::<Vec<_>>();
    radial_bins.shuffle(&mut rng);
    for (index, radial_bin) in radial_bins.into_iter().enumerate() {
        let angle = std::f32::consts::TAU * (index as f32 + rng.gen_range(0.0_f32..1.0))
            / impact_count as f32;
        let radial_progress =
            (radial_bin as f32 + rng.gen_range(0.0_f32..1.0)) / impact_count as f32;
        let radius = radial_progress.sqrt() * attack_range_radius.max(0.0);
        let impact_xy = (
            center_xy.0 + angle.cos() * radius,
            center_xy.1 + angle.sin() * radius,
        );
        let delay = LAUNCH_INTERVAL * index as i32;
        let duration = Duration::from_millis(rng.gen_range(75_i64..=125));
        let scale = rng.gen_range(0.5_f32..=1.5);
        let launch_angle = common::FullHouseProjectile::path_angle(start_xy, impact_xy, 0.0);
        ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalFlash(
            common::DirectionalFlash::new(start_xy, launch_angle, 2.0, ORANGE, created_at)
                .with_delay(delay),
        ));
        ATTACK_VISUAL.spawn(NewAttackParticle::FullHouseProjectile(
            common::FullHouseProjectile::new(
                start_xy, impact_xy, created_at, delay, duration, scale,
            ),
        ));
        for trail_index in 1..=TRAIL_SEGMENT_COUNT {
            let path_progress = trail_index as f32 / (TRAIL_SEGMENT_COUNT + 1) as f32;
            let time_progress =
                common::FullHouseProjectile::time_progress_for_path_progress(path_progress);
            let trail_xy =
                common::FullHouseProjectile::path_position(start_xy, impact_xy, path_progress);
            let trail_angle =
                common::FullHouseProjectile::path_angle(start_xy, impact_xy, path_progress);
            let trail_delay = delay
                + Duration::from_millis(
                    (duration.as_secs_f32() * 1_000.0 * time_progress) as i64 + 1,
                );
            ATTACK_VISUAL.spawn(NewAttackParticle::BulletTrail(common::BulletTrail::new(
                trail_xy,
                trail_angle.cos(),
                trail_angle.sin(),
                created_at,
                trail_delay,
                0.35,
                0.5,
            )));
        }

        let impact_delay = delay + duration;
        for flash_index in 0..IMPACT_FLASH_COUNT {
            let flash_angle =
                std::f32::consts::TAU * flash_index as f32 / IMPACT_FLASH_COUNT as f32;
            let flash_scale = rng.gen_range(0.5_f32..=1.0) * 4.0;
            ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalFlash(
                common::DirectionalFlash::new(
                    impact_xy,
                    flash_angle,
                    flash_scale,
                    ORANGE,
                    created_at,
                )
                .with_delay(impact_delay),
            ));
        }
        ember::spawn_from_point_with_delay(impact_xy, SKY_ANGLE, ORANGE, created_at, impact_delay);
        for smoke_index in 0..3 {
            let smoke_delay = impact_delay + Duration::from_millis(smoke_index as i64 * 25);
            let scale_offset = smoke_index as f32 * 0.1;
            let smoke_angle =
                SKY_ANGLE + rng.gen_range(-SMOKE_ANGLE_VARIATION..=SMOKE_ANGLE_VARIATION);
            let smoke_direction = (smoke_angle.cos(), smoke_angle.sin());
            ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalVolumeSmoke(
                common::DirectionalVolumeSmoke::new(
                    impact_xy,
                    smoke_direction,
                    1.5,
                    smoke_angle,
                    common::DirectionalVolumeSmokeOptions {
                        delay: smoke_delay,
                        start_color: Some(ORANGE),
                        end_color: Some(PALE_BROWN),
                        start_scale: Some(1.4 + scale_offset),
                        end_scale: Some(3.6 + scale_offset * 2.0),
                    },
                    created_at,
                ),
            ));
        }
        let up = (0.0, -1.0);
        ATTACK_VISUAL.spawn(NewAttackParticle::VaporRing(
            common::VaporRing::new(impact_xy, created_at, impact_delay, 1.8, up, 0.8)
                .with_color(PALE_BROWN)
                .with_scale_growth(5.0),
        ));
    }
}

pub(super) fn spawn_hit_effect(impact_xy: (f32, f32), created_at: Instant) {
    ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalFlash(
        common::DirectionalFlash::new(impact_xy, SKY_ANGLE, 2.0, ORANGE, created_at),
    ));
    ember::spawn_from_point_with_delay(impact_xy, SKY_ANGLE, ORANGE, created_at, Duration::ZERO);
}
