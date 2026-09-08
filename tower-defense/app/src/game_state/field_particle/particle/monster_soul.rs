use crate::game_state::field_particle::atlas;
use namui::*;

const SOUL_OPACITY_START: f32 = 0.75;
const SOUL_OFFSET_MAX_PX: f32 = 128.0;
const SOUL_SCALE_MIN: f32 = 0.0;
const SOUL_SCALE_MAX: f32 = 1.0;

#[derive(Clone)]
pub struct MonsterSoulParticle {
    pub position: Xy<Px>,
    pub created_at: Instant,
    pub duration: Duration,
    pub rotation: Angle,
    pub opacity: f32,
    pub scale: Xy<f32>,
    pub offset: Px,
}

impl MonsterSoulParticle {
    pub fn new(position: Xy<Px>, now: Instant, rotation: Angle) -> Self {
        Self {
            position,
            created_at: now,
            duration: 0.6.sec(),
            rotation,
            opacity: SOUL_OPACITY_START,
            scale: Xy::single(SOUL_SCALE_MIN),
            offset: 0.px(),
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }

    pub fn tick(&mut self, now: Instant, _delta_time: Duration) {
        let elapsed = now - self.created_at;
        let progress = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);

        let remain = 1.0 - progress;
        let remain_pow4 = remain * remain * remain * remain; // (1 - t)^4
        let eased = 1.0 - remain_pow4; // easeOutQuart(t)

        // Opacity: 0.75 -> 0.0 following (1 - t)^4
        self.opacity = SOUL_OPACITY_START * remain_pow4;

        // Offset: 0 -> MAX following easeOutQuart
        self.offset = px(SOUL_OFFSET_MAX_PX * eased);

        // Scale: MIN -> MAX following easeOutQuart
        let scale_v = SOUL_SCALE_MIN + (SOUL_SCALE_MAX - SOUL_SCALE_MIN) * eased;
        self.scale = Xy::single(scale_v);
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        let scale_value = self.scale.x;
        let alpha = (self.opacity * 255.0) as u8;
        let color = Color::WHITE.with_alpha(alpha);
        let angle_rad = self.rotation.as_radians();
        let src_rect = atlas::monster_soul();

        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();
        let offset_f = self.offset.as_f32();
        let cx = self.position.x + px(sin_a * offset_f);
        let cy = self.position.y - px(cos_a * offset_f);

        sprites.push(atlas::centered_rotated_sprite(
            src_rect,
            cx,
            cy,
            scale_value,
            angle_rad,
            Some(color),
        ));
        sprites
    }
}

impl namui::particle::Particle for MonsterSoulParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        MonsterSoulParticle::tick(self, now, dt);
    }
    fn render(&self) -> namui::particle::ParticleSprites {
        MonsterSoulParticle::render(self)
    }
    fn is_done(&self, now: Instant) -> bool {
        MonsterSoulParticle::is_done(self, now)
    }
}
