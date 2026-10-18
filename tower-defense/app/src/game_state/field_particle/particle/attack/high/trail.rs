use namui::*;

const DURATION: Duration = Duration::from_millis(125);

#[derive(Clone)]
pub struct Trail {
    start_x_px: f32,
    start_y_px: f32,
    length_px: f32,
    position_progress: f32,
    rotation_cos: f32,
    rotation_sin: f32,
    created_at: Instant,
    delay: Duration,
    progress: f32,
    started: bool,
    scale_variation: f32,
}

impl Trail {
    pub const INITIAL_SCALE: f32 = 0.72;
    pub const MAX_SCALE: f32 = 1.5;
    pub const SPACING_RATIO: f32 = 0.75;

    pub fn new(
        start_xy: (f32, f32),
        length_px: f32,
        position_progress: f32,
        rotation_cos: f32,
        rotation_sin: f32,
        created_at: Instant,
        delay: Duration,
    ) -> Self {
        let tile_size = crate::game_state::TILE_PX_SIZE;
        Self {
            start_x_px: start_xy.0 * tile_size.width.as_f32(),
            start_y_px: start_xy.1 * tile_size.height.as_f32(),
            length_px,
            position_progress,
            rotation_cos,
            rotation_sin,
            created_at,
            delay,
            progress: 0.0,
            started: false,
            scale_variation: 1.0,
        }
    }

    pub fn with_scale_variation(mut self, scale_variation: f32) -> Self {
        self.scale_variation = scale_variation;
        self
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let elapsed = now - self.created_at;
        self.started = elapsed >= self.delay;
        self.progress = if self.started {
            ((elapsed.as_secs_f32() - self.delay.as_secs_f32()) / DURATION.as_secs_f32())
                .clamp(0.0, 1.0)
        } else {
            0.0
        };
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if !self.started || self.progress >= 1.0 {
            return sprites;
        }

        let alpha = 1.0 - self.progress;
        let frames = crate::atlas::generated::ATTACK_HIGH_TRAIL;
        let src_rect = frames[frame_index_for_progress(self.progress, frames.len())];
        let sprite_width = src_rect.width().as_f32();
        let sprite_height = src_rect.height().as_f32();
        let desired_scale = (Self::INITIAL_SCALE
            + (Self::MAX_SCALE - Self::INITIAL_SCALE) * self.progress)
            * self.scale_variation;
        let (scale, center_offset_px) = trail_scale_and_center_offset(
            self.length_px,
            sprite_width,
            desired_scale,
            self.position_progress,
        );
        let scos = self.rotation_cos * scale;
        let ssin = self.rotation_sin * scale;
        let center_x = px(self.start_x_px + self.rotation_cos * center_offset_px);
        let center_y = px(self.start_y_px + self.rotation_sin * center_offset_px);

        sprites.push(namui::ImageSprite {
            src_rect,
            xform: namui::RSXform {
                scos,
                ssin,
                tx: center_x - px(scos * sprite_width / 2.0 - ssin * sprite_height / 2.0),
                ty: center_y - px(ssin * sprite_width / 2.0 + scos * sprite_height / 2.0),
            },
            color: Some(Color::WHITE.with_alpha((alpha * 96.0).round() as u8)),
        });
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.delay + DURATION
    }
}

fn frame_index_for_progress(progress: f32, frame_count: usize) -> usize {
    ((progress * frame_count as f32) as usize).min(frame_count - 1)
}

fn trail_scale_and_center_offset(
    length_px: f32,
    sprite_width: f32,
    desired_scale: f32,
    position_progress: f32,
) -> (f32, f32) {
    let scale = desired_scale.min(length_px / sprite_width);
    let sprite_width = sprite_width * scale;
    let center_offset = sprite_width / 2.0 + position_progress * (length_px - sprite_width);
    (scale, center_offset)
}

impl particle::Particle for Trail {
    fn tick(&mut self, now: Instant, dt: Duration) {
        Trail::tick(self, now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        Trail::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        Trail::is_done(self, now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trail_frames_advance_with_progress() {
        assert_eq!(frame_index_for_progress(0.0, 6), 0);
        assert_eq!(frame_index_for_progress(0.2, 6), 1);
        assert_eq!(frame_index_for_progress(0.4, 6), 2);
        assert_eq!(frame_index_for_progress(0.6, 6), 3);
        assert_eq!(frame_index_for_progress(0.8, 6), 4);
        assert_eq!(frame_index_for_progress(0.99, 6), 5);
    }

    #[test]
    fn trail_sprite_edges_align_with_path_endpoints() {
        let length_px = 400.0;
        let sprite_width = 128.0;
        let desired_scale = 1.5;
        let (start_scale, start_offset) =
            trail_scale_and_center_offset(length_px, sprite_width, desired_scale, 0.0);
        let (end_scale, end_offset) =
            trail_scale_and_center_offset(length_px, sprite_width, desired_scale, 1.0);

        assert!((start_offset - sprite_width * start_scale / 2.0).abs() < 0.001);
        assert!((end_offset + sprite_width * end_scale / 2.0 - length_px).abs() < 0.001);
    }

    #[test]
    fn trail_sprite_shrinks_to_fit_short_paths() {
        let length_px = 32.0;
        let sprite_width = 128.0;
        let (scale, center_offset) =
            trail_scale_and_center_offset(length_px, sprite_width, Trail::MAX_SCALE, 0.0);

        assert!((scale * sprite_width - length_px).abs() < 0.001);
        assert!((center_offset - length_px / 2.0).abs() < 0.001);
    }
}
