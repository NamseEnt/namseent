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
    geometry: crate::atlas::LineSpriteGeometry,
    delay: Duration,
    duration: Duration,
    progress: Option<f32>,
    eased_progress: f32,
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
        for line in &mut self.lines {
            line.progress = progress_after_delay(self.elapsed, line.delay, line.duration);
            if let Some(progress) = line.progress {
                line.eased_progress = ATTACK_FRAME_EASE_OUT.sample(progress);
            }
        }
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        let frames = &crate::atlas::generated::ATTACK_ROYAL_STRAIGHT_FLUSH_ENERGY_CHARGE_LINE_00;
        for line in &self.lines {
            let Some(progress) = line.progress else {
                continue;
            };
            let frame = frames[frame_index(line.eased_progress, frames.len())];
            let color = Color::from_f01(
                ROYAL_PURPLE[0],
                ROYAL_PURPLE[1],
                ROYAL_PURPLE[2],
                1.0 - progress,
            );
            if let Some(sprite) = line.geometry.sprite(frame, Some(color)) {
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
    let tile_size = crate::game_state::TILE_PX_SIZE;
    (0..LINE_COUNT)
        .map(|index| {
            let angle = std::f32::consts::TAU * index as f32 / LINE_COUNT as f32
                + rng.gen_range(-0.2_f32..=0.2);
            let radius = rng.gen_range(0.05_f32..=0.3);
            let direction = (angle.cos(), angle.sin());
            let endpoint_xy = point_at(start_xy, direction, radius);
            let length_tiles = rng.gen_range(0.35_f32..=0.85) * LINE_SIZE_MULTIPLIER;
            let thickness_tiles = rng.gen_range(0.12_f32..=0.24) * LINE_SIZE_MULTIPLIER;
            let delay = Duration::from_millis(rng.gen_range(0_i64..=60));
            let duration = Duration::from_millis(rng.gen_range(MIN_DURATION_MS..=MAX_DURATION_MS));
            let angle = angle
                + if rng.gen_bool(0.5) {
                    std::f32::consts::PI
                } else {
                    0.0
                };
            let start_xy = (
                endpoint_xy.0 - angle.cos() * length_tiles,
                endpoint_xy.1 - angle.sin() * length_tiles,
            );
            let geometry = crate::atlas::LineSpriteGeometry::new(
                px(start_xy.0 * tile_size.width.as_f32()),
                px(start_xy.1 * tile_size.height.as_f32()),
                px(endpoint_xy.0 * tile_size.width.as_f32()),
                px(endpoint_xy.1 * tile_size.height.as_f32()),
                tile_size.height.as_f32() * thickness_tiles,
            )
            .expect("charge line geometry is non-degenerate");
            Line {
                geometry,
                delay,
                duration,
                progress: None,
                eased_progress: 0.0,
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
            (Duration::from_millis(MIN_DURATION_MS)..=Duration::from_millis(MAX_DURATION_MS))
                .contains(&line.duration)
                && line.delay <= Duration::from_millis(60)
        }));
    }
}
