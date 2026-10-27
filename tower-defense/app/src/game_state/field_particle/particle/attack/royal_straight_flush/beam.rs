use super::{MAX_LIFETIME, WHITE, frame_index, progress_after_delay};
use namui::*;
use rand::Rng;

const BEAM_DURATION: Duration = Duration::from_millis(400);
const MIN_SEGMENT_DELAY_MS: i64 = 15;
const MAX_ADDITIONAL_SEGMENT_DELAY_MS: i64 = 30;
const SEGMENT_OVERLAP_TILES: f32 = 1.5;
const BEAM_WIDTH_MULTIPLIER: f32 = 3.0;
const MAX_OPACITY: f32 = 0.75;

#[derive(Clone, Copy)]
struct Segment {
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    geometry: crate::atlas::LineSpriteGeometry,
    length_px: f32,
    scale: f32,
    delay: Duration,
}

#[derive(Clone)]
pub struct Beam {
    created_at: Instant,
    elapsed: Duration,
    segments: Vec<Segment>,
}

impl Beam {
    pub fn new(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) -> Vec<Self> {
        let delta = (end_xy.0 - start_xy.0, end_xy.1 - start_xy.1);
        let tile_size = crate::game_state::TILE_PX_SIZE;
        let total_length_px =
            (delta.0 * tile_size.width.as_f32()).hypot(delta.1 * tile_size.height.as_f32());
        let segments = make_segments(start_xy, delta, total_length_px, &mut rand::thread_rng());
        let capacity = particle::ParticleSprites::new().capacity().max(1);

        segments
            .chunks(capacity)
            .map(|segments| Self {
                created_at,
                elapsed: Duration::ZERO,
                segments: segments.to_vec(),
            })
            .collect()
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.elapsed = now - self.created_at;
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        let frames = &crate::atlas::generated::ATTACK_ROYAL_STRAIGHT_FLUSH_BEAM_CORE;
        let tile_size = crate::game_state::TILE_PX_SIZE;

        for segment in &self.segments {
            let Some(progress) = progress_after_delay(self.elapsed, segment.delay, BEAM_DURATION)
            else {
                continue;
            };
            let frame = frames[frame_index(progress, frames.len())];
            let alpha_only =
                Color::from_f01(WHITE[0], WHITE[1], WHITE[2], MAX_OPACITY - progress * 0.25);
            let thickness =
                (tile_size.height.as_f32() * 0.7 * segment.scale * BEAM_WIDTH_MULTIPLIER)
                    .min(segment.length_px * frame.height().as_f32() / frame.width().as_f32());
            if let Some(sprite) =
                segment
                    .geometry
                    .sprite_with_thickness(frame, thickness, Some(alpha_only))
            {
                sprites.push(sprite);
            }
        }
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= MAX_LIFETIME
    }
}

fn make_segments(
    start_xy: (f32, f32),
    delta: (f32, f32),
    total_length_px: f32,
    rng: &mut impl Rng,
) -> Vec<Segment> {
    if total_length_px <= f32::EPSILON {
        return Vec::new();
    }
    let frames = &crate::atlas::generated::ATTACK_ROYAL_STRAIGHT_FLUSH_BEAM_CORE;
    let minimum_frame_aspect = frames
        .iter()
        .map(|frame| frame.width().as_f32() / frame.height().as_f32())
        .fold(f32::INFINITY, f32::min);
    let tile_size = crate::game_state::TILE_PX_SIZE;
    let overlap_px = SEGMENT_OVERLAP_TILES * tile_size.width.as_f32();
    let mut segments = Vec::new();
    let mut start_distance_px = 0.0;
    let mut index = 0;
    while start_distance_px < total_length_px {
        let scale = if index < 5 {
            0.5 + index as f32 * 0.125
        } else {
            1.0
        };
        let thickness = tile_size.height.as_f32() * 0.7 * scale * BEAM_WIDTH_MULTIPLIER;
        let max_segment_length_px = minimum_frame_aspect * thickness;
        let end_distance_px = (start_distance_px + max_segment_length_px).min(total_length_px);
        let start_progress = start_distance_px / total_length_px;
        let end_progress = end_distance_px / total_length_px;
        let segment_start_xy = (
            start_xy.0 + delta.0 * start_progress,
            start_xy.1 + delta.1 * start_progress,
        );
        let segment_end_xy = (
            start_xy.0 + delta.0 * end_progress,
            start_xy.1 + delta.1 * end_progress,
        );
        let geometry = crate::atlas::LineSpriteGeometry::new(
            px(segment_start_xy.0 * tile_size.width.as_f32()),
            px(segment_start_xy.1 * tile_size.height.as_f32()),
            px(segment_end_xy.0 * tile_size.width.as_f32()),
            px(segment_end_xy.1 * tile_size.height.as_f32()),
            1.0,
        )
        .expect("beam segment geometry is non-degenerate");
        segments.push(Segment {
            start_xy: segment_start_xy,
            end_xy: segment_end_xy,
            geometry,
            length_px: end_distance_px - start_distance_px,
            scale,
            delay: Duration::from_millis(
                MIN_SEGMENT_DELAY_MS + rng.gen_range(0_i64..=MAX_ADDITIONAL_SEGMENT_DELAY_MS),
            ),
        });
        if end_distance_px >= total_length_px {
            break;
        }
        let segment_length_px = end_distance_px - start_distance_px;
        let segment_overlap_px = overlap_px.min(segment_length_px * 0.5);
        start_distance_px = end_distance_px - segment_overlap_px;
        index += 1;
    }
    segments
}

