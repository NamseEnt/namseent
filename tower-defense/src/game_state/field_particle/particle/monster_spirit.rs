use crate::asset_loader::get_particle_asset;
use crate::game_state::field_particle::particle_kind::ParticleKind;
use namui::*;

#[derive(Clone)]
pub struct MonsterSpiritParticle {
    pub xy: Xy<Px>,
    pub velocity: Per<Xy<Px>, Duration>,
    pub size: Px,
    pub opacity: f32,
    pub scale: f32,
    pub angle: Angle,
    pub behavior: MonsterSpiritBehavior,
}

#[derive(Clone)]
pub struct MonsterSpiritBehavior {
    pub duration: Duration,
    pub created_at: Instant,
    pub initial_scale: f32,
    pub initial_velocity: Per<Xy<Px>, Duration>,
}

impl MonsterSpiritParticle {
    pub fn new(
        xy: Xy<Px>,
        angle: Angle,
        speed: f32,
        size: Px,
        duration: Duration,
        now: Instant,
    ) -> Self {
        // Add offset to make vertical up direction (0 degrees) the base
        let velocity_angle = angle + (-90.0_f32).deg();
        let velocity_x = velocity_angle.cos() * speed;
        let velocity_y = velocity_angle.sin() * speed;
        let velocity = Per::new(Xy::new(px(velocity_x), px(velocity_y)), 1.sec());

        Self {
            xy,
            velocity,
            size,
            opacity: 1.0,
            scale: 0.0,
            angle,
            behavior: MonsterSpiritBehavior {
                duration,
                created_at: now,
                initial_scale: 0.0,
                initial_velocity: velocity,
            },
        }
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) {
        let elapsed = now - self.behavior.created_at;
        let progress =
            (elapsed.as_secs_f64() / self.behavior.duration.as_secs_f64()).clamp(0.0, 1.0) as f32;

        // Ease out movement (fast start, slow end)
        let eased_progress = ease_out(progress);
        let velocity_multiplier = 1.0 - eased_progress;

        // Move particle with eased velocity
        let v = self.behavior.initial_velocity * dt * velocity_multiplier;
        self.xy.x += v.x;
        self.xy.y += v.y;

        // Fade out over time
        self.opacity = (1.0 - progress).max(0.0);

        // Scale animation: 0 -> 1 (first 30%) -> 0 (last 70%)
        self.scale = if progress < 0.3 {
            // Fast scale up in first 30%
            let scale_progress = progress / 0.3;
            self.behavior.initial_scale + scale_progress * (1.0 - self.behavior.initial_scale)
        } else {
            // Slow scale down in last 70%
            let scale_progress = (progress - 0.3) / 0.7;
            1.0 - scale_progress
        };
    }

    pub fn render(&self) -> RenderingTree {
        let Some(spirit_image) = get_particle_asset(ParticleKind::MonsterSpirit) else {
            return RenderingTree::Empty;
        };

        let scaled_size = self.size * self.scale;
        let half_size = scaled_size / 2.0;

        namui::translate(
            self.xy.x,
            self.xy.y,
            namui::rotate(
                self.angle,
                namui::translate(
                    -half_size,
                    -half_size,
                    namui::image(ImageParam {
                        rect: Rect::from_xy_wh(Xy::zero(), Wh::single(scaled_size)),
                        image: spirit_image,
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: Some(Paint {
                                color: Color::from_u8(255, 255, 255, (self.opacity * 255.0) as u8),
                                ..Default::default()
                            }),
                        },
                    }),
                ),
            ),
        )
    }

    pub fn is_done(&self, now: Instant) -> bool {
        let elapsed = now - self.behavior.created_at;
        elapsed >= self.behavior.duration
    }
}

fn ease_out(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    1.0 - (1.0 - t) * (1.0 - t)
}
