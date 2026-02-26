use crate::game_state::field_particle::{BlackSmokeParticle, spawn_black_smoke_particle};
use namui::*;
use rand::Rng;

const BLACK_SMOKE_EMIT_DURATION_MS: i64 = 120;
const BLACK_SMOKE_EMIT_RATE_PER_SEC: f32 = 512.0;
const BLACK_SMOKE_PHASE_SPREAD_RAD: f32 = std::f32::consts::PI * 0.6;
const BLACK_SMOKE_DASH_TRAIL_DENSITY_PER_TILE: f32 = 16.0;
const BLACK_SMOKE_DASH_TRAIL_POSITION_JITTER_TILE: f32 = 0.15;
const BLACK_SMOKE_DASH_TRAIL_DIRECTION_JITTER_DEG: f32 = 15.0;
const BLACK_SMOKE_DASH_TRAIL_MAX_SPEED_TILE_PER_SEC: f32 = 1.0;
const BLACK_SMOKE_PUFF_COUNT: usize = 32;
const BLACK_SMOKE_PUFF_POSITION_JITTER_TILE: f32 = 0.5;
const BLACK_SMOKE_PUFF_MAX_SPEED_TILE_PER_SEC: f32 = 1.5;

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

pub fn spawn_black_smoke_dash_trail(from_xy: (f32, f32), to_xy: (f32, f32), now: Instant) {
    let mut rng = rand::thread_rng();
    let dx = to_xy.0 - from_xy.0;
    let dy = to_xy.1 - from_xy.1;
    let distance = (dx * dx + dy * dy).sqrt();
    if distance <= 1e-6 {
        return;
    }

    let base_angle = dy.atan2(dx);
    let spawn_count = (distance * BLACK_SMOKE_DASH_TRAIL_DENSITY_PER_TILE)
        .round()
        .max(1.0) as usize;

    for _ in 0..spawn_count {
        let t = rng.gen_range(0.0..1.0);
        let path_xy = (from_xy.0 + dx * t, from_xy.1 + dy * t);
        let jitter_xy = (
            rng.gen_range(
                -BLACK_SMOKE_DASH_TRAIL_POSITION_JITTER_TILE
                    ..=BLACK_SMOKE_DASH_TRAIL_POSITION_JITTER_TILE,
            ),
            rng.gen_range(
                -BLACK_SMOKE_DASH_TRAIL_POSITION_JITTER_TILE
                    ..=BLACK_SMOKE_DASH_TRAIL_POSITION_JITTER_TILE,
            ),
        );
        let spawn_xy = (path_xy.0 + jitter_xy.0, path_xy.1 + jitter_xy.1);

        let angle_jitter_rad = rng
            .gen_range(
                -BLACK_SMOKE_DASH_TRAIL_DIRECTION_JITTER_DEG
                    ..=BLACK_SMOKE_DASH_TRAIL_DIRECTION_JITTER_DEG,
            )
            .to_radians();
        let move_angle = base_angle + angle_jitter_rad;
        let speed_tile_per_sec = rng.gen_range(0.0..=BLACK_SMOKE_DASH_TRAIL_MAX_SPEED_TILE_PER_SEC);
        let velocity_xy = (
            move_angle.cos() * speed_tile_per_sec,
            move_angle.sin() * speed_tile_per_sec,
        );

        spawn_black_smoke_particle(BlackSmokeParticle::new_dash_trail(
            spawn_xy,
            velocity_xy,
            now,
            &mut rng,
        ));
    }
}

pub fn spawn_black_smoke_puff_burst(xy: (f32, f32), now: Instant) {
    let mut rng = rand::thread_rng();

    for _ in 0..BLACK_SMOKE_PUFF_COUNT {
        let spawn_xy = (
            xy.0 + rng.gen_range(
                -BLACK_SMOKE_PUFF_POSITION_JITTER_TILE..=BLACK_SMOKE_PUFF_POSITION_JITTER_TILE,
            ),
            xy.1 + rng.gen_range(
                -BLACK_SMOKE_PUFF_POSITION_JITTER_TILE..=BLACK_SMOKE_PUFF_POSITION_JITTER_TILE,
            ),
        );
        let angle = rng.gen_range(0.0..std::f32::consts::TAU);
        let speed = rng.gen_range(0.0..=BLACK_SMOKE_PUFF_MAX_SPEED_TILE_PER_SEC);
        let velocity_xy = (angle.cos() * speed, angle.sin() * speed);

        spawn_black_smoke_particle(BlackSmokeParticle::new_puff(
            spawn_xy,
            velocity_xy,
            now,
            &mut rng,
        ));
    }
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
