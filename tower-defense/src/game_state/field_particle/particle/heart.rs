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

// === Mushroom Explosion (폭발) ===
const MUSHROOM_EXPLOSION_SPEED_MIN: f32 = 0.3;
const MUSHROOM_EXPLOSION_SPEED_MAX: f32 = 0.6;
const MUSHROOM_EXPLOSION_LIFETIME_MIN_MS: i64 = 300;
const MUSHROOM_EXPLOSION_LIFETIME_MAX_MS: i64 = 800;
const MUSHROOM_EXPLOSION_ALPHA_MIN: f32 = 0.6;
const MUSHROOM_EXPLOSION_ALPHA_MAX: f32 = 0.85;

// === Mushroom Column (기둥) ===
const MUSHROOM_COLUMN_WOBBLE_RANGE: f32 = 0.1; // 좌우 흔들림 범위
const MUSHROOM_COLUMN_LIFETIME_MIN_MS: i64 = 400;
const MUSHROOM_COLUMN_LIFETIME_MAX_MS: i64 = 800;
const MUSHROOM_COLUMN_ALPHA_MIN: f32 = 0.5;
const MUSHROOM_COLUMN_ALPHA_MAX: f32 = 0.7;

// === Rising Heart (상단 하트) ===
const RISING_HEART_LIFETIME_MIN_MS: i64 = 1000;
const RISING_HEART_LIFETIME_MAX_MS: i64 = 1500;
const RISING_HEART_INITIAL_ALPHA: f32 = 0.75;
const RISING_HEART_START_SCALE: f32 = 0.0;
const RISING_HEART_FINAL_SCALE_MIN: f32 = 2.70;
const RISING_HEART_FINAL_SCALE_MAX: f32 = 3.45;
const RISING_HEART_INITIAL_ANGLE_DEG: f32 = 5.0;
const RISING_HEART_MAX_OPACITY: f32 = 0.75;
const RISING_HEART_RISE_DISTANCE_TILE: f32 = 0.6;
const RISING_HEART_ROTATION_DEG_PER_SEC_MAX: f32 = 10.0;

// === Mushroom Sphere (분홍 반투명 구체) ===
const MUSHROOM_EXPLOSION_RADIUS_MIN_TILE: f32 = 0.15;
const MUSHROOM_EXPLOSION_RADIUS_MAX_TILE: f32 = 0.25;
const MUSHROOM_COLUMN_RADIUS_MIN_TILE: f32 = 0.1;
const MUSHROOM_COLUMN_RADIUS_MAX_TILE: f32 = 0.15;
const MUSHROOM_SPHERE_INNER_RADIUS_RATIO: f32 = 0.4; // inner = outer * ratio
// Colors: 분홍색 (RGB)
const MUSHROOM_OUTER_COLOR_RGB: (f32, f32, f32) = (1.0, 0.6, 0.75); // 분홍색
const MUSHROOM_INNER_COLOR_RGB: (f32, f32, f32) = (1.0, 0.8, 0.9); // 밝은 분홍색
const MUSHROOM_OUTER_ALPHA_MULT: f32 = 0.6; // 외부는 더 반투명

#[derive(Clone, State)]
pub struct HeartParticle {
    pub xy: (f32, f32),
    pub velocity: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub initial_opacity: f32,
    pub alpha: f32,
    pub scale: f32,
    pub kind: HeartParticleKind,
}

