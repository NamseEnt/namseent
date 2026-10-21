use super::super::ATTACK_FRAME_EASE_OUT;
use namui::*;

const ARC_HEIGHT_TILES: f32 = 2.0;
const FRAME_DURATION: Duration = Duration::from_millis(15);

#[derive(Clone)]
pub struct FullHouseProjectile {
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    delay: Duration,
    duration: Duration,
    scale: f32,
    progress: f32,
    started: bool,
}

impl FullHouseProjectile {
    pub fn new(
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        delay: Duration,
        duration: Duration,
        scale: f32,
    ) -> Self {
        Self {
            start_xy,
            end_xy,
            created_at,
            delay,
            duration,
            scale,
            progress: 0.0,
            started: false,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let elapsed = now - self.created_at;
        self.started = elapsed >= self.delay;
        self.progress = if self.started {
            ((elapsed - self.delay).as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if !self.started || self.progress >= 1.0 {
            return sprites;
        }

        let frames = &crate::atlas::generated::ATTACK_FULL_HOUSE_PROJECTILE;
        let elapsed = Duration::from_secs_f32(self.progress * self.duration.as_secs_f32());
        let frame_index = Self::animation_frame_index(elapsed, frames.len());
        let frame = frames[frame_index];
        let tile_width = crate::game_state::TILE_PX_SIZE.width.as_f32();
        let tile_height = crate::game_state::TILE_PX_SIZE.height.as_f32();
        let motion_progress = ATTACK_FRAME_EASE_OUT.sample(self.progress);
        let position = Self::path_position(self.start_xy, self.end_xy, motion_progress);
        let angle = Self::path_angle(self.start_xy, self.end_xy, motion_progress);
        let center_x = px(position.0 * tile_width);
        let center_y = px(position.1 * tile_height);
        let scale = tile_width * self.scale / frame.width().as_f32();
        sprites.push(
            crate::game_state::field_particle::atlas::centered_rotated_sprite(
                frame,
                center_x,
                center_y,
                scale,
                angle,
                Some(Color::WHITE),
            ),
        );
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.delay + self.duration
    }

    fn animation_frame_index(elapsed: Duration, frame_count: usize) -> usize {
        (elapsed.as_secs_f32() / FRAME_DURATION.as_secs_f32()).floor() as usize % frame_count
    }

    pub fn path_position(start_xy: (f32, f32), end_xy: (f32, f32), progress: f32) -> (f32, f32) {
        let progress = progress.clamp(0.0, 1.0);
        let inverse = 1.0 - progress;
        let delta_x = end_xy.0 - start_xy.0;
        let control_1 = (start_xy.0 + delta_x / 3.0, start_xy.1 - ARC_HEIGHT_TILES);
        let control_2 = (end_xy.0 - delta_x / 3.0, end_xy.1 - ARC_HEIGHT_TILES);
        let start_weight = inverse * inverse * inverse;
        let control_1_weight = 3.0 * inverse * inverse * progress;
        let control_2_weight = 3.0 * inverse * progress * progress;
        let end_weight = progress * progress * progress;

        (
            start_xy.0 * start_weight
                + control_1.0 * control_1_weight
                + control_2.0 * control_2_weight
                + end_xy.0 * end_weight,
            start_xy.1 * start_weight
                + control_1.1 * control_1_weight
                + control_2.1 * control_2_weight
                + end_xy.1 * end_weight,
        )
    }

    pub fn path_angle(start_xy: (f32, f32), end_xy: (f32, f32), progress: f32) -> f32 {
        let progress = progress.clamp(0.0, 1.0);
        let inverse = 1.0 - progress;
        let delta_x = end_xy.0 - start_xy.0;
        let control_1 = (start_xy.0 + delta_x / 3.0, start_xy.1 - ARC_HEIGHT_TILES);
        let control_2 = (end_xy.0 - delta_x / 3.0, end_xy.1 - ARC_HEIGHT_TILES);
        let tangent_x = 3.0
            * (inverse * inverse * (control_1.0 - start_xy.0)
                + 2.0 * inverse * progress * (control_2.0 - control_1.0)
                + progress * progress * (end_xy.0 - control_2.0));
        let tangent_y = 3.0
            * (inverse * inverse * (control_1.1 - start_xy.1)
                + 2.0 * inverse * progress * (control_2.1 - control_1.1)
                + progress * progress * (end_xy.1 - control_2.1));
        (tangent_y * crate::game_state::TILE_PX_SIZE.height.as_f32())
            .atan2(tangent_x * crate::game_state::TILE_PX_SIZE.width.as_f32())
    }

    pub fn time_progress_for_path_progress(path_progress: f32) -> f32 {
        let target = path_progress.clamp(0.0, 1.0);
        let mut low = 0.0;
        let mut high = 1.0;
        for _ in 0..16 {
            let middle = (low + high) * 0.5;
            if ATTACK_FRAME_EASE_OUT.sample(middle) < target {
                low = middle;
            } else {
                high = middle;
            }
        }
        (low + high) * 0.5
    }
}

impl particle::Particle for FullHouseProjectile {
    fn tick(&mut self, now: Instant, dt: Duration) {
        FullHouseProjectile::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        FullHouseProjectile::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        FullHouseProjectile::is_done(self, now)
    }
}

#[cfg(test)]
mod tests {
    use super::{FRAME_DURATION, FullHouseProjectile};
    use namui::Duration;

    #[test]
    fn path_starts_and_ends_at_the_requested_points() {
        let start_xy = (1.0, 4.0);
        let end_xy = (9.0, 2.0);

        assert_eq!(
            FullHouseProjectile::path_position(start_xy, end_xy, 0.0),
            start_xy
        );
        assert_eq!(
            FullHouseProjectile::path_position(start_xy, end_xy, 1.0),
            end_xy
        );
    }

    #[test]
    fn path_rises_before_falling_toward_the_landing_point() {
        let start_xy = (1.0, 4.0);
        let end_xy = (9.0, 4.0);
        let apex_xy = FullHouseProjectile::path_position(start_xy, end_xy, 0.5);

        assert!(apex_xy.1 < start_xy.1);
        assert!(FullHouseProjectile::path_angle(start_xy, end_xy, 0.0).sin() < 0.0);
        assert!(FullHouseProjectile::path_angle(start_xy, end_xy, 1.0).sin() > 0.0);
    }

    #[test]
    fn time_progress_matches_the_eased_path_progress() {
        let path_progress = 0.5;
        let time_progress = FullHouseProjectile::time_progress_for_path_progress(path_progress);

        assert!(time_progress < path_progress);
        assert!(
            (super::super::super::ATTACK_FRAME_EASE_OUT.sample(time_progress) - path_progress)
                .abs()
                < 0.001
        );
    }

    #[test]
    fn animation_frames_repeat_from_the_first_frame() {
        let frame_duration = FRAME_DURATION.as_millis();
        let frame_count = crate::atlas::generated::ATTACK_FULL_HOUSE_PROJECTILE.len();
        let cycle_duration = frame_duration * frame_count as i64;

        for frame_index in 0..frame_count {
            let elapsed =
                Duration::from_millis(cycle_duration + frame_index as i64 * frame_duration);
            assert_eq!(
                FullHouseProjectile::animation_frame_index(elapsed, frame_count),
                frame_index
            );
        }
    }
}
