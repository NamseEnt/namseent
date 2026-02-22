use crate::{MapCoordF32, game_state::TILE_PX_SIZE};
use crate::game_state::field_particle::atlas;
use namui::*;
use rand::Rng;

// Animation parameters
const PARTICLE_LIFETIME_MS: i64 = 800;

// Movement parameters
const INITIAL_VELOCITY: f32 = 8.0; // tile units per second
const DIRECTION_SPREAD: f32 = 45.0; // degrees left/right from top
const GRAVITY: f32 = 8.0; // tile units per second squared
const AIR_RESISTANCE: f32 = 0.0005; // velocity multiplier per second (0.8 = 20% loss per second)
const POSITION_RANDOMIZATION: f32 = 0.125; // ±tiles from center

// Scale parameters
const SCALE_ACCELERATION_PHASE_PROGRESS: f32 = 0.3;
const SCALE_ACCELERATION_POWER: i32 = 4;
const SCALE_DECELERATION_POWER: i32 = 1;
const MAX_SCALE: f32 = 1.0;
const MIN_SCALE: f32 = 0.0;

// Rotation parameters
const MAX_ROTATION_SPEED: f32 = 25.0; // degrees per second

// Color thresholds
const COLOR_YELLOW_THRESHOLD: f32 = 2000.0;
const COLOR_RED_THRESHOLD: f32 = 10000.0;

#[derive(Clone)]
pub struct DisplayValue {
    buf: [u8; 8],
    len: usize,
}

impl DisplayValue {
    fn new() -> Self {
        Self { buf: [0; 8], len: 0 }
    }
    fn bytes(&self) -> &[u8] {
        &self.buf[..self.len]
    }
    fn len(&self) -> usize {
        self.len
    }
}

impl core::fmt::Write for DisplayValue {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for &b in s.as_bytes() {
            if self.len >= 8 { break; }
            self.buf[self.len] = b;
            self.len += 1;
        }
        Ok(())
    }
}

#[derive(Clone)]
pub struct DamageTextParticle {
    pub position: MapCoordF32,
    pub initial_position: MapCoordF32,
    pub velocity: Xy<f32>,
    pub display_color: Color,
    pub display_value: DisplayValue,
    pub created_at: Instant,
    pub duration: Duration,
    pub opacity: u8,
    pub rotation: Angle,
    pub rotation_speed: f32,
    pub scale: f32,
}

