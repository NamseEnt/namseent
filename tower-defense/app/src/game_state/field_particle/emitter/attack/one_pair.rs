use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{self, NewAttackParticle},
};
use namui::*;
use rand::Rng;

pub(super) fn spawn(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) {
    let tile_size = crate::game_state::TILE_PX_SIZE;
    let delta_x = end_xy.0 - start_xy.0;
    let delta_y = end_xy.1 - start_xy.1;
    let delta_x_px = delta_x * tile_size.width.as_f32();
    let delta_y_px = delta_y * tile_size.height.as_f32();
    let length_px = delta_x_px.hypot(delta_y_px);
    let angle = delta_y_px.atan2(delta_x_px);
    let rotation_cos = angle.cos();
    let rotation_sin = angle.sin();
    let trail_width_px = crate::atlas::generated::ATTACK_ONE_PAIR_TRAIL[0]
        .width()
        .as_f32();
    let spacing_px = trail_width_px
        * attack::one_pair::Trail::INITIAL_SCALE
        * attack::one_pair::Trail::SPACING_RATIO;
    let trail_count = if length_px <= f32::EPSILON {
        1
    } else {
        (length_px / spacing_px).ceil() as usize + 1
    };

    ATTACK_VISUAL.spawn(NewAttackParticle::OnePairProjectile(
        attack::one_pair::Projectile::new(start_xy, end_xy, created_at),
    ));

    let mut rng = rand::thread_rng();
    let muzzle_ring_scale = rng.gen_range(0.9..=1.1);
    ATTACK_VISUAL.spawn(NewAttackParticle::OnePairMuzzleRing(
        attack::one_pair::MuzzleRing::new(
            start_xy,
            rotation_cos,
            rotation_sin,
            created_at,
            muzzle_ring_scale,
        ),
    ));

    for index in 0..trail_count {
        let progress = if trail_count == 1 {
            0.0
        } else {
            index as f32 / (trail_count - 1) as f32
        };
        let delay = Duration::from_millis(index as i64 * 3);
        let scale_variation = rng.gen_range(0.85..=1.15);
        ATTACK_VISUAL.spawn(NewAttackParticle::OnePairTrail(
            attack::one_pair::Trail::new(
                start_xy,
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
}
