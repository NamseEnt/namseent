use super::common::{ember, energy_ring, muzzle_flash};
use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{NewAttackParticle, common},
};
use namui::*;
use rand::Rng;

const FLUSH_COLOR: [f32; 3] = [1.0, 0.5, 0.85];
const PROJECTILE_DURATION: Duration = Duration::from_millis(100);
const EXPLOSION_DELAY: Duration = Duration::from_millis(85);
const HIT_DELAY: Duration = Duration::from_millis(50);
const EXPLOSION_FLASH_COUNT: usize = 12;
const EXPLOSION_SMOKE_COUNT: usize = 8;
const TRAIL_EMBER_ANGLE_VARIATION: f32 = std::f32::consts::PI / 12.0;
const EXPLOSION_SMOKE_LAYER_RADII_TILES: [f32; 2] = [0.3, 0.7];
const EXPLOSION_SMOKE_ANGLE_JITTER: f32 = std::f32::consts::PI / 36.0;
const EXPLOSION_SMOKE_RADIUS_JITTER_TILES: f32 = 0.12;

pub(super) fn spawn(start_xy: (f32, f32), target_xy: (f32, f32), created_at: Instant) {
    let direction_x = target_xy.0 - start_xy.0;
    let direction_y = target_xy.1 - start_xy.1;
    let length = direction_x.hypot(direction_y);
    if length <= f32::EPSILON {
        return;
    }

    let angle = direction_y.atan2(direction_x);
    ATTACK_VISUAL.spawn(NewAttackParticle::EnergyBall(common::EnergyBall::new(
        start_xy,
        target_xy,
        FLUSH_COLOR,
        created_at,
    )));
    muzzle_flash::spawn_fan(start_xy, angle, FLUSH_COLOR, created_at);
    energy_ring::spawn_with_color(
        start_xy,
        target_xy,
        created_at,
        Duration::ZERO,
        Duration::from_millis(10),
        1.5,
        FLUSH_COLOR,
    );

    let mut rng = rand::thread_rng();
    let ember_count = (length * 2.0).ceil() as usize;
    for index in 0..ember_count {
        let progress = (index as f32 + rng.gen_range(0.0_f32..=1.0)) / ember_count as f32;
        let position = (
            start_xy.0 + direction_x * progress,
            start_xy.1 + direction_y * progress,
        );
        let direction = angle
            + std::f32::consts::PI
            + rng.gen_range(-TRAIL_EMBER_ANGLE_VARIATION..=TRAIL_EMBER_ANGLE_VARIATION);
        let distance = rng.gen_range(0.5_f32..=3.0);
        let scale = rng.gen_range(0.1_f32..=0.25);
        let delay = Duration::from_secs_f32(PROJECTILE_DURATION.as_secs_f32() * progress);
        ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalEmber(
            common::DirectionalEmber::new(
                position,
                direction,
                distance,
                scale,
                FLUSH_COLOR,
                created_at,
            )
            .with_delay(delay),
        ));
    }

    spawn_explosion(target_xy, created_at);
}

fn spawn_explosion(center_xy: (f32, f32), created_at: Instant) {
    let mut rng = rand::thread_rng();
    for index in 0..EXPLOSION_FLASH_COUNT {
        let angle = std::f32::consts::TAU * index as f32 / EXPLOSION_FLASH_COUNT as f32;
        let scale = rng.gen_range(3.0_f32..=5.5);
        ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalFlash(
            common::DirectionalFlash::new(center_xy, angle, scale, FLUSH_COLOR, created_at)
                .with_delay(EXPLOSION_DELAY),
        ));
    }

    for (layer_index, layer_radius) in EXPLOSION_SMOKE_LAYER_RADII_TILES.iter().enumerate() {
        for index in 0..EXPLOSION_SMOKE_COUNT {
            let angle = std::f32::consts::TAU * (index as f32 + layer_index as f32 * 0.5)
                / EXPLOSION_SMOKE_COUNT as f32
                + rng.gen_range(-EXPLOSION_SMOKE_ANGLE_JITTER..=EXPLOSION_SMOKE_ANGLE_JITTER);
            let direction = (angle.cos(), angle.sin());
            let radius = (layer_radius
                + rng.gen_range(
                    -EXPLOSION_SMOKE_RADIUS_JITTER_TILES..=EXPLOSION_SMOKE_RADIUS_JITTER_TILES,
                ))
            .max(0.0);
            let start_xy = (
                center_xy.0 + direction.0 * radius,
                center_xy.1 + direction.1 * radius,
            );
            let delay = EXPLOSION_DELAY + Duration::from_millis(rng.gen_range(0..=25));
            ATTACK_VISUAL.spawn(NewAttackParticle::DirectionalVolumeSmoke(
                common::DirectionalVolumeSmoke::new(
                    start_xy,
                    direction,
                    0.0,
                    angle,
                    common::DirectionalVolumeSmokeOptions {
                        delay,
                        start_color: Some(FLUSH_COLOR),
                        end_color: Some(FLUSH_COLOR),
                        start_scale: Some(1.5),
                        end_scale: Some(3.0),
                    },
                    created_at,
                ),
            ));
        }
    }

    for index in 0..3 {
        let delay = EXPLOSION_DELAY + Duration::from_millis(index * 25);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let rotation = (angle.cos(), angle.sin());
        ATTACK_VISUAL.spawn(NewAttackParticle::VaporRing(
            common::VaporRing::new(center_xy, created_at, delay, 2.0, rotation, 1.0)
                .with_color(FLUSH_COLOR)
                .with_scale_growth(6.0),
        ));
    }

    ember::spawn_radial(center_xy, FLUSH_COLOR, created_at, EXPLOSION_DELAY);
}

pub(super) fn spawn_hit_effect(
    explosion_xy: (f32, f32),
    impact_xy: (f32, f32),
    created_at: Instant,
) {
    let direction_x = impact_xy.0 - explosion_xy.0;
    let direction_y = impact_xy.1 - explosion_xy.1;
    let angle = if direction_x.hypot(direction_y) > f32::EPSILON {
        direction_y.atan2(direction_x)
    } else {
        -std::f32::consts::FRAC_PI_2
    };
    let direction = (angle.cos(), angle.sin());
    ATTACK_VISUAL.spawn(NewAttackParticle::VaporRing(
        common::VaporRing::new(impact_xy, created_at, HIT_DELAY, 0.8, direction, 1.0)
            .with_color(FLUSH_COLOR)
            .with_movement(direction, 32.0),
    ));
    ember::spawn_hit_direction(impact_xy, angle, FLUSH_COLOR, created_at, HIT_DELAY);
}
