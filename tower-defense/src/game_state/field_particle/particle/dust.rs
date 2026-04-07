use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;

const DUST_SPRITE_SIZE_PX: f32 = 128.0;
const DUST_PARTICLE_MAX_ALPHA: f32 = 0.45;
const DUST_SPEED_DECELERATION_EXPONENT: f32 = 1.4;

#[derive(Clone, Copy)]
pub struct DustParticleConfig {
    pub lifetime: Duration,
    pub scale_in_duration: Duration,
    pub base_size_tile: f32,
    pub final_scale_multiplier: f32,
}

#[derive(Clone, Copy)]
pub struct DustParticleParams {
    pub cluster_spawn_xy: (f32, f32),
    pub cluster_velocity_xy: (f32, f32),
    pub local_spawn_offset_xy: (f32, f32),
    pub local_velocity_xy: (f32, f32),
    pub cluster_rotation_speed_turns_per_sec: f32,
}

#[derive(Clone)]
pub struct DustParticle {
    pub xy: (f32, f32),
    pub cluster_spawn_xy: (f32, f32),
    pub cluster_velocity_xy: (f32, f32),
    pub local_spawn_offset_xy: (f32, f32),
    pub local_velocity_xy: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub scale_in_duration: Duration,
    pub alpha: f32,
    pub size_multiplier: f32,
    pub base_size_tile: f32,
    pub final_scale_multiplier: f32,
    pub rotation_rad: f32,
    pub rotation_speed_rad_per_sec: f32,
}

impl DustParticle {
    pub fn new<R: Rng + ?Sized>(
        params: DustParticleParams,
        now: Instant,
        config: DustParticleConfig,
        rng: &mut R,
    ) -> Self {
        Self {
            xy: (
                params.cluster_spawn_xy.0 + params.local_spawn_offset_xy.0,
                params.cluster_spawn_xy.1 + params.local_spawn_offset_xy.1,
            ),
            cluster_spawn_xy: params.cluster_spawn_xy,
            cluster_velocity_xy: params.cluster_velocity_xy,
            local_spawn_offset_xy: params.local_spawn_offset_xy,
            local_velocity_xy: params.local_velocity_xy,
            created_at: now,
            lifetime: config.lifetime,
            scale_in_duration: config.scale_in_duration,
            alpha: DUST_PARTICLE_MAX_ALPHA,
            size_multiplier: 0.0,
            base_size_tile: config.base_size_tile,
            final_scale_multiplier: config.final_scale_multiplier,
            rotation_rad: rng.gen_range(0.0..std::f32::consts::TAU),
            rotation_speed_rad_per_sec: params.cluster_rotation_speed_turns_per_sec
                * std::f32::consts::TAU,
        }
    }

    fn elapsed_secs(&self, now: Instant) -> f32 {
        (now - self.created_at).as_secs_f32().max(0.0)
    }

    fn progress(&self, now: Instant) -> f32 {
        (self.elapsed_secs(now) / self.lifetime.as_secs_f32()).clamp(0.0, 1.0)
    }

    fn speed_factor(&self, now: Instant) -> f32 {
        let progress = self.progress(now);
        let base_speed_factor = 1.0 - progress.sqrt();
        base_speed_factor
            .powf(DUST_SPEED_DECELERATION_EXPONENT)
            .max(0.0)
    }

    fn current_xy(&self, now: Instant) -> (f32, f32) {
        let elapsed = self.elapsed_secs(now);
        let speed_factor = self.speed_factor(now);

        let cluster_xy = (
            self.cluster_spawn_xy.0 + self.cluster_velocity_xy.0 * elapsed * speed_factor,
            self.cluster_spawn_xy.1 + self.cluster_velocity_xy.1 * elapsed * speed_factor,
        );
        (
            cluster_xy.0
                + self.local_spawn_offset_xy.0
                + self.local_velocity_xy.0 * elapsed * speed_factor,
            cluster_xy.1
                + self.local_spawn_offset_xy.1
                + self.local_velocity_xy.1 * elapsed * speed_factor,
        )
    }

    pub fn tick_impl(&mut self, now: Instant, _dt: Duration) {
        self.xy = self.current_xy(now);

        let progress = self.progress(now);
        let eased_progress = progress.sqrt();
        self.alpha = (DUST_PARTICLE_MAX_ALPHA * (1.0 - eased_progress)).max(0.0);

        let elapsed = now - self.created_at;
        if elapsed <= self.scale_in_duration {
            let t = (elapsed.as_secs_f32() / self.scale_in_duration.as_secs_f32()).clamp(0.0, 1.0);
            self.size_multiplier = t;
        } else {
            let remain_duration = (self.lifetime - self.scale_in_duration).max(Duration::ZERO);
            let remain_elapsed = elapsed - self.scale_in_duration;
            let t = if remain_duration > Duration::ZERO {
                (remain_elapsed.as_secs_f32() / remain_duration.as_secs_f32()).clamp(0.0, 1.0)
            } else {
                1.0
            };
            self.size_multiplier = 1.0 + (self.final_scale_multiplier - 1.0) * t;
        }

        let rotation_speed_factor = self.speed_factor(now);
        self.rotation_rad = (self.rotation_rad
            + self.rotation_speed_rad_per_sec * _dt.as_secs_f32() * rotation_speed_factor)
            .rem_euclid(std::f32::consts::TAU);
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.alpha <= 0.0 || self.size_multiplier <= 0.0 {
            return sprites;
        }

        let px_xy = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);
        let size_tile = self.base_size_tile * self.size_multiplier;
        let scale = (TILE_PX_SIZE.width.as_f32() * size_tile) / DUST_SPRITE_SIZE_PX;
        let color = Color::WHITE.with_alpha((self.alpha * 255.0) as u8);

        let src_rect = Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: px(DUST_SPRITE_SIZE_PX),
            height: px(DUST_SPRITE_SIZE_PX),
        };

        sprites.push(atlas::centered_rotated_sprite(
            src_rect,
            px_xy.x,
            px_xy.y,
            scale,
            self.rotation_rad,
            Some(color),
        ));

        sprites
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.lifetime
    }
}

impl namui::particle::Particle for DustParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        DustParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        DustParticle::is_done(self, now)
    }
}
