use crate::game_state::TILE_PX_SIZE;
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;
use std::f32::consts::PI;

const HEART_LIFETIME_MIN_MS: i64 = 200;
const HEART_LIFETIME_MAX_MS: i64 = 400;
const HEART_SIZE_TILE: f32 = 0.3;
const OFFSET_RANGE: f32 = 0.15;

const TRAIL_SPEED_MIN: f32 = 2.0;
const TRAIL_SPEED_MAX: f32 = 8.0;
const TRAIL_ANGLE_RANGE_DEG: f32 = 22.5;

const MUSHROOM_EXPLOSION_SPEED_MIN: f32 = 0.3;
const MUSHROOM_EXPLOSION_SPEED_MAX: f32 = 0.6;
const MUSHROOM_EXPLOSION_SPEED_MULT_MIN: f32 = 1.0;
const MUSHROOM_EXPLOSION_SPEED_MULT_MAX: f32 = 2.5;
const MUSHROOM_EXPLOSION_LIFETIME_MIN_MS: i64 = 300;
const MUSHROOM_EXPLOSION_LIFETIME_MAX_MS: i64 = 800;
const MUSHROOM_EXPLOSION_ALPHA_MIN: f32 = 0.2;
const MUSHROOM_EXPLOSION_ALPHA_MAX: f32 = 0.4;

const MUSHROOM_COLUMN_WOBBLE_RANGE: f32 = 0.2;
const MUSHROOM_COLUMN_SPEED_MULT_MIN: f32 = 1.0;
const MUSHROOM_COLUMN_SPEED_MULT_MAX: f32 = 3.5;
const MUSHROOM_COLUMN_LIFETIME_MIN_MS: i64 = 400;
const MUSHROOM_COLUMN_LIFETIME_MAX_MS: i64 = 800;
const MUSHROOM_COLUMN_ALPHA_MIN: f32 = 0.2;
const MUSHROOM_COLUMN_ALPHA_MAX: f32 = 0.4;

const RISING_HEART_LIFETIME_MIN_MS: i64 = 1000;
const RISING_HEART_LIFETIME_MAX_MS: i64 = 1500;
const RISING_HEART_INITIAL_ALPHA: f32 = 0.7;
const RISING_HEART_START_SCALE: f32 = 0.0;
const RISING_HEART_FINAL_SCALE_MIN: f32 = 2.0;
const RISING_HEART_FINAL_SCALE_MAX: f32 = 3.0;
const RISING_HEART_INITIAL_ANGLE_DEG: f32 = 5.0;
const RISING_HEART_MAX_OPACITY: f32 = 0.75;
const RISING_HEART_RISE_DISTANCE_TILE: f32 = 0.6;
const RISING_HEART_ROTATION_DEG_PER_SEC_MAX: f32 = 10.0;

const MUSHROOM_EXPLOSION_RADIUS_MIN_TILE: f32 = 0.2;
const MUSHROOM_EXPLOSION_RADIUS_MAX_TILE: f32 = 0.5;
const MUSHROOM_COLUMN_RADIUS_MIN_TILE: f32 = 0.15;
const MUSHROOM_COLUMN_RADIUS_MAX_TILE: f32 = 0.3;
const MUSHROOM_OUTER_ALPHA_MULT: f32 = 0.2;
const MUSHROOM_SMOKE_ROTATION_DEG_PER_SEC_MIN: f32 = -8.0;
const MUSHROOM_SMOKE_ROTATION_DEG_PER_SEC_MAX: f32 = 8.0;

