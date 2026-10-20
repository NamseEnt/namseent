mod bullet_trail;
mod vapor_ring;

use namui::*;

pub(super) fn spawn_bullet_trail(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    delay_offset: Duration,
    delay_step: Duration,
    scale: f32,
) {
    bullet_trail::spawn(
        start_xy,
        end_xy,
        created_at,
        delay_offset,
        delay_step,
        scale,
    );
}

pub(super) fn spawn_vapor_ring_trail(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    delay_offset: Duration,
    delay_step: Duration,
    scale: f32,
) {
    vapor_ring::spawn(
        start_xy,
        end_xy,
        created_at,
        delay_offset,
        delay_step,
        scale,
    );
}
