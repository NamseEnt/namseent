use super::super::{ATTACK_FRAME_EASE_IN, ATTACK_FRAME_EASE_OUT};
use namui::*;

const EXTEND_DURATION: Duration = Duration::from_millis(50);
const RETRACT_DURATION: Duration = Duration::from_millis(100);
const MAX_WIDTH_TILES: f32 = 2.0;
const FRAME_DURATION: Duration = Duration::from_millis(50);

#[derive(Clone)]
pub struct BeamCore {
    start_xy: (f32, f32),
    angle: f32,
    target_length_tiles: f32,
    created_at: Instant,
    elapsed: Duration,
    started: bool,
}

impl BeamCore {
    pub fn new(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) -> Self {
        let direction_x = end_xy.0 - start_xy.0;
        let direction_y = end_xy.1 - start_xy.1;
        Self {
            start_xy,
            angle: direction_y.atan2(direction_x),
            target_length_tiles: direction_x.hypot(direction_y),
            created_at,
            elapsed: Duration::ZERO,
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
        let scale_x = total_length_px / frame.width().as_f32();
        let thickness =
            crate::game_state::TILE_PX_SIZE.height.as_f32() * Self::width_at(self.elapsed);
        let scale_y = thickness / frame.height().as_f32();
        if total_length_px <= f32::EPSILON || thickness <= f32::EPSILON {
            return sprites;
        }

        let start_x_px = self.start_xy.0 * crate::game_state::TILE_PX_SIZE.width.as_f32();
        let start_y_px = self.start_xy.1 * crate::game_state::TILE_PX_SIZE.height.as_f32();
        let center_x = px(start_x_px + self.angle.cos() * total_length_px / 2.0);
        let center_y = px(start_y_px + self.angle.sin() * total_length_px / 2.0);
        let color = Color::from_f01(0.6, 0.95, 1.0, 1.0);
        sprites.push(crate::atlas::centered_rotated_sprite_non_uniform(
            frame,
            center_x,
            center_y,
            scale_x,
            scale_y,
            self.angle,
            Some(color),
        ));
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= EXTEND_DURATION + RETRACT_DURATION
    }
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
    use super::BeamCore;
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
    fn changing_width_keeps_the_target_length_fixed() {
        let frame_width = 128.0;
        let frame_height = 64.0;
        let length_px = 11.0 * 64.0;
        let scale_x = length_px / frame_width;
        let narrow_scale_y = 64.0 / frame_height;
        let wide_scale_y = 128.0 / frame_height;

        assert_eq!(scale_x * frame_width, length_px);
        assert!(narrow_scale_y < wide_scale_y);
        assert_eq!(narrow_scale_y * frame_height, 64.0);
        assert_eq!(wide_scale_y * frame_height, 128.0);
    }

    #[test]
    fn render_produces_only_one_sprite() {
        let now = namui::Instant::now();
        let mut beam = BeamCore::new((0.0, 0.0), (11.0, 0.0), now);
        beam.tick(now + Duration::from_millis(50), Duration::ZERO);

        assert_eq!(beam.render().len(), 1);
    }
}