#[derive(Clone)]
pub struct HeartParticle {
    pub xy: (f32, f32),
    pub velocity: (f32, f32),
    pub smoke_rotation_rad: f32,
    pub smoke_rotation_rad_per_sec: f32,
    pub created_at: Instant,
    pub lifetime: Duration,
    pub initial_opacity: f32,
    pub alpha: f32,
    pub scale: f32,
    pub kind: HeartParticleKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HeartParticleKind {
    Heart00,
    Heart01,
    Heart02,
    MushroomExplosion {
        radius_px: Px,
    },
    MushroomColumn {
        radius_px: Px,
    },
    RisingHeart {
        final_scale: f32,
        rotation_rad_per_sec: f32,
    },
}

#[inline]
fn ease_out_cubic(t: f32) -> f32 {
    1.0 - (1.0 - t).powi(3)
}

#[inline]
fn random_heart_kind<R: Rng + ?Sized>(rng: &mut R) -> HeartParticleKind {
    match rng.gen_range(0..3) {
        0 => HeartParticleKind::Heart00,
        1 => HeartParticleKind::Heart01,
        _ => HeartParticleKind::Heart02,
    }
}

impl HeartParticle {
    pub fn new_trail<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        from_direction: (f32, f32),
        rng: &mut R,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let final_xy = (xy.0 + offset_x, xy.1 + offset_y);

        let direction_angle = from_direction.1.atan2(from_direction.0);
        let opposite_angle = direction_angle + PI;
        let angle_range_rad = TRAIL_ANGLE_RANGE_DEG * PI / 180.0;
        let angle_offset = rng.gen_range(-angle_range_rad..=angle_range_rad);
        let final_angle = opposite_angle + angle_offset;

        let speed = rng.gen_range(TRAIL_SPEED_MIN..=TRAIL_SPEED_MAX);
        let velocity_x = final_angle.cos() * speed;
        let velocity_y = final_angle.sin() * speed;

        let lifetime_ms = rng.gen_range(HEART_LIFETIME_MIN_MS..=HEART_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            smoke_rotation_rad: 0.0,
            smoke_rotation_rad_per_sec: 0.0,
            created_at,
            lifetime,
            initial_opacity: 1.0,
            alpha: 1.0,
            scale: 1.0,
            kind: random_heart_kind(rng),
        }
    }

