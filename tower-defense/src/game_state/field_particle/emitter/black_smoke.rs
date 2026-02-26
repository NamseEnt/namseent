use crate::game_state::field_particle::{BlackSmokeParticle, spawn_black_smoke_particle};
use namui::*;
use rand::Rng;

const BLACK_SMOKE_EMIT_DURATION_MS: i64 = 120;
const BLACK_SMOKE_EMIT_RATE_PER_SEC: f32 = 512.0;
const BLACK_SMOKE_PHASE_SPREAD_RAD: f32 = std::f32::consts::PI * 0.6;

#[derive(Clone, Copy, State)]
pub struct BlackSmokeSource {
    xy: (f32, f32),
    remaining: Duration,
    spawn_accumulator: f32,
    base_phase_rad: f32,
    reverse_progress: bool,
}

pub fn spawn_black_smoke_burst(sources: &mut Vec<BlackSmokeSource>, xy: (f32, f32), now: Instant) {
    spawn_black_smoke_burst_impl(sources, xy, now, false);
}

pub fn spawn_black_smoke_burst_reversed(
    sources: &mut Vec<BlackSmokeSource>,
    xy: (f32, f32),
    now: Instant,
) {
    spawn_black_smoke_burst_impl(sources, xy, now, true);
}

fn spawn_black_smoke_burst_impl(
    sources: &mut Vec<BlackSmokeSource>,
    xy: (f32, f32),
    now: Instant,
    reverse_progress: bool,
) {
    let _ = now;
    let mut rng = rand::thread_rng();
    sources.push(BlackSmokeSource {
        xy,
        remaining: Duration::from_millis(BLACK_SMOKE_EMIT_DURATION_MS),
        spawn_accumulator: 0.0,
        base_phase_rad: rng.gen_range(0.0..std::f32::consts::TAU),
        reverse_progress,
    });
}

pub fn tick_black_smoke_emitters(sources: &mut Vec<BlackSmokeSource>, now: Instant, dt: Duration) {
    let mut rng = rand::thread_rng();

    for i in (0..sources.len()).rev() {
        let source = &mut sources[i];
        let active_dt = source.remaining.min(dt);
        let active_secs = active_dt.as_secs_f32();

        source.spawn_accumulator += active_secs * BLACK_SMOKE_EMIT_RATE_PER_SEC;
        let spawn_count = source.spawn_accumulator.floor() as usize;
        source.spawn_accumulator -= spawn_count as f32;

        for _ in 0..spawn_count {
            let phase_offset_rad = (source.base_phase_rad
                + rng.gen_range(-BLACK_SMOKE_PHASE_SPREAD_RAD..=BLACK_SMOKE_PHASE_SPREAD_RAD))
            .rem_euclid(std::f32::consts::TAU);
            spawn_black_smoke_particle(BlackSmokeParticle::new_with_phase_offset(
                source.xy,
                now,
                phase_offset_rad,
                source.reverse_progress,
                &mut rng,
            ));
        }

        source.remaining -= active_dt;
        if source.remaining == Duration::ZERO {
            sources.remove(i);
        }
    }
}
