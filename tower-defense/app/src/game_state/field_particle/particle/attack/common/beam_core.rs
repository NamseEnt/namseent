use super::super::{ATTACK_FRAME_EASE_IN, ATTACK_FRAME_EASE_OUT};
use namui::*;

const EXTEND_DURATION: Duration = Duration::from_millis(50);
const RETRACT_DURATION: Duration = Duration::from_millis(100);
const MAX_WIDTH_TILES: f32 = 2.0;
const FRAME_DURATION: Duration = Duration::from_millis(50);
const SEGMENT_OVERLAP_TILES: f32 = 0.2;

#[derive(Clone)]
pub struct BeamCore {
    start_xy: (f32, f32),
    direction: (f32, f32),
    target_length_tiles: f32,
    created_at: Instant,
    elapsed: Duration,
    width: f32,
    started: bool,
}

impl BeamCore {
    pub fn new(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) -> Self {
        let direction_x = end_xy.0 - start_xy.0;
        let direction_y = end_xy.1 - start_xy.1;
        let angle = direction_y.atan2(direction_x);
        let (direction_sin, direction_cos) = angle.sin_cos();
        Self {
            start_xy,
            direction: (direction_cos, direction_sin),
            target_length_tiles: direction_x.hypot(direction_y),
            created_at,
            elapsed: Duration::ZERO,
            width: 0.0,
            started: false,
        }
    }

    fn width_at(elapsed: Duration) -> f32 {
        if elapsed <= EXTEND_DURATION {
            let progress = elapsed.as_secs_f32() / EXTEND_DURATION.as_secs_f32();
            MAX_WIDTH_TILES * ATTACK_FRAME_EASE_OUT.sample(progress)
        } else {
            let progress = ((elapsed - EXTEND_DURATION).as_secs_f32()
                / RETRACT_DURATION.as_secs_f32())
            .clamp(0.0, 1.0);
            MAX_WIDTH_TILES * (1.0 - ATTACK_FRAME_EASE_IN.sample(progress))
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.elapsed = now - self.created_at;
        self.width = if self.elapsed < EXTEND_DURATION + RETRACT_DURATION {
            Self::width_at(self.elapsed)
        } else {
            0.0
        };
        self.started = true;
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if !self.started || self.elapsed >= EXTEND_DURATION + RETRACT_DURATION {
            return sprites;
        }

        let frames = &crate::atlas::generated::ATTACK_COMMON_BEAM_CORE;
        let frame_index = (self.elapsed.as_millis() / FRAME_DURATION.as_millis()) as usize;
        let frame = frames[frame_index.min(frames.len() - 1)];
        let total_length_px =
            self.target_length_tiles * crate::game_state::TILE_PX_SIZE.width.as_f32();
        let thickness = crate::game_state::TILE_PX_SIZE.height.as_f32() * self.width;
        if total_length_px <= f32::EPSILON || thickness <= f32::EPSILON {
            return sprites;
        }

        let sprite_scale_for_thickness = thickness / frame.height().as_f32();
        let segment_length_for_thickness = frame.width().as_f32() * sprite_scale_for_thickness;
        let overlap_px = SEGMENT_OVERLAP_TILES * crate::game_state::TILE_PX_SIZE.width.as_f32();
        let overlap_for_thickness = overlap_px.min(segment_length_for_thickness / 2.0);
        let desired_segment_count = segment_count(
            total_length_px,
            segment_length_for_thickness,
            overlap_for_thickness,
        );
        let segment_length_px = if desired_segment_count > sprites.capacity() {
            (total_length_px + overlap_px * (sprites.capacity() - 1) as f32)
                / sprites.capacity() as f32
        } else {
            segment_length_for_thickness
        };
        let sprite_scale = segment_length_px / frame.width().as_f32();
        let segment_count = segment_count(
            total_length_px,
            segment_length_px,
            overlap_px.min(segment_length_px / 2.0),
        )
        .min(sprites.capacity());
        let segment_stride_px = segment_length_px - overlap_px.min(segment_length_px / 2.0);
        let segment_thickness = frame.height().as_f32() * sprite_scale;
        let opacity = thickness / segment_thickness;
        let start_x_px = self.start_xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32();
        let start_y_px = self.start_xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32();

        for segment_index in 0..segment_count {
            let segment_start_px = segment_index as f32 * segment_stride_px;
            let segment_end_px = (segment_start_px + segment_length_px).min(total_length_px);
            let segment_start_x_px = start_x_px + self.direction.0 * segment_start_px;
            let segment_start_y_px = start_y_px + self.direction.1 * segment_start_px;
            let segment_end_x_px = start_x_px + self.direction.0 * segment_end_px;
            let segment_end_y_px = start_y_px + self.direction.1 * segment_end_px;
            let color = Color::from_f01(0.6, 0.95, 1.0, opacity);
            if let Some(sprite) = crate::atlas::line_sprite_from_rect(
                frame,
                px(segment_start_x_px),
                px(segment_start_y_px),
                px(segment_end_x_px),
                px(segment_end_y_px),
                segment_thickness,
                Some(color),
            ) {
                sprites.push(sprite);
            }
        }
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= EXTEND_DURATION + RETRACT_DURATION
    }
}

fn segment_count(total_length_px: f32, segment_length_px: f32, overlap_px: f32) -> usize {
    if total_length_px <= segment_length_px {
        return 1;
    }

    let stride_px = segment_length_px - overlap_px;
    ((total_length_px - segment_length_px) / stride_px).ceil() as usize + 1
}

impl particle::Particle for BeamCore {
    fn tick(&mut self, now: Instant, dt: Duration) {
        BeamCore::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        BeamCore::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        BeamCore::is_done(self, now)
    }
}

#[cfg(test)]
mod tests {
    use super::{BeamCore, SEGMENT_OVERLAP_TILES};
    use namui::Duration;

    #[test]
    fn beam_reaches_two_tiles_at_fifty_ms_and_retracts_by_one_hundred_fifty_ms() {
        assert_eq!(BeamCore::width_at(Duration::ZERO), 0.0);
        assert_eq!(
            BeamCore::width_at(Duration::from_millis(50)),
            super::MAX_WIDTH_TILES
        );
        assert_eq!(BeamCore::width_at(Duration::from_millis(150)), 0.0);
    }

    #[test]
    fn render_overlaps_segments_to_cover_beam_length() {
        let now = namui::Instant::now();
        let mut beam = BeamCore::new((0.0, 0.0), (11.0, 0.0), now);
        beam.tick(now + Duration::from_millis(50), Duration::ZERO);

        let sprites = beam.render();
        let segment_length_px = sprites[0].src_rect.width().as_f32() * sprites[0].xform.scos;
        let segment_stride_px = sprites[1].xform.tx.as_f32() - sprites[0].xform.tx.as_f32();
        let overlap_px = segment_length_px - segment_stride_px;

        assert_eq!(sprites.len(), 3);
        let expected_overlap_px =
            SEGMENT_OVERLAP_TILES * crate::game_state::TILE_PX_SIZE.width.as_f32();
        assert!((overlap_px - expected_overlap_px).abs() < 0.001);
        assert!((sprites[2].xform.tx.as_f32() - segment_stride_px * 2.0).abs() < 0.001);
    }

    #[test]
    fn render_caps_extremely_long_beams_to_particle_sprite_capacity() {
        let now = namui::Instant::now();
        let mut beam = BeamCore::new((0.0, 0.0), (100.0, 0.0), now);
        beam.tick(now + Duration::from_millis(50), Duration::ZERO);

        assert_eq!(
            beam.render().len(),
            namui::particle::ParticleSprites::new().capacity()
        );
    }
}