    pub fn new_mushroom_explosion<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE / 2.0..=OFFSET_RANGE / 2.0);
        let final_xy = (xy.0 + offset_x, xy.1 + offset_y);

        let angle = rng.gen_range(0.0..2.0 * PI);
        let base_speed = rng.gen_range(MUSHROOM_EXPLOSION_SPEED_MIN..=MUSHROOM_EXPLOSION_SPEED_MAX);
        let speed_mult =
            rng.gen_range(MUSHROOM_EXPLOSION_SPEED_MULT_MIN..=MUSHROOM_EXPLOSION_SPEED_MULT_MAX);
        let speed = base_speed * speed_mult;
        let velocity_x = angle.cos() * speed;
        let velocity_y = angle.sin() * speed;

        let lifetime_ms =
            rng.gen_range(MUSHROOM_EXPLOSION_LIFETIME_MIN_MS..=MUSHROOM_EXPLOSION_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let radius_tile =
            rng.gen_range(MUSHROOM_EXPLOSION_RADIUS_MIN_TILE..=MUSHROOM_EXPLOSION_RADIUS_MAX_TILE);
        let radius_px = TILE_PX_SIZE.width * radius_tile;

        let initial_opacity =
            rng.gen_range(MUSHROOM_EXPLOSION_ALPHA_MIN..=MUSHROOM_EXPLOSION_ALPHA_MAX);
        let smoke_rotation_deg_per_sec = rng.gen_range(
            MUSHROOM_SMOKE_ROTATION_DEG_PER_SEC_MIN..=MUSHROOM_SMOKE_ROTATION_DEG_PER_SEC_MAX,
        );
        let smoke_rotation_rad_per_sec = smoke_rotation_deg_per_sec * PI / 180.0;

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            smoke_rotation_rad: rng.gen_range(0.0..2.0 * PI),
            smoke_rotation_rad_per_sec,
            created_at,
            lifetime,
            initial_opacity,
            alpha: initial_opacity,
            scale: 1.0,
            kind: HeartParticleKind::MushroomExplosion { radius_px },
        }
    }

    pub fn new_mushroom_column<R: Rng + ?Sized>(
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let wobble_x = rng.gen_range(-MUSHROOM_COLUMN_WOBBLE_RANGE..=MUSHROOM_COLUMN_WOBBLE_RANGE);
        let min_y = start_xy.1.min(end_xy.1);
        let max_y = start_xy.1.max(end_xy.1);
        let random_y = rng.gen_range(min_y..=max_y);
        let final_xy = (start_xy.0 + wobble_x, random_y);

        let vertical_distance = end_xy.1 - start_xy.1;
        let lifetime_ms =
            rng.gen_range(MUSHROOM_COLUMN_LIFETIME_MIN_MS..=MUSHROOM_COLUMN_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);
        let base_target_speed = vertical_distance.abs() / lifetime.as_secs_f32();
        let speed_mult =
            rng.gen_range(MUSHROOM_COLUMN_SPEED_MULT_MIN..=MUSHROOM_COLUMN_SPEED_MULT_MAX);
        let target_speed = base_target_speed * speed_mult;

        let velocity_x = rng.gen_range(-0.2..=0.2);
        let velocity_y = -target_speed;

        let radius_tile =
            rng.gen_range(MUSHROOM_COLUMN_RADIUS_MIN_TILE..=MUSHROOM_COLUMN_RADIUS_MAX_TILE);
        let radius_px = TILE_PX_SIZE.width * radius_tile;

        let initial_opacity = rng.gen_range(MUSHROOM_COLUMN_ALPHA_MIN..=MUSHROOM_COLUMN_ALPHA_MAX);
        let smoke_rotation_deg_per_sec = rng.gen_range(
            MUSHROOM_SMOKE_ROTATION_DEG_PER_SEC_MIN..=MUSHROOM_SMOKE_ROTATION_DEG_PER_SEC_MAX,
        );
        let smoke_rotation_rad_per_sec = smoke_rotation_deg_per_sec * PI / 180.0;

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            smoke_rotation_rad: rng.gen_range(0.0..2.0 * PI),
            smoke_rotation_rad_per_sec,
            created_at,
            lifetime,
            initial_opacity,
            alpha: initial_opacity,
            scale: 1.0,
            kind: HeartParticleKind::MushroomColumn { radius_px },
        }
    }

    pub fn new_rising_heart<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        spawn_index: f32,
        rng: &mut R,
    ) -> Self {
        let offset_y = rng.gen_range(-0.08..=0.02);
        let start_xy = (xy.0, xy.1 + offset_y);

        let lifetime_ms =
            rng.gen_range(RISING_HEART_LIFETIME_MIN_MS..=RISING_HEART_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);
        let lifetime_secs = lifetime.as_secs_f32();

        let angle_offset_deg =
            rng.gen_range(-RISING_HEART_INITIAL_ANGLE_DEG..=RISING_HEART_INITIAL_ANGLE_DEG);
        let angle_offset_rad = angle_offset_deg * PI / 180.0;
        let speed = RISING_HEART_RISE_DISTANCE_TILE * 4.0 / lifetime_secs;

        let velocity_x = angle_offset_rad.sin() * speed;
        let velocity_y = -angle_offset_rad.cos() * speed;

        let final_scale =
            rng.gen_range(RISING_HEART_FINAL_SCALE_MIN..=RISING_HEART_FINAL_SCALE_MAX);

        let spawn_alpha_mul = (1.0 / (1.0 + spawn_index * 0.08)).clamp(0.35, 1.0);
        let initial_opacity =
            (RISING_HEART_INITIAL_ALPHA * spawn_alpha_mul).min(RISING_HEART_MAX_OPACITY);

        let rotation_deg_per_sec = rng.gen_range(
            -RISING_HEART_ROTATION_DEG_PER_SEC_MAX..=RISING_HEART_ROTATION_DEG_PER_SEC_MAX,
        );
        let rotation_rad_per_sec = rotation_deg_per_sec * PI / 180.0;

        Self {
            xy: start_xy,
            velocity: (velocity_x, velocity_y),
            smoke_rotation_rad: 0.0,
            smoke_rotation_rad_per_sec: 0.0,
            created_at,
            lifetime,
            initial_opacity,
            alpha: initial_opacity,
            scale: RISING_HEART_START_SCALE,
            kind: HeartParticleKind::RisingHeart {
                final_scale,
                rotation_rad_per_sec,
            },
        }
    }

    pub fn tick_impl(&mut self, now: Instant, dt: Duration) {
        let elapsed = (now - self.created_at).as_secs_f32();
        let lifetime = self.lifetime.as_secs_f32();
        let progress = (elapsed / lifetime).clamp(0.0, 1.0);
        let dt_secs = dt.as_secs_f32();
        let mut movement_speed_mul = 1.0;

        if let HeartParticleKind::RisingHeart {
            final_scale,
            rotation_rad_per_sec,
        } = self.kind
        {
            let ease_out_progress = ease_out_cubic(progress);
            movement_speed_mul = (1.0 - ease_out_progress).clamp(0.0, 1.0);

            self.alpha = (self.initial_opacity * (1.0 - ease_out_progress))
                .clamp(0.0, RISING_HEART_MAX_OPACITY);

            self.scale = RISING_HEART_START_SCALE
                + (final_scale - RISING_HEART_START_SCALE) * ease_out_progress;

            let dtheta = rotation_rad_per_sec * dt_secs;
            let (vx, vy) = self.velocity;
            let cos_t = dtheta.cos();
            let sin_t = dtheta.sin();
            self.velocity = (vx * cos_t - vy * sin_t, vx * sin_t + vy * cos_t);
        } else {
            if matches!(
                self.kind,
                HeartParticleKind::MushroomExplosion { .. }
                    | HeartParticleKind::MushroomColumn { .. }
            ) {
                let ease_out_progress = ease_out_cubic(progress);
                movement_speed_mul = (1.0 - ease_out_progress).clamp(0.0, 1.0);
            }

            let alpha_progress = if progress < 0.5 {
                0.95 - progress * 0.1
            } else {
                (1.0 - progress) * 1.8
            };
            self.alpha = (self.initial_opacity * alpha_progress).clamp(0.0, 1.0);
        }

        self.xy.0 += self.velocity.0 * dt_secs * movement_speed_mul;
        self.xy.1 += self.velocity.1 * dt_secs * movement_speed_mul;

        if matches!(
            self.kind,
            HeartParticleKind::MushroomExplosion { .. } | HeartParticleKind::MushroomColumn { .. }
        ) {
            self.smoke_rotation_rad += self.smoke_rotation_rad_per_sec * dt_secs;
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        (now - self.created_at) > self.lifetime
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();

        let px_xy = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);

        match self.kind {
            HeartParticleKind::MushroomExplosion { radius_px }
            | HeartParticleKind::MushroomColumn { radius_px } => {
                if self.alpha <= 0.0 {
                    return sprites;
                }

                let radius = radius_px.as_f32().max(1.0);
                let outer_scale = (radius * 2.0) / 128.0;
                let outer_color =
                    Color::WHITE.with_alpha((self.alpha * MUSHROOM_OUTER_ALPHA_MULT * 255.0) as u8);
                sprites.push(atlas::centered_rotated_sprite(
                    atlas::projectile_pink_smoke(),
                    px_xy.x,
                    px_xy.y,
                    outer_scale,
                    self.smoke_rotation_rad,
                    Some(outer_color),
                ));
            }
            HeartParticleKind::Heart00
            | HeartParticleKind::Heart01
            | HeartParticleKind::Heart02
            | HeartParticleKind::RisingHeart { .. } => {
                let scale = (TILE_PX_SIZE.width.as_f32() * HEART_SIZE_TILE * self.scale) / 128.0;
                let color = Color::WHITE.with_alpha((self.alpha * 255.0) as u8);
                let src_rect = atlas::heart_particle_rect(self.kind);
                sprites.push(atlas::centered_sprite(
                    src_rect,
                    px_xy.x,
                    px_xy.y,
                    scale,
                    Some(color),
                ));
            }
        }

        sprites
    }
}

impl namui::particle::Particle for HeartParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> namui::particle::ParticleSprites {
        HeartParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        HeartParticle::is_done(self, now)
    }
}
