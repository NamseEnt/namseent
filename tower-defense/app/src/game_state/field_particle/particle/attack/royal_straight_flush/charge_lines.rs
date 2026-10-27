use super::super::ATTACK_FRAME_EASE_OUT;
use super::{MAX_LIFETIME, ROYAL_PURPLE, frame_index, point_at, progress_after_delay};
use namui::*;
use rand::Rng;

const LINE_COUNT: usize = 8;
const LINE_SIZE_MULTIPLIER: f32 = 4.0;
const MIN_DURATION_MS: i64 = 150;
const MAX_DURATION_MS: i64 = 250;

#[derive(Clone, Copy)]
struct Line {
    endpoint_xy: (f32, f32),
    angle: f32,
    length_tiles: f32,
    thickness_tiles: f32,
    delay: Duration,
    duration: Duration,
    flip_vertical: bool,
}

#[derive(Clone)]
pub struct ChargeLines {
    created_at: Instant,
    elapsed: Duration,
    lines: Vec<Line>,
}

impl ChargeLines {
    pub fn new(start_xy: (f32, f32), created_at: Instant) -> Self {
        Self {
            created_at,
            elapsed: Duration::ZERO,
            lines: make_lines(start_xy, &mut rand::thread_rng()),
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.elapsed = now - self.created_at;
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        let frames = &crate::atlas::generated::ATTACK_ROYAL_STRAIGHT_FLUSH_ENERGY_CHARGE_LINE_00;
        let tile_size = crate::game_state::TILE_PX_SIZE;
        for line in &self.lines {
            let Some(progress) = progress_after_delay(self.elapsed, line.delay, line.duration)
            else {
                continue;
            };
            let frame = frames[frame_index(ATTACK_FRAME_EASE_OUT.sample(progress), frames.len())];
            let angle = line.angle
                + if line.flip_vertical {
                    std::f32::consts::PI
                } else {
                    0.0
                };
            let start_xy = (
                line.endpoint_xy.0 - angle.cos() * line.length_tiles,
                line.endpoint_xy.1 - angle.sin() * line.length_tiles,
            );
            let color = Color::from_f01(
                ROYAL_PURPLE[0],
                ROYAL_PURPLE[1],
                ROYAL_PURPLE[2],
                1.0 - progress,
            );
            if let Some(sprite) = crate::atlas::line_sprite_from_rect(
                frame,
                px(start_xy.0 * tile_size.width.as_f32()),
                px(start_xy.1 * tile_size.height.as_f32()),
                px(line.endpoint_xy.0 * tile_size.width.as_f32()),
                px(line.endpoint_xy.1 * tile_size.height.as_f32()),
                tile_size.height.as_f32() * line.thickness_tiles,
                Some(color),
            ) {
                sprites.push(sprite);
            }
        }
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= MAX_LIFETIME
    }
}

fn make_lines(start_xy: (f32, f32), rng: &mut impl Rng) -> Vec<Line> {
    (0..LINE_COUNT)
        .map(|index| {
            let angle = std::f32::consts::TAU * index as f32 / LINE_COUNT as f32
                + rng.gen_range(-0.2_f32..=0.2);
            let radius = rng.gen_range(0.05_f32..=0.3);
            let direction = (angle.cos(), angle.sin());
            Line {
                endpoint_xy: point_at(start_xy, direction, radius),
                angle,
                length_tiles: rng.gen_range(0.35_f32..=0.85) * LINE_SIZE_MULTIPLIER,
                thickness_tiles: rng.gen_range(0.12_f32..=0.24) * LINE_SIZE_MULTIPLIER,
                delay: Duration::from_millis(rng.gen_range(0_i64..=60)),
                duration: Duration::from_millis(rng.gen_range(MIN_DURATION_MS..=MAX_DURATION_MS)),
                flip_vertical: rng.gen_bool(0.5),
            }
        })
        .collect()
}

impl particle::Particle for ChargeLines {
    fn tick(&mut self, now: Instant, dt: Duration) {
        ChargeLines::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        ChargeLines::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        ChargeLines::is_done(self, now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_eight_radial_lines() {
        let lines = ChargeLines::new((0.0, 0.0), Instant::now());

        assert_eq!(lines.lines.len(), LINE_COUNT);
        assert!(lines.lines.iter().all(|line| {
            (1.4..=3.4).contains(&line.length_tiles)
                && (0.48..=0.96).contains(&line.thickness_tiles)
                && (200..=400).contains(&line.duration.as_millis())
                && line.delay <= Duration::from_millis(120)
        }));
    }
}
