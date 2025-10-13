use crate::icon::Icon;
use namui::*;

#[derive(Clone, State)]
pub struct IconParticle {
    pub icon: Icon,
    pub xy: Xy<Px>,
    pub rotation: Angle,
    pub behavior: IconParticleBehavior,
}
impl IconParticle {
    pub fn tick(&mut self, now: Instant, dt: Duration) {
        match &mut self.behavior {
            IconParticleBehavior::FadeRise {
                duration,
                speed,
                created_at,
                initial_opacity,
            } => {
                let elapsed = now - *created_at;
                let progress = (elapsed.as_secs_f64() / duration.as_secs_f64()) as f32;

                self.xy.y -= px(*speed * dt.as_secs_f32());

                self.icon.opacity = *initial_opacity * (1.0_f32 - progress).max(0.0_f32);
            }
            IconParticleBehavior::Physics {
                velocity,
                angular_velocity,
                created_at,
                duration,
                scale,
                air_resistance,
                angular_resistance,
                gravity_acceleration_per_second,
                ..
            } => {
                let elapsed = now - *created_at;
                let progress =
                    (elapsed.as_secs_f64() / duration.as_secs_f64()).clamp(0.0, 1.0) as f32;
                let mut delta_position_per_second = *velocity * 1.sec();

                let gravity = *gravity_acceleration_per_second * 1.sec() * dt.as_secs_f32();
                delta_position_per_second.y += gravity;

                let resistance = 1.0 - (*air_resistance * dt);
                delta_position_per_second *= resistance;

                *velocity = Per::new(delta_position_per_second, 1.sec());

                let v = *velocity * dt;
                self.xy.x += v.x;
                self.xy.y += v.y;

                let mut delta_rotation_per_second = *angular_velocity * 1.sec();
                let ang_resistance = 1.0 - (*angular_resistance * dt);
                delta_rotation_per_second *= ang_resistance;
                *angular_velocity = Per::new(delta_rotation_per_second, 1.sec());

                self.rotation += *angular_velocity * dt;

                let eased = fast_in_slow_out_easing(progress);
                *scale = eased;
                self.icon.opacity = eased;
            }
        }
    }

    pub fn render(&self) -> RenderingTree {
        let half_wh = self.icon.wh / 2.0;
        namui::translate(
            self.xy.x,
            self.xy.y,
            namui::rotate(
                self.rotation,
                namui::translate(
                    -half_wh.width,
                    -half_wh.height,
                    self.icon.to_rendering_tree(),
                ),
            ),
        )
    }

    pub fn is_done(&self, now: Instant) -> bool {
        match &self.behavior {
            IconParticleBehavior::FadeRise {
                duration,
                created_at,
                ..
            } => {
                let elapsed = now - *created_at;
                elapsed >= *duration
            }
            IconParticleBehavior::Physics {
                created_at,
                duration,
                ..
            } => {
                let elapsed = now - *created_at;
                elapsed >= *duration
            }
        }
    }
}

#[derive(Clone, State)]
pub enum IconParticleBehavior {
    FadeRise {
        duration: Duration,
        speed: f32,
        created_at: Instant,
        initial_opacity: f32,
    },
    Physics {
        duration: Duration,
        created_at: Instant,
        velocity: Per<Xy<Px>, Duration>,
        angular_velocity: Per<Angle, Duration>,
        scale: f32,
        air_resistance: Per<f32, Duration>,
        angular_resistance: Per<f32, Duration>,
        gravity_acceleration_per_second: Per<Px, Duration>,
    },
}

fn fast_in_slow_out_easing(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t < 0.1 {
        t / 0.1
    } else {
        (1.0 - (t - 0.1) / 0.9).max(0.0)
    }
}
