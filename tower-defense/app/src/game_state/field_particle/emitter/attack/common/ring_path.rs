use crate::game_state::field_particle::{
    ATTACK_VISUAL,
    particle::attack::{
        NewAttackParticle,
        common::{EnergyRing, VaporRing},
    },
};
use namui::*;
use rand::Rng;

const SPACING_TILES: f32 = 1.5;
const OFFSET_JITTER_TILES: f32 = 0.25;
pub(super) const ROYAL_PATH_SCALE_VARIANCE: f32 = 0.1;

pub(super) enum RingKind {
    Vapor,
    Energy {
        color: Option<[f32; 3]>,
        ease_in_fade: bool,
        scale_variance: f32,
    },
}

pub(super) fn spawn(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    delay_offset: Duration,
    delay_step: Duration,
    scale: f32,
    kind: RingKind,
) {
    spawn_with_color(
        start_xy,
        end_xy,
        created_at,
        delay_offset,
        delay_step,
        scale,
        kind,
    );
}

pub(super) fn spawn_with_color(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    delay_offset: Duration,
    delay_step: Duration,
    scale: f32,
    kind: RingKind,
) {
    spawn_with_color_inner(
        start_xy,
        end_xy,
        created_at,
        delay_offset,
        delay_step,
        scale,
        kind,
    );
}

pub(super) fn spawn_with_color_and_ease_in_fade(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    delay_offset: Duration,
    delay_step: Duration,
    scale: f32,
    kind: RingKind,
) {
    spawn_with_color_inner(
        start_xy,
        end_xy,
        created_at,
        delay_offset,
        delay_step,
        scale,
        kind,
    );
}

fn spawn_with_color_inner(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    delay_offset: Duration,
    delay_step: Duration,
    scale: f32,
    kind: RingKind,
) {
    let delta_x = end_xy.0 - start_xy.0;
    let delta_y = end_xy.1 - start_xy.1;
    let path_length_tiles = delta_x.hypot(delta_y);
    let angle = (delta_y * crate::game_state::TILE_PX_SIZE.height.as_f32())
        .atan2(delta_x * crate::game_state::TILE_PX_SIZE.width.as_f32());
    let rotation = (angle.cos(), angle.sin());
    let step_count = (path_length_tiles / SPACING_TILES).ceil() as usize;
    let mut rng = rand::thread_rng();

    for index in 0..=step_count {
        let progress = if step_count == 0 {
            0.0
        } else {
            index as f32 / step_count as f32
        };
        let expected_distance = path_length_tiles * progress;
        let offset = if index == 0 || index == step_count {
            0.0
        } else {
            rng.gen_range(-OFFSET_JITTER_TILES..=OFFSET_JITTER_TILES)
        };
        let distance = (expected_distance + offset).clamp(0.0, path_length_tiles);
        let path_progress = if path_length_tiles <= f32::EPSILON {
            0.0
        } else {
            distance / path_length_tiles
        };
        let position_xy = (
            start_xy.0 + delta_x * path_progress,
            start_xy.1 + delta_y * path_progress,
        );
        let delay = delay_offset + delay_step * index as i32;
        let opacity = 1.0 - path_progress;
        match kind {
            RingKind::Vapor => {
                let ring = VaporRing::new(position_xy, created_at, delay, scale, rotation, opacity)
                    .with_movement(rotation, 64.0);
                ATTACK_VISUAL.spawn(NewAttackParticle::VaporRing(ring));
            }
            RingKind::Energy {
                color,
                ease_in_fade,
                scale_variance,
            } => {
                let ring_scale = scale_with_variance(scale, scale_variance, &mut rng);
                let mut ring = EnergyRing::new(
                    position_xy,
                    created_at,
                    delay,
                    ring_scale,
                    rotation,
                    opacity,
                )
                .with_movement(rotation, 64.0);
                if let Some(color) = color {
                    ring = ring.with_color(color);
                }
                if ease_in_fade {
                    ring = ring.with_ease_in_fade();
                }
                ATTACK_VISUAL.spawn(NewAttackParticle::EnergyRing(ring));
            }
        }
    }
}

fn scale_with_variance(scale: f32, variance: f32, rng: &mut impl Rng) -> f32 {
    scale * rng.gen_range((1.0 - variance)..=(1.0 + variance))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn royal_path_scale_variation_stays_within_ten_percent() {
        let mut rng = rand::thread_rng();
        let scales: Vec<_> = (0..100)
            .map(|_| scale_with_variance(3.0, ROYAL_PATH_SCALE_VARIANCE, &mut rng))
            .collect();

        assert!(scales.iter().all(|scale| (2.7..=3.3).contains(scale)));
        assert!(scales.iter().any(|scale| *scale != 3.0));
    }
}
