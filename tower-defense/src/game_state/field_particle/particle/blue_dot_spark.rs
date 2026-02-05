use crate::game_state::TILE_PX_SIZE;
use namui::*;
use rand::Rng;

const BLUE_DOT_LIFETIME_MIN_MS: i64 = 150;
const BLUE_DOT_LIFETIME_MAX_MS: i64 = 600;
const BLUE_DOT_RADIUS_MIN_TILE: f32 = 0.03;
const BLUE_DOT_RADIUS_MAX_TILE: f32 = 0.06;
const BLUE_DOT_SPEED_MIN: f32 = 4.0; // 맵 좌표 단위/초
const BLUE_DOT_SPEED_MAX: f32 = 12.0;
const BLUE_DOT_GRAVITY: f32 = 0.3; // 맵 좌표 단위/초^2 (아래로, EmberSpark보다 약함)
const BLUE_DOT_FADE_START: f32 = 0.6; // progress 60%부터 페이드 시작

// Colors (RGB, 0.0..1.0)
const BLUE_DOT_OUTER_COLOR_RGB: (f32, f32, f32) = (0.2, 0.6, 1.0); // 청색
const BLUE_DOT_INNER_COLOR_RGB: (f32, f32, f32) = (0.7, 0.95, 1.0); // 밝은 청색
const BLUE_DOT_INNER_RADIUS_RATIO: f32 = 0.3;

#[derive(Clone, State)]
pub struct BlueDotSparkParticle {
    pub xy: (f32, f32),
    pub velocity: (f32, f32), // 맵 좌표 단위/초
    pub created_at: Instant,
    pub lifetime: Duration,
    pub radius: Px,
    pub alpha: f32,
}

impl BlueDotSparkParticle {
    pub fn new_with_random<R: Rng + ?Sized>(
        xy: (f32, f32),
        movement_direction: (f32, f32), // 진행 방향 벡터 (정규화되어 있어야 함)
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let lifetime_ms = rng.gen_range(BLUE_DOT_LIFETIME_MIN_MS..=BLUE_DOT_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let radius =
            TILE_PX_SIZE.width * rng.gen_range(BLUE_DOT_RADIUS_MIN_TILE..=BLUE_DOT_RADIUS_MAX_TILE);

        let speed = rng.gen_range(BLUE_DOT_SPEED_MIN..=BLUE_DOT_SPEED_MAX);
        let velocity = (movement_direction.0 * speed, movement_direction.1 * speed);

        Self {
            xy,
            velocity,
            created_at,
            lifetime,
            radius,
            alpha: 1.0,
        }
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) {
        let dt_sec = dt.as_secs_f32();

        // 속도 업데이트 (중력 적용)
        self.velocity.1 += BLUE_DOT_GRAVITY * dt_sec;

        // 위치 업데이트
        self.xy.0 += self.velocity.0 * dt_sec;
        self.xy.1 += self.velocity.1 * dt_sec;

        // 알파 업데이트
        let progress = self.progress(now);
        if progress >= BLUE_DOT_FADE_START {
            let fade_progress = (progress - BLUE_DOT_FADE_START) / (1.0 - BLUE_DOT_FADE_START);
            self.alpha = 1.0 - fade_progress;
        } else {
            self.alpha = 1.0;
        }
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 {
            return RenderingTree::Empty;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);
        let outer_radius = self.radius;
        let inner_radius = px(self.radius.as_f32() * BLUE_DOT_INNER_RADIUS_RATIO);

        let outer_path = Path::new().add_oval(Rect::Ltrb {
            left: xy_px.x - outer_radius,
            top: xy_px.y - outer_radius,
            right: xy_px.x + outer_radius,
            bottom: xy_px.y + outer_radius,
        });
        let inner_path = Path::new().add_oval(Rect::Ltrb {
            left: xy_px.x - inner_radius,
            top: xy_px.y - inner_radius,
            right: xy_px.x + inner_radius,
            bottom: xy_px.y + inner_radius,
        });

        let (or_r, or_g, or_b) = BLUE_DOT_OUTER_COLOR_RGB;
        let (ir_r, ir_g, ir_b) = BLUE_DOT_INNER_COLOR_RGB;
        let outer_color = Color::from_f01(or_r, or_g, or_b, self.alpha * 0.6);
        let inner_color = Color::from_f01(ir_r, ir_g, ir_b, self.alpha);

        let outer_paint = Paint::new(outer_color)
            .set_style(PaintStyle::Fill)
            .set_blend_mode(BlendMode::Screen);
        let inner_paint = Paint::new(inner_color)
            .set_style(PaintStyle::Fill)
            .set_blend_mode(BlendMode::Screen);

        namui::render([
            namui::path(outer_path, outer_paint),
            namui::path(inner_path, inner_paint),
        ])
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.lifetime
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / self.lifetime.as_secs_f32()).min(1.0)
    }
}