impl particle::Particle for Beam {
    fn tick(&mut self, now: Instant, dt: Duration) {
        Beam::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        Beam::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        Beam::is_done(self, now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long_beam_stays_within_sprite_capacity() {
        let created_at = Instant::now();
        let mut beams = Beam::new((0.0, 0.0), (100.0, 0.0), created_at);
        let capacity = particle::ParticleSprites::new().capacity();
        assert!(beams.iter().all(|beam| beam.segments.len() <= capacity));
        let all_segments: Vec<_> = beams.iter().flat_map(|beam| &beam.segments).collect();
        assert!(all_segments.len() > 5);
        assert!(all_segments.iter().all(|segment| {
            segment.delay >= Duration::from_millis(MIN_SEGMENT_DELAY_MS)
                && segment.delay
                    <= Duration::from_millis(MIN_SEGMENT_DELAY_MS + MAX_ADDITIONAL_SEGMENT_DELAY_MS)
        }));
        let overlap = all_segments[0].end_xy.0 - all_segments[1].start_xy.0;
        let first_segment_length = all_segments[0].end_xy.0 - all_segments[0].start_xy.0;
        let expected_overlap = SEGMENT_OVERLAP_TILES.min(first_segment_length * 0.5);
        assert!((overlap - expected_overlap).abs() < 1e-5);
        assert!((all_segments[0].start_xy.0).abs() < 1e-5);
        assert!((all_segments.last().unwrap().end_xy.0 - 100.0).abs() < 1e-5);
        assert!(
            all_segments
                .iter()
                .all(|segment| segment.start_xy.0 <= segment.end_xy.0)
        );
        let frames = &crate::atlas::generated::ATTACK_ROYAL_STRAIGHT_FLUSH_BEAM_CORE;
        let tile_size = crate::game_state::TILE_PX_SIZE;
        for segment in &all_segments {
            let length_px = ((segment.end_xy.0 - segment.start_xy.0) * tile_size.width.as_f32())
                .hypot((segment.end_xy.1 - segment.start_xy.1) * tile_size.height.as_f32());
            for frame in frames {
                let thickness =
                    (tile_size.height.as_f32() * 0.7 * segment.scale * BEAM_WIDTH_MULTIPLIER)
                        .min(length_px * frame.height().as_f32() / frame.width().as_f32());
                let source_render_length =
                    frame.width().as_f32() * thickness / frame.height().as_f32();
                assert!((source_render_length - length_px).abs() < 1e-3);
            }
        }
        for beam in &mut beams {
            beam.tick(created_at + Duration::from_millis(50), Duration::ZERO);
            assert!(beam.render().len() <= capacity);
            assert!(!beam.is_done(created_at + Duration::from_millis(649)));
            assert!(beam.is_done(created_at + MAX_LIFETIME + Duration::from_millis(1)));
        }
    }

    #[test]
    fn short_beam_last_segment_renders_full_frame() {
        let created_at = Instant::now();
        let frames = &crate::atlas::generated::ATTACK_ROYAL_STRAIGHT_FLUSH_BEAM_CORE;
        let tile_size = crate::game_state::TILE_PX_SIZE;

        for range_tiles in [0.1, 0.5, 1.0] {
            let beams = Beam::new((0.0, 0.0), (range_tiles, 0.0), created_at);
            for segment in beams.iter().flat_map(|beam| &beam.segments) {
                let length_px = (segment.end_xy.0 - segment.start_xy.0) * tile_size.width.as_f32();
                for frame in frames {
                    let thickness =
                        (tile_size.height.as_f32() * 0.7 * segment.scale * BEAM_WIDTH_MULTIPLIER)
                            .min(length_px * frame.height().as_f32() / frame.width().as_f32());
                    let source_render_length =
                        frame.width().as_f32() * thickness / frame.height().as_f32();
                    assert!((source_render_length - length_px).abs() < 1e-3);
                }
            }
        }
    }
}
