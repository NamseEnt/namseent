use super::super::ATTACK_FRAME_EASE_OUT;
use namui::*;

const DURATION: Duration = Duration::from_millis(100);
const PROJECTILE_SCALE: f32 = 2.0;

#[derive(Clone)]
pub struct EnergyBall {
    start_xy: (f32, f32),
    end_xy: (f32, f32),
    created_at: Instant,
    color: [f32; 3],
    progress: f32,
}

impl EnergyBall {
    pub fn new(
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        color: [f32; 3],
        created_at: Instant,
    ) -> Self {
        Self {
            start_xy,
            end_xy,
            created_at,
            color,
            progress: 0.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.progress =
            ((now - self.created_at).as_secs_f32() / DURATION.as_secs_f32()).clamp(0.0, 1.0);
    }

    pub fn render(&self) -> particle::ParticleSprites {
        let mut sprites = particle::ParticleSprites::new();
        if self.progress >= 1.0 {
            return sprites;
        }

        let frames = &crate::atlas::generated::ATTACK_COMMON_ENERGY_BALL_ENERGY_BALL;
        let frame_index = ((ATTACK_FRAME_EASE_OUT.sample(self.progress) * frames.len() as f32)
            as usize)
            .min(frames.len() - 1);
        let frame = frames[frame_index];
        let tile_width = crate::game_state::TILE_PX_SIZE.width.as_f32();
        let tile_height = crate::game_state::TILE_PX_SIZE.height.as_f32();
        let direction_x = (self.end_xy.0 - self.start_xy.0) * tile_width;
        let direction_y = (self.end_xy.1 - self.start_xy.1) * tile_height;
        let angle = direction_y.atan2(direction_x);
        let center_x =
            px((self.start_xy.0 + (self.end_xy.0 - self.start_xy.0) * self.progress) * tile_width);
        let center_y =
            px((self.start_xy.1 + (self.end_xy.1 - self.start_xy.1) * self.progress) * tile_height);
        let scale = tile_width * PROJECTILE_SCALE / frame.width().as_f32();
        let color = Color::from_f01(
            self.color[0].clamp(0.0, 1.0),
            self.color[1].clamp(0.0, 1.0),
            self.color[2].clamp(0.0, 1.0),
            1.0,
        );
        sprites.push(
            crate::game_state::field_particle::atlas::centered_rotated_sprite(
                frame,
                center_x,
                center_y,
                scale,
                angle,
                Some(color),
            ),
        );
        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= DURATION
    }
}

impl particle::Particle for EnergyBall {
    fn tick(&mut self, now: Instant, dt: Duration) {
        EnergyBall::tick(self, now, dt);
    }

    fn render(&self) -> particle::ParticleSprites {
        EnergyBall::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        EnergyBall::is_done(self, now)
    }
}
