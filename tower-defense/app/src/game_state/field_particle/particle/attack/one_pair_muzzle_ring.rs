use crate::game_state::TILE_PX_SIZE;
use namui::*;

const DURATION: Duration = Duration::from_millis(500);
const SCALE_MULTIPLIER: f32 = 2.5;

#[derive(Clone)]
pub struct OnePairMuzzleRing {
    center_x_px: f32,
    center_y_px: f32,
    rotation_cos: f32,
    rotation_sin: f32,
    scale: f32,
    created_at: Instant,
    progress: f32,
}

impl OnePairMuzzleRing {
    pub fn new(
        start_xy: (f32, f32),
        rotation_cos: f32,
        rotation_sin: f32,
        created_at: Instant,
        scale: f32,
    ) -> Self {
        Self {
            center_x_px: start_xy.0 * TILE_PX_SIZE.width.as_f32(),
            center_y_px: start_xy.1 * TILE_PX_SIZE.height.as_f32(),
            rotation_cos,
            rotation_sin,
            scale,
            created_at,
            progress: 0.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.progress =
            ((now - self.created_at).as_secs_f32() / DURATION.as_secs_f32()).clamp(0.0, 1.0);
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.progress >= 1.0 {
            return sprites;
        }

        let frames = crate::atlas::generated::ATTACK_ONE_PAIR_MUZZLE_RING;
        let src_rect = frames[frame_index_for_progress(self.progress, frames.len())];
        let sprite_width = src_rect.width().as_f32();
        let sprite_height = src_rect.height().as_f32();
        let scale = self.scale * SCALE_MULTIPLIER;
        let scos = self.rotation_cos * scale;
        let ssin = self.rotation_sin * scale;
        let (center_x_px, center_y_px) = center_for_backward_endpoint(
            (self.center_x_px, self.center_y_px),
            self.rotation_cos,
            self.rotation_sin,
            sprite_width,
            scale,
        );
        let center_x = px(center_x_px);
        let center_y = px(center_y_px);

        sprites.push(namui::ImageSprite {
            src_rect,
            xform: namui::RSXform {
                scos,
                ssin,
                tx: center_x - px(scos * sprite_width / 2.0 - ssin * sprite_height / 2.0),
                ty: center_y - px(ssin * sprite_width / 2.0 + scos * sprite_height / 2.0),
            },
            color: Some(Color::WHITE.with_alpha(((1.0 - self.progress) * 255.0).round() as u8)),
        });
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= DURATION
    }
}

fn frame_index_for_progress(progress: f32, frame_count: usize) -> usize {
    ((super::ATTACK_FRAME_EASE_OUT.sample(progress) * frame_count as f32) as usize)
        .min(frame_count - 1)
}

fn center_for_backward_endpoint(
    anchor_xy: (f32, f32),
    rotation_cos: f32,
    rotation_sin: f32,
    sprite_width: f32,
    scale: f32,
) -> (f32, f32) {
    let half_width = sprite_width * scale / 2.0;
    (
        anchor_xy.0 + rotation_cos * half_width,
        anchor_xy.1 + rotation_sin * half_width,
    )
}

impl particle::Particle for OnePairMuzzleRing {
    fn tick(&mut self, now: Instant, dt: Duration) {
        OnePairMuzzleRing::tick(self, now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        OnePairMuzzleRing::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        OnePairMuzzleRing::is_done(self, now)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn muzzle_ring_frames_advance_with_ease_out_progress() {
        assert_eq!(frame_index_for_progress(0.0, 7), 0);
        assert!(frame_index_for_progress(0.2, 7) > 1);
        assert_eq!(frame_index_for_progress(0.8, 7), 6);
        assert_eq!(frame_index_for_progress(1.0, 7), 6);
    }

    #[test]
    fn muzzle_ring_backward_edge_anchors_at_start() {
        let anchor_xy = (180.0, 240.0);
        let rotation_cos = 0.6;
        let rotation_sin = 0.8;
        let sprite_width = 64.0;
        let scale = 1.1 * SCALE_MULTIPLIER;
        let center_xy = center_for_backward_endpoint(
            anchor_xy,
            rotation_cos,
            rotation_sin,
            sprite_width,
            scale,
        );
        let half_width = sprite_width * scale / 2.0;

        assert!((center_xy.0 - rotation_cos * half_width - anchor_xy.0).abs() < 0.001);
        assert!((center_xy.1 - rotation_sin * half_width - anchor_xy.1).abs() < 0.001);
    }
}
