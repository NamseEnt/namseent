use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{self, NewAttackParticle},
};
use namui::*;
use rand::Rng;

const TRAIL_RING_SPACING_TILES: f32 = 1.5;

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let mut rng = rand::thread_rng();
    let tile_size = crate::game_state::TILE_PX_SIZE;
    let trail_width_px = crate::atlas::generated::ATTACK_TWO_PAIR_TRAIL[0]
        .width()
        .as_f32();
    let trail_spacing_px = trail_width_px
        * attack::two_pair::Trail::INITIAL_SCALE
        * attack::two_pair::Trail::SPACING_RATIO;

    for bundle_index in 0..2 {
        let bundle_delay = Duration::from_millis(bundle_index * 75);
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let radius = rng.gen_range(0.25_f32..0.5).sqrt();
        let projectile_start_xy = (
            start_xy.0 + radius * angle.cos(),
            start_xy.1 + radius * angle.sin(),
        );
        let delta_x = end_xy.0 - projectile_start_xy.0;
        let delta_y = end_xy.1 - projectile_start_xy.1;
        let delta_x_px = delta_x * tile_size.width.as_f32();
        let delta_y_px = delta_y * tile_size.height.as_f32();
        let length_px = delta_x_px.hypot(delta_y_px);
        let length_tiles = delta_x.hypot(delta_y);
        let angle = delta_y_px.atan2(delta_x_px);
        let rotation_cos = angle.cos();
        let rotation_sin = angle.sin();
        let trail_count = if length_px <= f32::EPSILON {
            1
        } else {
            (length_px / trail_spacing_px).ceil() as usize + 1
        };

        ATTACK_VISUAL.spawn(NewAttackParticle::TwoPairProjectile(
            attack::two_pair::Projectile::new(projectile_start_xy, end_xy, created_at)
                .with_start_delay(bundle_delay),
        ));

        let muzzle_ring_scale = rng.gen_range(0.9..=1.1);
        ATTACK_VISUAL.spawn(NewAttackParticle::TwoPairMuzzleRing(
            attack::two_pair::MuzzleRing::new(
                projectile_start_xy,
                rotation_cos,
                rotation_sin,
                created_at,
                muzzle_ring_scale,
            )
            .with_start_delay(bundle_delay),
        ));

        for index in 0..trail_count {
            let progress = if trail_count == 1 {
                0.0
            } else {
                index as f32 / (trail_count - 1) as f32
            };
            let delay = bundle_delay + Duration::from_millis(index as i64 * 10);
            let scale_variation = rng.gen_range(0.5..=2.0);
            ATTACK_VISUAL.spawn(NewAttackParticle::TwoPairTrail(
                attack::two_pair::Trail::new(
                    projectile_start_xy,
                    length_px,
                    progress,
                    rotation_cos,
                    rotation_sin,
                    created_at,
                    delay,
                )
                .with_scale_variation(scale_variation),
            ));
        }

        let trail_ring_count = (length_tiles / TRAIL_RING_SPACING_TILES).ceil() as usize;
        for index in 0..trail_ring_count {
            let progress = (index as f32 + rng.gen_range(0.0..1.0)) / trail_ring_count as f32;
            let ring_center_xy = (
                projectile_start_xy.0 + delta_x * progress,
                projectile_start_xy.1 + delta_y * progress,
            );
            let delay = bundle_delay + Duration::from_millis(index as i64 * 3);
            let scale_variation = rng.gen_range(0.75..=1.25);
            let duration_variation = rng.gen_range(0.8..=1.2);
            ATTACK_VISUAL.spawn(NewAttackParticle::TwoPairTrailRing(
                attack::two_pair::TrailRing::new(
                    ring_center_xy,
                    rotation_cos,
                    rotation_sin,
                    created_at,
                    delay,
                    scale_variation,
                    duration_variation,
                ),
            ));
        }
    }
}
