use super::super::{ATTACK_FRAME_EASE_IN, ATTACK_FRAME_EASE_OUT};
use super::{ROYAL_PURPLE, STAR_COLORS, frame_index, point_at, progress_after_delay};
use namui::*;
use rand::Rng;

const FLARE_DURATION: Duration = Duration::from_millis(375);
const MAX_DELAY_MS: i64 = 150;
const MAX_DURATION_MS: i64 = 1500;
const PATH_STARS_PER_TILE: usize = 2;
const MAX_STAR_LIFETIME: Duration = Duration::from_millis(MAX_DELAY_MS + MAX_DURATION_MS);
const MAX_PATH_STAR_DISTANCE_TILES: f32 = 6.0;
const MAX_RADIAL_STAR_DISTANCE_TILES: f32 = 3.0;
const MAX_PATH_ANGLE_DEVIATION: f32 = 20.0 * std::f32::consts::PI / 180.0;
const MAX_PATH_STAR_OFFSET_TILES: f32 = 0.875;
#[derive(Clone, Copy)]
struct Star {
    start_xy: (f32, f32),
    direction: (f32, f32),
    travel_distance_tiles: f32,
    scale_tiles: f32,
    rotation: f32,
    rotation_delta: f32,
    color: [f32; 3],
    delay: Duration,
    duration: Duration,
    shrink: bool,
}

#[derive(Clone)]
pub struct MagicStars {
    created_at: Instant,
    elapsed: Duration,
    stars: Vec<Star>,
}

impl MagicStars {
    pub fn new(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) -> Vec<Self> {
        let delta = (end_xy.0 - start_xy.0, end_xy.1 - start_xy.1);
        let length = delta.0.hypot(delta.1);
        let direction = if length > f32::EPSILON {
            (delta.0 / length, delta.1 / length)
        } else {
            (1.0, 0.0)
        };
        let mut rng = rand::thread_rng();
        let mut stars = Vec::with_capacity(length.ceil() as usize * PATH_STARS_PER_TILE + 9);
        stars.push(Star {
            start_xy,
            direction: (0.0, 0.0),
            travel_distance_tiles: 0.0,
            scale_tiles: 4.0,
            rotation: rng.gen_range(0.0..std::f32::consts::TAU),
            rotation_delta: std::f32::consts::TAU,
            color: ROYAL_PURPLE,
            delay: Duration::ZERO,
            duration: FLARE_DURATION,
            shrink: false,
        });
        stars.extend(make_path_stars(start_xy, direction, length, &mut rng));
        stars.extend(make_radial_stars(start_xy, &mut rng));

        let capacity = particle::ParticleSprites::new().capacity().max(1);
        stars
            .chunks(capacity)
            .map(|batch| Self {
                created_at,
                elapsed: Duration::ZERO,
                stars: batch.to_vec(),
            })
            .collect()
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.elapsed = now - self.created_at;
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        let frames = &crate::atlas::generated::ATTACK_ROYAL_STRAIGHT_FLUSH_MAGIC_STAR_FLARE_00;
        let tile_size = crate::game_state::TILE_PX_SIZE;
        for star in &self.stars {
            let Some(progress) = progress_after_delay(self.elapsed, star.delay, star.duration)
            else {
                continue;
            };
            let eased_progress = ATTACK_FRAME_EASE_OUT.sample(progress);
            let frame = frames[frame_index(eased_progress, frames.len())];
            let scale_tiles = if star.shrink {
                star.scale_tiles * (1.0 - ATTACK_FRAME_EASE_IN.sample(progress))
            } else {
                star.scale_tiles
            };
            if scale_tiles <= f32::EPSILON {
                continue;
            }
            let center_xy = (
                star.start_xy.0 + star.direction.0 * star.travel_distance_tiles * eased_progress,
                star.start_xy.1 + star.direction.1 * star.travel_distance_tiles * eased_progress,
            );
            let scale = tile_size.width.as_f32() * scale_tiles / frame.width().as_f32();
            let color =
                Color::from_f01(star.color[0], star.color[1], star.color[2], 1.0 - progress);
            sprites.push(crate::atlas::centered_rotated_sprite(
                frame,
                px(center_xy.0 * tile_size.width.as_f32()),
                px(center_xy.1 * tile_size.height.as_f32()),
                scale,
                star.rotation + star.rotation_delta * eased_progress,
                Some(color),
            ));
        }
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= MAX_STAR_LIFETIME
    }
}

