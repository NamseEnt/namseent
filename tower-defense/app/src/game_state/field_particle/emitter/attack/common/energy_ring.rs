use namui::*;

pub(in crate::game_state::field_particle::emitter::attack) fn spawn(
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    delay_offset: Duration,
    delay_step: Duration,
    scale: f32,
) {
    super::ring_path::spawn(
        start_xy,
        end_xy,
        created_at,
        delay_offset,
        delay_step,
        scale,
        super::ring_path::RingKind::Energy,
    );
}
