use super::super::ATTACK_FRAME_EASE_OUT;
use namui::*;

const ARC_HEIGHT_TILES: f32 = 2.0;
const FRAME_DURATION: Duration = Duration::from_millis(15);

#[derive(Clone, Copy)]
struct RenderState {
    frame: Rect<Px>,
    center_x: Px,
    center_y: Px,
    scale: f32,
    rotation_cos: f32,
    rotation_sin: f32,
}

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
    render_state: Option<RenderState>,
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
            render_state: None,
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
        self.render_state = if !self.started || self.progress >= 1.0 {
            None
        } else {
            let frames = &crate::atlas::generated::ATTACK_FULL_HOUSE_PROJECTILE;
            let frame_elapsed =
                Duration::from_secs_f32(self.progress * self.duration.as_secs_f32());
            let frame = frames[Self::animation_frame_index(frame_elapsed, frames.len())];
            let tile_width = crate::game_state::TILE_PX_SIZE.width.as_f32();
            let tile_height = crate::game_state::TILE_PX_SIZE.height.as_f32();
            let motion_progress = ATTACK_FRAME_EASE_OUT.sample(self.progress);
            let position = Self::path_position(self.start_xy, self.end_xy, motion_progress);
            let angle = Self::path_angle(self.start_xy, self.end_xy, motion_progress);
            Some(RenderState {
                frame,
                center_x: px(position.0 * tile_width),
                center_y: px(position.1 * tile_height),
                scale: tile_width * self.scale / frame.width().as_f32(),
                rotation_cos: angle.cos(),
                rotation_sin: angle.sin(),
            })
        };
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        let Some(render_state) = self.render_state else {
            return sprites;
        };
        sprites.push(crate::atlas::centered_rotated_sprite_with_sin_cos(
            render_state.frame,
            render_state.center_x,
            render_state.center_y,
            render_state.scale,
            render_state.rotation_cos,
            render_state.rotation_sin,
            Some(Color::WHITE),
        ));
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
        let delta_x = end_xy.0 - start_xy.0;
        let delta_y = end_xy.1 - start_xy.1;
        let eased_y_progress = progress * progress * (3.0 - 2.0 * progress);

        (
            start_xy.0 + delta_x * progress,
            start_xy.1 + delta_y * eased_y_progress
                - 3.0 * ARC_HEIGHT_TILES * progress * (1.0 - progress),
        )
    }

    pub fn path_angle(start_xy: (f32, f32), end_xy: (f32, f32), progress: f32) -> f32 {
        let progress = progress.clamp(0.0, 1.0);
        let inverse = 1.0 - progress;
        let delta_x = end_xy.0 - start_xy.0;
        let delta_y = end_xy.1 - start_xy.1;
        let tangent_y = 3.0
            * (-ARC_HEIGHT_TILES * inverse * inverse
                + 2.0 * delta_y * inverse * progress
                + ARC_HEIGHT_TILES * progress * progress);
        (tangent_y * crate::game_state::TILE_PX_SIZE.height.as_f32())
            .atan2(delta_x * crate::game_state::TILE_PX_SIZE.width.as_f32())
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
    use namui::{Duration, Instant, px};

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
    fn horizontal_path_position_is_linear() {
        let start_xy = (1.5, 4.0);
        let end_xy = (9.0, 6.0);
        let progress = 0.4;

        assert_eq!(
            FullHouseProjectile::path_position(start_xy, end_xy, progress).0,
            start_xy.0 + (end_xy.0 - start_xy.0) * progress
        );
    }

    #[test]
    fn tick_caches_projectile_render_geometry() {
        let created_at = Instant::now();
        let mut projectile = FullHouseProjectile::new(
            (1.0, 4.0),
            (9.0, 2.0),
            created_at,
            Duration::ZERO,
            Duration::from_millis(100),
            1.0,
        );
        projectile.tick(created_at + Duration::from_millis(50), Duration::ZERO);

        let render_state = projectile.render_state.unwrap();
        let motion_progress = super::super::super::ATTACK_FRAME_EASE_OUT.sample(0.5);
        let position = FullHouseProjectile::path_position((1.0, 4.0), (9.0, 2.0), motion_progress);
        assert_eq!(
            render_state.center_x,
            px(position.0 * crate::game_state::TILE_PX_SIZE.width.as_f32())
        );
        assert_eq!(
            render_state.center_y,
            px(position.1 * crate::game_state::TILE_PX_SIZE.height.as_f32())
        );
        assert_eq!(projectile.render().len(), 1);
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