impl DamageTextParticle {
    pub fn new(position: MapCoordF32, damage_value: f32, now: Instant) -> Self {
        let display_color = Self::calculate_display_color(damage_value);
        let display_value = Self::format_display_value(damage_value);

        let mut rng = rand::thread_rng();
        let rotation_speed = rng.gen_range(-MAX_ROTATION_SPEED..=MAX_ROTATION_SPEED); // degrees per second

        // Randomize position within ±POSITION_RANDOMIZATION tiles
        let randomized_position = position
            + MapCoordF32::new(
                rng.gen_range(-POSITION_RANDOMIZATION..=POSITION_RANDOMIZATION),
                rng.gen_range(-POSITION_RANDOMIZATION..=POSITION_RANDOMIZATION),
            );

        // Generate random direction: upward (top is -90 degrees) with ±DIRECTION_SPREAD
        let direction_degrees = -90.0 + rng.gen_range(-DIRECTION_SPREAD..=DIRECTION_SPREAD);
        let direction_angle = direction_degrees.deg();
        let velocity_x = INITIAL_VELOCITY * direction_angle.cos();
        let velocity_y = INITIAL_VELOCITY * direction_angle.sin(); // negative because y goes down

        Self {
            position: randomized_position,
            initial_position: randomized_position,
            velocity: Xy::new(velocity_x, velocity_y),
            display_color,
            display_value,
            created_at: now,
            duration: Duration::from_millis(PARTICLE_LIFETIME_MS),
            opacity: 255,
            rotation: 0.0.deg(),
            rotation_speed,
            scale: 0.0,
        }
    }
    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }
    pub fn tick(&mut self, now: Instant, _delta_time: Duration) {
        let elapsed = now - self.created_at;
        let progress = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        let elapsed_secs = elapsed.as_secs_f32();

        // Position update with physics (velocity + gravity + air resistance)
        self.position = self.calculate_position(elapsed_secs);

        // Opacity: linear fade from 255 to 0
        self.opacity = ((1.0 - progress) * 255.0).round() as u8;

        // Rotation: continuous spin
        self.rotation = (self.rotation_speed * elapsed_secs).deg();

        // Scale: ease-in and ease-out
        self.scale = self.calculate_scale(progress);
    }
    fn calculate_position(&self, elapsed_secs: f32) -> MapCoordF32 {
        let resistance_factor = AIR_RESISTANCE.powf(elapsed_secs);
        let resistance_ln = AIR_RESISTANCE.ln();
        let integration_factor = if resistance_ln.abs() > 0.0001 {
            (resistance_factor - 1.0) / resistance_ln
        } else {
            elapsed_secs
        };

        let displacement_x = self.velocity.x * integration_factor;
        let displacement_y =
            self.velocity.y * integration_factor + 0.5 * GRAVITY * elapsed_secs * elapsed_secs;

        MapCoordF32::new(
            self.initial_position.x + displacement_x,
            self.initial_position.y + displacement_y,
        )
    }

    fn calculate_scale(&self, progress: f32) -> f32 {
        if progress < SCALE_ACCELERATION_PHASE_PROGRESS {
            let t = progress / SCALE_ACCELERATION_PHASE_PROGRESS;
            1.0 - (1.0 - t).powi(SCALE_ACCELERATION_POWER)
        } else {
            let t = (progress - SCALE_ACCELERATION_PHASE_PROGRESS)
                / (1.0 - SCALE_ACCELERATION_PHASE_PROGRESS);
            let ease_out = 1.0 - (1.0 - t).powi(SCALE_DECELERATION_POWER);
            MAX_SCALE - (MAX_SCALE - MIN_SCALE) * ease_out
        }
    }
    fn format_display_value(damage_value: f32) -> DisplayValue {
        let mut buf = DisplayValue::new();
        use core::fmt::Write;
        if damage_value >= 1_000_000_000.0 {
            let _ = write!(buf, "{:.1}b", damage_value / 1_000_000_000.0);
        } else if damage_value >= 1_000_000.0 {
            let _ = write!(buf, "{:.1}m", damage_value / 1_000_000.0);
        } else if damage_value >= 1_000.0 {
            let _ = write!(buf, "{:.1}k", damage_value / 1_000.0);
        } else {
            let _ = write!(buf, "{:.0}", damage_value);
        }
        buf
    }

    pub fn render(&self) -> namui::particle::ParticleSprites {
        let mut sprites = namui::particle::ParticleSprites::new();
        if self.opacity == 0 {
            return sprites;
        }
        let tile_size = TILE_PX_SIZE.to_xy();
        let position_px = tile_size * self.position;
        let base_scale = self.scale * 0.5;
        let color = self.display_color.with_alpha(self.opacity);
        let angle_rad = self.rotation.as_radians();
        let cos_a = angle_rad.cos();
        let sin_a = angle_rad.sin();

        let char_count = self.display_value.len() as f32;
        let char_w = 64.0 * base_scale;
        let total_w = char_count * char_w;
        let start_offset = -total_w / 2.0 + char_w / 2.0;

        for (i, &ch) in self.display_value.bytes().iter().enumerate() {
            let src_rect = atlas::digit_rect(ch);
            let local_x = start_offset + i as f32 * char_w;
            let cx = position_px.x + px(cos_a * local_x);
            let cy = position_px.y + px(sin_a * local_x);
            sprites.push(atlas::centered_rotated_sprite(
                src_rect, cx, cy, base_scale, angle_rad, Some(color),
            ));
            if sprites.remaining_capacity() == 0 {
                break;
            }
        }
        sprites
    }

    fn calculate_display_color(damage_value: f32) -> Color {
        let (r, g, b) = if damage_value < COLOR_YELLOW_THRESHOLD {
            let t = (damage_value / COLOR_YELLOW_THRESHOLD).clamp(0.0, 1.0);
            (
                ((255.0 * t + 255.0 * (1.0 - t)).round()) as u8,
                ((220.0 * t + 255.0 * (1.0 - t)).round()) as u8,
                ((40.0 * t + 255.0 * (1.0 - t)).round()) as u8,
            )
        } else if damage_value < COLOR_RED_THRESHOLD {
            let t = ((damage_value - COLOR_YELLOW_THRESHOLD)
                / (COLOR_RED_THRESHOLD - COLOR_YELLOW_THRESHOLD))
                .clamp(0.0, 1.0);
            (
                255u8,
                ((40.0 * t + 220.0 * (1.0 - t)).round()) as u8,
                ((40.0 * t + 40.0 * (1.0 - t)).round()) as u8,
            )
        } else {
            (255u8, 40u8, 40u8)
        };
        Color::from_u8(r, g, b, 255)
    }
}

impl namui::particle::Particle for DamageTextParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        DamageTextParticle::tick(self, now, dt);
    }
    fn render(&self) -> namui::particle::ParticleSprites {
        DamageTextParticle::render(self)
    }
    fn is_done(&self, now: Instant) -> bool {
        DamageTextParticle::is_done(self, now)
    }
}