fn make_path_stars(
    start_xy: (f32, f32),
    direction: (f32, f32),
    length: f32,
    rng: &mut impl Rng,
) -> Vec<Star> {
    let count = length.ceil() as usize * PATH_STARS_PER_TILE;
    (0..count)
        .map(|_| {
            let distance = rng.gen_range(0.0_f32..=length);
            let angle_offset = rng.gen_range(-MAX_PATH_ANGLE_DEVIATION..=MAX_PATH_ANGLE_DEVIATION);
            let lateral_offset =
                rng.gen_range(-MAX_PATH_STAR_OFFSET_TILES..=MAX_PATH_STAR_OFFSET_TILES);
            let varied_direction = rotate_direction(direction, angle_offset);
            let path_position = point_at(start_xy, direction, distance);
            let perpendicular = (-direction.1, direction.0);
            make_moving_star(
                (
                    path_position.0 + perpendicular.0 * lateral_offset,
                    path_position.1 + perpendicular.1 * lateral_offset,
                ),
                varied_direction,
                rng.gen_range(1.0_f32..=MAX_PATH_STAR_DISTANCE_TILES),
                rng.gen_range(0.25_f32..=0.75),
                rng,
            )
        })
        .collect()
}

fn make_radial_stars(start_xy: (f32, f32), rng: &mut impl Rng) -> Vec<Star> {
    const COUNT: usize = 8;
    (0..COUNT)
        .map(|index| {
            let angle =
                std::f32::consts::TAU * index as f32 / COUNT as f32 + rng.gen_range(-0.2_f32..=0.2);
            make_moving_star(
                start_xy,
                (angle.cos(), angle.sin()),
                rng.gen_range(0.5_f32..=MAX_RADIAL_STAR_DISTANCE_TILES),
                rng.gen_range(0.5_f32..=1.0),
                rng,
            )
        })
        .collect()
}

fn rotate_direction(direction: (f32, f32), angle: f32) -> (f32, f32) {
    let (sin, cos) = angle.sin_cos();
    (
        direction.0 * cos - direction.1 * sin,
        direction.0 * sin + direction.1 * cos,
    )
}

fn make_moving_star(
    start_xy: (f32, f32),
    direction: (f32, f32),
    travel_distance_tiles: f32,
    scale_tiles: f32,
    rng: &mut impl Rng,
) -> Star {
    let turns = rng.gen_range(0.5_f32..=2.0);
    Star {
        start_xy,
        direction,
        travel_distance_tiles,
        scale_tiles,
        rotation: rng.gen_range(0.0..std::f32::consts::TAU),
        rotation_delta: turns * std::f32::consts::TAU * if rng.gen_bool(0.5) { 1.0 } else { -1.0 },
        color: STAR_COLORS[rng.gen_range(0..STAR_COLORS.len())],
        delay: Duration::from_millis(rng.gen_range(0_i64..=MAX_DELAY_MS)),
        duration: Duration::from_millis(rng.gen_range(150_i64..=MAX_DURATION_MS)),
        shrink: true,
    }
}

impl particle::Particle for MagicStars {
    fn tick(&mut self, now: Instant, dt: Duration) {
        MagicStars::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        MagicStars::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        MagicStars::is_done(self, now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_path_star_batches_respect_sprite_capacity() {
        let created_at = Instant::now();
        let capacity = particle::ParticleSprites::new().capacity();
        let stars = MagicStars::new((0.0, 0.0), (100.0, 0.0), created_at);

        assert!(stars.iter().all(|batch| batch.stars.len() <= capacity));
        assert_eq!(stars[0].stars[0].scale_tiles, 4.0);
        for mut batch in stars {
            batch.tick(created_at + Duration::from_millis(50), Duration::ZERO);
            assert!(batch.render().len() <= capacity);
        }
    }

    #[test]
    fn star_distances_and_path_angle_variation_stay_within_limits() {
        let mut rng = rand::thread_rng();
        let path_stars = make_path_stars((0.0, 0.0), (1.0, 0.0), 100.0, &mut rng);
        let radial_stars = make_radial_stars((0.0, 0.0), &mut rng);

        assert_eq!(
            MAX_PATH_ANGLE_DEVIATION,
            20.0 * std::f32::consts::PI / 180.0
        );
        assert_eq!(MAX_PATH_STAR_OFFSET_TILES, 0.875);
        assert!(path_stars.iter().all(|star| {
            star.travel_distance_tiles <= MAX_PATH_STAR_DISTANCE_TILES
                && star.direction.1.abs().atan2(star.direction.0) <= MAX_PATH_ANGLE_DEVIATION
                && star.start_xy.1.abs() <= MAX_PATH_STAR_OFFSET_TILES
        }));
        assert_eq!(path_stars.len(), 100 * PATH_STARS_PER_TILE);
        assert!(path_stars.iter().any(|star| star.start_xy.1.abs() > 0.0));
        assert!(
            radial_stars
                .iter()
                .all(|star| star.travel_distance_tiles <= MAX_RADIAL_STAR_DISTANCE_TILES)
        );
    }

    #[test]
    fn stars_live_until_the_extended_maximum_duration() {
        let created_at = Instant::now();
        let stars = MagicStars::new((0.0, 0.0), (5.0, 0.0), created_at);

        assert_eq!(stars[0].stars[0].duration, FLARE_DURATION);
        assert_eq!(FLARE_DURATION, Duration::from_millis(375));
        assert!(!stars[0].is_done(created_at + MAX_STAR_LIFETIME - Duration::from_millis(1)));
        assert!(stars[0].is_done(created_at + MAX_STAR_LIFETIME));
    }
}
