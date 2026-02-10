use crate::MapCoordF32;
use crate::game_state::projectile::ProjectileKind;
use namui::*;

#[derive(Clone)]
pub struct ProjectileParticle {
    pub xy: MapCoordF32,
    pub kind: ProjectileKind,
    pub rotation: Angle,
    pub rotation_speed: Angle,
    pub velocity: Xy<f32>,
    pub created_at: Instant,
    pub duration: Duration,
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
        }
    }

    pub fn tick(&mut self, _now: Instant, dt: Duration) {
        // Update rotation
        self.rotation += self.rotation_speed * dt.as_secs_f32();

        // Update position based on velocity vector (tiles/second)
        self.xy += self.velocity * dt.as_secs_f32();
    }

    pub fn is_alive(&self, now: Instant) -> bool {
        now - self.created_at < self.duration
    }

    pub fn render_particle(&self) -> RenderingTree {
        let tile_px_size = crate::game_state::TILE_PX_SIZE;
        let projectile_wh = tile_px_size * Wh::new(0.4, 0.4);
        let image = self.kind.image();
        let half_wh = projectile_wh / 2.0;
        let tile_px = tile_px_size.to_xy();
        let particle_px_xy = tile_px * Xy::new(self.xy.x, self.xy.y);
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
                            paint: None,
                        },
                    }),
                ),
            ),
        )
    }
}

impl namui::particle::Particle for ProjectileParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        ProjectileParticle::tick(self, now, dt);
    }
    fn render(&self) -> RenderingTree {
        ProjectileParticle::render_particle(self)
    }
    fn is_done(&self, now: Instant) -> bool {
        !self.is_alive(now)
    }
}
