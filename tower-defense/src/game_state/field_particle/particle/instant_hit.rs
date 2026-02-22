use crate::game_state::field_particle::atlas;
use crate::game_state::{TILE_PX_SIZE, attack};
use namui::*;

#[derive(Clone)]
pub struct InstantHitParticle {
    pub xy: (f32, f32),
    pub created_at: Instant,
    pub kind: attack::instant_effect::InstantEffectKind,
    pub scale: f32,
    pub progress: f32,
    pub current_scale: f32,
    pub alpha: f32,
}

impl InstantHitParticle {
    pub fn new(
        xy: (f32, f32),
        created_at: Instant,
        kind: attack::instant_effect::InstantEffectKind,
        scale: f32,
    ) -> Self {
        Self {
            xy,
            created_at,
            kind,
            scale,
            progress: 0.0,
            current_scale: scale,
            alpha: 1.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.progress = self.progress(now);
        self.current_scale = self.current_scale(now);
        self.alpha = self.current_alpha(now);
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.alpha <= 0.0 {
            return sprites;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);

        let sprite = match self.kind {
            attack::instant_effect::InstantEffectKind::Explosion => {
                let scale = (32.0 * self.current_scale * 2.0) / 128.0;
                let color = Color::from_f01(1.0, 0.5, 0.0, self.alpha);
                atlas::centered_sprite(atlas::glow_circle(), xy_px.x, xy_px.y, scale, Some(color))
            }
            attack::instant_effect::InstantEffectKind::Lightning => {
                let scale = (24.0 * self.current_scale * 2.0) / 128.0;
                let color = Color::from_f01(1.0, 1.0, 0.2, self.alpha);
                atlas::centered_sprite(atlas::cross(), xy_px.x, xy_px.y, scale, Some(color))
            }
            attack::instant_effect::InstantEffectKind::MagicCircle => {
                let scale = (28.0 * self.current_scale * 2.0) / 128.0;
                let color = Color::from_f01(0.5, 0.2, 1.0, self.alpha);
                atlas::centered_sprite(atlas::ring(), xy_px.x, xy_px.y, scale, Some(color))
            }
        };
        sprites.push(sprite);
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= attack::instant_effect::EFFECT_LIFETIME
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / attack::instant_effect::EFFECT_LIFETIME.as_secs_f32()).min(1.0)
    }

    fn current_scale(&self, now: Instant) -> f32 {
        let progress = self.progress(now);
        if progress < 0.2 {
            self.scale * (progress / 0.2)
        } else {
            self.scale * (1.0 - (progress - 0.2) / 0.8)
        }
    }

    fn current_alpha(&self, now: Instant) -> f32 {
        let progress = self.progress(now);
        if progress < 0.1 {
            progress / 0.1
        } else {
            1.0 - (progress - 0.1) / 0.9
        }
    }
}

impl namui::particle::Particle for InstantHitParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        InstantHitParticle::tick(self, now, dt);
    }
    fn render(&self) -> namui::particle::ParticleSprites {
        InstantHitParticle::render(self)
    }
    fn is_done(&self, now: Instant) -> bool {
        InstantHitParticle::is_done(self, now)
    }
}