#[derive(Debug, Clone, Copy, PartialEq, State)]
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
            initial_opacity: 1.0,
            alpha: 1.0,
            scale: 1.0,
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
            initial_opacity: 1.0,
            alpha: 1.0,
            scale: 1.0,
            kind,
        }
    }

    /// 버섯구름 폭발 - 착탄 초기 30ms, 저속 반투명
    pub fn new_mushroom_explosion<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE / 2.0..=OFFSET_RANGE / 2.0); // 위아래 움직임 줄임
        let final_xy = (xy.0 + offset_x, xy.1 + offset_y);

        // 모든 방향 저속 (1.0~2.5 tiles/sec)
        let angle = rng.gen_range(0.0..2.0 * PI);
        let speed = rng.gen_range(MUSHROOM_EXPLOSION_SPEED_MIN..=MUSHROOM_EXPLOSION_SPEED_MAX);
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

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            created_at,
            lifetime,
            initial_opacity,
            alpha: initial_opacity,
            scale: 1.0,
            kind: HeartParticleKind::MushroomExplosion { radius_px },
        }
    }

    /// 버섯구름 기둥 - 30~120ms, 아래에서 위로 이동하며 흔들림
    pub fn new_mushroom_column<R: Rng + ?Sized>(
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        // 수직 1타일 범위(기둥 구간) 내 랜덤 위치에서 생성
        let wobble_x = rng.gen_range(-MUSHROOM_COLUMN_WOBBLE_RANGE..=MUSHROOM_COLUMN_WOBBLE_RANGE);
        let min_y = start_xy.1.min(end_xy.1);
        let max_y = start_xy.1.max(end_xy.1);
        let random_y = rng.gen_range(min_y..=max_y);
        let final_xy = (start_xy.0 + wobble_x, random_y);

        // 주요 이동: 위쪽 방향 (y 감소)
        let vertical_distance = end_xy.1 - start_xy.1; // 음수값 (위쪽)
        // 수명 동안 거리만큼 이동하도록 속도 계산
        let lifetime_ms =
            rng.gen_range(MUSHROOM_COLUMN_LIFETIME_MIN_MS..=MUSHROOM_COLUMN_LIFETIME_MAX_MS) as f32
                / 1000.0;
        let target_speed = vertical_distance.abs() / lifetime_ms;

        let velocity_x = rng.gen_range(-0.2..=0.2); // 약한 좌우 표류
        let velocity_y = -target_speed;

        let radius_tile =
            rng.gen_range(MUSHROOM_COLUMN_RADIUS_MIN_TILE..=MUSHROOM_COLUMN_RADIUS_MAX_TILE);
        let radius_px = TILE_PX_SIZE.width * radius_tile;

        let initial_opacity = rng.gen_range(MUSHROOM_COLUMN_ALPHA_MIN..=MUSHROOM_COLUMN_ALPHA_MAX);

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            created_at,
            lifetime: Duration::from_millis(
                rng.gen_range(MUSHROOM_COLUMN_LIFETIME_MIN_MS..=MUSHROOM_COLUMN_LIFETIME_MAX_MS),
            ),
            initial_opacity,
            alpha: initial_opacity,
            scale: 1.0,
            kind: HeartParticleKind::MushroomColumn { radius_px },
        }
    }

    /// 버섯구름 상단 하트 - ease-out 상승/스케일/투명도
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

        // 초기 회전: 정방향(위쪽) 기준 좌우 5도 랜덤
        let angle_offset_deg =
            rng.gen_range(-RISING_HEART_INITIAL_ANGLE_DEG..=RISING_HEART_INITIAL_ANGLE_DEG);
        let angle_offset_rad = angle_offset_deg * PI / 180.0;
        // ease-out 속도 곡선(1 - ease_out_cubic)의 적분값이 0.25이므로 이를 반영해 기본 속도 계산
        let speed = RISING_HEART_RISE_DISTANCE_TILE * 4.0 / lifetime_secs;

        let velocity_x = angle_offset_rad.sin() * speed;
        let velocity_y = -angle_offset_rad.cos() * speed;

        let final_scale =
            rng.gen_range(RISING_HEART_FINAL_SCALE_MIN..=RISING_HEART_FINAL_SCALE_MAX);

        // 새로 생성될수록 옅어지게 (emitter의 spawn_index 증가를 이용)
        let spawn_alpha_mul = (1.0 / (1.0 + spawn_index * 0.08)).clamp(0.35, 1.0);
        let initial_opacity =
            (RISING_HEART_INITIAL_ALPHA * spawn_alpha_mul).min(RISING_HEART_MAX_OPACITY);

        // 회전 속도: 좌/우 초당 10도 내 랜덤
        let rotation_deg_per_sec = rng.gen_range(
            -RISING_HEART_ROTATION_DEG_PER_SEC_MAX..=RISING_HEART_ROTATION_DEG_PER_SEC_MAX,
        );
        let rotation_rad_per_sec = rotation_deg_per_sec * PI / 180.0;

        Self {
            xy: start_xy,
            velocity: (velocity_x, velocity_y),
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

    pub fn tick(&mut self, now: Instant, dt: Duration) {
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
            // 상승 속도 ease-out: 초반 빠르고 후반으로 갈수록 천천히
            let ease_out_progress = ease_out_cubic(progress);
            movement_speed_mul = (1.0 - ease_out_progress).clamp(0.0, 1.0);

            // opacity도 ease-out 곡선, 최대 0.5
            self.alpha = (self.initial_opacity * (1.0 - ease_out_progress))
                .clamp(0.0, RISING_HEART_MAX_OPACITY);

            // scale도 ease-out 곡선
            self.scale = RISING_HEART_START_SCALE
                + (final_scale - RISING_HEART_START_SCALE) * ease_out_progress;

            // 회전 속도는 개체별로 고정된 랜덤 값(좌/우 초당 10도 내)
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

            // 기존 분홍 구체용: 기존 알파 감쇠 유지
            let alpha_progress = if progress < 0.5 {
                0.95 - progress * 0.1
            } else {
                (1.0 - progress) * 1.8
            };
            self.alpha = (self.initial_opacity * alpha_progress).clamp(0.0, 1.0);
        }

        // Velocity 적용
        self.xy.0 += self.velocity.0 * dt_secs * movement_speed_mul;
        self.xy.1 += self.velocity.1 * dt_secs * movement_speed_mul;
    }

    pub fn is_done(&self, now: Instant) -> bool {
        (now - self.created_at) > self.lifetime
    }

    pub fn render(&self) -> RenderingTree {
        let px_xy_tiles = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);
        let px_xy = Xy::new(px_xy_tiles.x.as_f32(), px_xy_tiles.y.as_f32());

        match self.kind {
            // 분홍 반투명 구체로 렌더링
            HeartParticleKind::MushroomExplosion { radius_px } => {
                self.render_mushroom_sphere(px_xy, radius_px.as_f32())
            }
            HeartParticleKind::MushroomColumn { radius_px } => {
                self.render_mushroom_sphere(px_xy, radius_px.as_f32())
            }
            // 하트 이미지로 렌더링 (기존 방식)
            HeartParticleKind::Heart00
            | HeartParticleKind::Heart01
            | HeartParticleKind::Heart02
            | HeartParticleKind::RisingHeart { .. } => self.render_heart_image(px_xy),
        }
    }

    fn render_heart_image(&self, px_xy: Xy<f32>) -> RenderingTree {
        let heart_size_tile = HEART_SIZE_TILE * self.scale;
        let heart_size_px = TILE_PX_SIZE.width * heart_size_tile;
        let wh = Wh::new(heart_size_px, heart_size_px);

        let image = match self.kind {
            HeartParticleKind::Heart00 => crate::asset::image::attack::particle::HEART_00,
            HeartParticleKind::Heart01 => crate::asset::image::attack::particle::HEART_01,
            HeartParticleKind::Heart02 => crate::asset::image::attack::particle::HEART_02,
            HeartParticleKind::RisingHeart { .. } => {
                crate::asset::image::attack::projectile::HEART_00
            }
            _ => crate::asset::image::attack::particle::HEART_00,
        };

        let paint = Paint::new(Color::WHITE.with_alpha((self.alpha * 255.0) as u8));

        namui::translate(
            px(px_xy.x),
            px(px_xy.y),
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

    fn render_mushroom_sphere(&self, xy_px: Xy<f32>, radius_px: f32) -> RenderingTree {
        if self.alpha <= 0.0 {
            return RenderingTree::Empty;
        }

        let outer_val = radius_px.max(1.0);
        let inner_val = (radius_px * MUSHROOM_SPHERE_INNER_RADIUS_RATIO).max(0.5);

        let outer_path = Path::new().add_oval(Rect::Ltrb {
            left: px(xy_px.x - outer_val),
            top: px(xy_px.y - outer_val),
            right: px(xy_px.x + outer_val),
            bottom: px(xy_px.y + outer_val),
        });
        let inner_path = Path::new().add_oval(Rect::Ltrb {
            left: px(xy_px.x - inner_val),
            top: px(xy_px.y - inner_val),
            right: px(xy_px.x + inner_val),
            bottom: px(xy_px.y + inner_val),
        });

        let (or_r, or_g, or_b) = MUSHROOM_OUTER_COLOR_RGB;
        let (ir_r, ir_g, ir_b) = MUSHROOM_INNER_COLOR_RGB;
        let outer_color = Color::from_f01(or_r, or_g, or_b, self.alpha * MUSHROOM_OUTER_ALPHA_MULT);
        let inner_color = Color::from_f01(ir_r, ir_g, ir_b, self.alpha);

        let outer_paint = Paint::new(outer_color).set_style(PaintStyle::Fill);
        let inner_paint = Paint::new(inner_color)
            .set_style(PaintStyle::Fill)
            .set_blend_mode(BlendMode::Screen);

        namui::render([
            namui::path(outer_path, outer_paint),
            namui::path(inner_path, inner_paint),
        ])
    }
}
