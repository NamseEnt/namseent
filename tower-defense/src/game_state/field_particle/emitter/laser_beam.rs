use crate::game_state::field_particle::{
    BLUE_DOT_SPARKS, BlueDotSparkParticle, LASER_LINES, LIGHTNING_BOLTS, LaserLineParticle,
    LightningBoltParticle,
};
use namui::*;
use rand::Rng;

const LASER_LINE_COUNT: usize = 8;
const LINE_THICKNESS_MIN: f32 = 0.1;
const LINE_THICKNESS_MAX: f32 = 0.25;
const LASER_LIFETIME_MS: i64 = 120;
const START_OFFSET_RANGE: f32 = 0.9;
const END_OFFSET_RANGE: f32 = 0.9;
const MOVEMENT_SPEED: f32 = 32.0;
const LIGHTNING_BOLT_COUNT: usize = 4;
const LIGHTNING_BOLT_SPAWN_CHANCE: f32 = 0.8;
const BLUE_DOT_SPARK_COUNT: usize = 4;
const BLUE_DOT_SPARK_ANGLE_RANGE: f32 = 0.436;

pub fn spawn_laser_beam(start_xy: (f32, f32), end_xy: (f32, f32), now: Instant) {
    let mut rng = rand::thread_rng();

    let dx = end_xy.0 - start_xy.0;
    let dy = end_xy.1 - start_xy.1;
    let laser_length = (dx * dx + dy * dy).sqrt();

    emit_laser_lines(start_xy, now, dx, dy, &mut rng);

    for _ in 0..LIGHTNING_BOLT_COUNT {
        let lightning = create_lightning_bolt(start_xy, now, dx, dy, laser_length, &mut rng);
        LIGHTNING_BOLTS.spawn(lightning);
    }

    emit_blue_dot_sparks(end_xy, dx, dy, &mut rng, now);
}

fn emit_laser_lines(
    start_xy: (f32, f32),
    now: Instant,
    dx: f32,
    dy: f32,
    rng: &mut rand::rngs::ThreadRng,
) {
    for i in 0..LASER_LINE_COUNT {
        let (start_t, end_t, thickness) = if i == 0 {
            (0.0, 1.0, LINE_THICKNESS_MAX)
        } else {
            let start_t = rng.gen_range(0.0..START_OFFSET_RANGE);
            let end_t = rng.gen_range((1.0 - END_OFFSET_RANGE)..1.0);
            let thickness = rng.gen_range(LINE_THICKNESS_MIN..LINE_THICKNESS_MAX);
            (start_t, end_t, thickness)
        };

        let line_start = (start_xy.0 + dx * start_t, start_xy.1 + dy * start_t);
        let line_end = (start_xy.0 + dx * end_t, start_xy.1 + dy * end_t);
        let target_xy = (start_xy.0 + dx, start_xy.1 + dy);

        let particle = LaserLineParticle::new(
            line_start,
            line_end,
            target_xy,
            now,
            Duration::from_millis(LASER_LIFETIME_MS),
            thickness,
            MOVEMENT_SPEED,
        );

        LASER_LINES.spawn(particle);
    }
}

fn create_lightning_bolt(
    start_xy: (f32, f32),
    now: Instant,
    dx: f32,
    dy: f32,
    laser_length: f32,
    rng: &mut rand::rngs::ThreadRng,
) -> LightningBoltParticle {
    let t = rng.gen_range(0.0..1.0);
    let bolt_start = (start_xy.0 + dx * t, start_xy.1 + dy * t);

    let perp_x = -dy / laser_length.max(0.001);
    let perp_y = dx / laser_length.max(0.001);

    let end_t = rng.gen_range(0.6..1.0);
    let mut bolt_end = (start_xy.0 + dx * end_t, start_xy.1 + dy * end_t);

    let angle_offset = rng.gen_range(-0.5..0.5);
    bolt_end.0 += perp_x * angle_offset;
    bolt_end.1 += perp_y * angle_offset;

    let bolt_lifetime = Duration::from_millis(rng.gen_range(50..100));

    LightningBoltParticle::new(
        bolt_start,
        bolt_end,
        now,
        bolt_lifetime,
        LIGHTNING_BOLT_SPAWN_CHANCE,
    )
}

fn emit_blue_dot_sparks(
    end_xy: (f32, f32),
    dx: f32,
    dy: f32,
    rng: &mut rand::rngs::ThreadRng,
    now: Instant,
) {
    let laser_length = (dx * dx + dy * dy).sqrt();
    if laser_length < 0.001 {
        return;
    }

    let base_dir_x = -dx / laser_length;
    let base_dir_y = -dy / laser_length;

    for _ in 0..BLUE_DOT_SPARK_COUNT {
        let angle_variation =
            rng.gen_range(-BLUE_DOT_SPARK_ANGLE_RANGE..BLUE_DOT_SPARK_ANGLE_RANGE);
        let cos_a = angle_variation.cos();
        let sin_a = angle_variation.sin();

        let movement_dir_x = base_dir_x * cos_a - base_dir_y * sin_a;
        let movement_dir_y = base_dir_x * sin_a + base_dir_y * cos_a;

        let particle = BlueDotSparkParticle::new_with_random(
            end_xy,
            (movement_dir_x, movement_dir_y),
            now,
            rng,
        );

        BLUE_DOT_SPARKS.spawn(particle);
    }
}
