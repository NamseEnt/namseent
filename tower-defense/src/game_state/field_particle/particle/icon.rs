use crate::icon::Icon;
use namui::*;

#[derive(Clone)]
pub struct IconParticle {
    pub icon: Icon,
    pub xy: Xy<Px>,
    pub behavior: IconParticleBehavior,
}
impl IconParticle {
    pub fn tick(&mut self, now: Instant, dt: Duration) {
        match &self.behavior {
            IconParticleBehavior::FadeRise {
                duration,
                speed,
                created_at,
                initial_opacity,
            } => {
                let elapsed = now - *created_at;
                let progress = (elapsed.as_secs_f64() / duration.as_secs_f64()) as f32;

                // Move upward
                self.xy.y -= px(speed * dt.as_secs_f32());

                // Fade out over time
                self.icon.opacity = initial_opacity * (1.0_f32 - progress).max(0.0_f32);
            }
        }
    }

    pub fn render(&self) -> RenderingTree {
        namui::translate(self.xy.x, self.xy.y, self.icon.to_rendering_tree())
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
        }
    }
}

#[derive(Clone)]
pub enum IconParticleBehavior {
    FadeRise {
        duration: Duration,
        speed: f32,
        created_at: Instant,
        initial_opacity: f32,
    },
}
