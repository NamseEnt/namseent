use crate::MapCoordF32;
use crate::TILE_PX_SIZE;
use crate::game_state::projectile::ProjectileKind;
use namui::*;

#[derive(Clone, State)]
pub struct ProjectileParticle {
    pub xy: MapCoordF32,
    pub kind: ProjectileKind,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub velocity: Xy<f32>,
    pub created_at: Instant,
    pub duration: Duration,
    pub alpha: f32,
}

impl ProjectileParticle {
    pub fn new(
        xy: MapCoordF32,
        kind: ProjectileKind,
        rotation: Angle,
        rotation_speed: Angle,
        velocity: Xy<f32>,
        created_at: Instant,
        duration: Duration,
    ) -> Self {
        Self {
            xy,
            kind,
            rotation,
            rotation_speed,
            velocity,
            created_at,
            duration,
            alpha: 1.0,
        }
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) {
        // Update rotation
        self.rotation += self.rotation_speed * dt.as_secs_f32();

        // Update position based on velocity vector (tiles/second)
        self.xy += self.velocity * dt.as_secs_f32();

        // Fade out linearly by remaining lifetime
        self.alpha = self.remaining_life_ratio(now);
    }

    pub fn render(&self) -> RenderingTree {
        let projectile_wh = TILE_PX_SIZE * Wh::new(0.4, 0.4);
        let image = self.kind.image();
        let half_wh = projectile_wh / 2.0;

        let tile_px = TILE_PX_SIZE.to_xy();
        let particle_px_xy = tile_px * Xy::new(self.xy.x, self.xy.y);

        let paint = Paint::new(Color::WHITE.with_alpha((self.alpha * 255.0) as u8));

        namui::translate(
            particle_px_xy.x,
            particle_px_xy.y,
            namui::rotate(
                self.rotation,
                namui::translate(
                    -half_wh.width,
                    -half_wh.height,
                    namui::image(ImageParam {
                        rect: Rect::from_xy_wh(Xy::zero(), projectile_wh),
                        image,
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: Some(paint),
                        },
                    }),
                ),
            ),
        )
    }

    pub fn is_alive(&self, now: Instant) -> bool {
        now - self.created_at < self.duration
    }

    fn remaining_life_ratio(&self, now: Instant) -> f32 {
        let duration_secs = self.duration.as_secs_f32();
        if duration_secs <= f32::EPSILON {
            return 0.0;
        }

        let elapsed = now - self.created_at;
        let progress = (elapsed.as_secs_f32() / duration_secs).clamp(0.0, 1.0);
        1.0 - progress
    }
}
