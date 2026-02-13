use crate::game_state::TILE_PX_SIZE;
use namui::*;
use rand::Rng;
use std::f32::consts::PI;

const HEART_LIFETIME_MIN_MS: i64 = 200;
const HEART_LIFETIME_MAX_MS: i64 = 400;
const HEART_SIZE_TILE: f32 = 0.3;
const OFFSET_RANGE: f32 = 0.15;

// Burst 설정
const BURST_SPEED_MAX: f32 = 5.0; // tiles/sec

// Trail 설정
const TRAIL_SPEED_MIN: f32 = 2.0; // tiles/sec
const TRAIL_SPEED_MAX: f32 = 8.0; // tiles/sec
const TRAIL_ANGLE_RANGE_DEG: f32 = 22.5; // 반대방향 ±22.5도

#[derive(Clone, State)]
pub struct HeartParticle {
    pub xy: (f32, f32),
    pub velocity: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub alpha: f32,
    pub kind: HeartParticleKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub enum HeartParticleKind {
    Heart00,
    Heart01,
    Heart02,
}

impl HeartParticle {
    /// Burst 이펙트용 - 모든 방향으로 0-5 tiles/sec 속도
    pub fn new_burst<R: Rng + ?Sized>(xy: (f32, f32), created_at: Instant, rng: &mut R) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let final_xy = (xy.0 + offset_x, xy.1 + offset_y);

        // 모든 방향 랜덤 속도 (0-5 tiles/sec)
        let angle = rng.gen_range(0.0..2.0 * PI);
        let speed = rng.gen_range(0.0..BURST_SPEED_MAX);
        let velocity_x = angle.cos() * speed;
        let velocity_y = angle.sin() * speed;

        let lifetime_ms = rng.gen_range(HEART_LIFETIME_MIN_MS..=HEART_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let kind = match rng.gen_range(0..3) {
            0 => HeartParticleKind::Heart00,
            1 => HeartParticleKind::Heart01,
            _ => HeartParticleKind::Heart02,
        };

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            created_at,
            lifetime,
            alpha: 1.0,
            kind,
        }
    }

    /// Trail 이펙트용 - 진행방향 반대로 15도 범위 내에서 1-5 tiles/sec 속도
    pub fn new_trail<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        from_direction: (f32, f32), // 진행방향 벡터
        rng: &mut R,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let final_xy = (xy.0 + offset_x, xy.1 + offset_y);

        // 진행방향의 각도 계산
        let direction_angle = from_direction.1.atan2(from_direction.0);

        // 반대방향 (180도)
        let opposite_angle = direction_angle + PI;

        // 반대방향 주변 ±15도 범위 내에서 랜덤
        let angle_range_rad = TRAIL_ANGLE_RANGE_DEG * PI / 180.0;
        let angle_offset = rng.gen_range(-angle_range_rad..=angle_range_rad);
        let final_angle = opposite_angle + angle_offset;

        // 1-5 tiles/sec 속도
        let speed = rng.gen_range(TRAIL_SPEED_MIN..=TRAIL_SPEED_MAX);
        let velocity_x = final_angle.cos() * speed;
        let velocity_y = final_angle.sin() * speed;

        let lifetime_ms = rng.gen_range(HEART_LIFETIME_MIN_MS..=HEART_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let kind = match rng.gen_range(0..3) {
            0 => HeartParticleKind::Heart00,
            1 => HeartParticleKind::Heart01,
            _ => HeartParticleKind::Heart02,
        };

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            created_at,
            lifetime,
            alpha: 1.0,
            kind,
        }
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) {
        let elapsed = (now - self.created_at).as_secs_f32();
        let lifetime = self.lifetime.as_secs_f32();
        let progress = (elapsed / lifetime).clamp(0.0, 1.0);

        // 페이드 아웃
        self.alpha = (1.0 - progress).max(0.0);

        // Velocity 적용
        let dt_secs = dt.as_secs_f32();
        self.xy.0 += self.velocity.0 * dt_secs;
        self.xy.1 += self.velocity.1 * dt_secs;
    }

    pub fn is_done(&self, now: Instant) -> bool {
        (now - self.created_at) > self.lifetime
    }

    pub fn render(&self) -> RenderingTree {
        let heart_size_px = TILE_PX_SIZE.width * HEART_SIZE_TILE;
        let wh = Wh::new(heart_size_px, heart_size_px);

        let px_xy = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);

        let image = match self.kind {
            HeartParticleKind::Heart00 => crate::asset::image::attack::particle::HEART_00,
            HeartParticleKind::Heart01 => crate::asset::image::attack::particle::HEART_01,
            HeartParticleKind::Heart02 => crate::asset::image::attack::particle::HEART_02,
        };

        let paint = Paint::new(Color::WHITE.with_alpha((self.alpha * 255.0) as u8));

        namui::translate(
            px_xy.x,
            px_xy.y,
            namui::image(ImageParam {
                rect: Rect::from_xy_wh(wh.to_xy() * -0.5, wh),
                image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: Some(paint),
                },
            }),
        )
    }
}
