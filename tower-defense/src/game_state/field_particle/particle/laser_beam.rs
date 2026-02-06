use crate::game_state::{TILE_PX_SIZE, attack};
use namui::*;

// 푸른색 상수
const OUTER_COLOR_RGB: (f32, f32, f32) = (0.2, 0.5, 1.0); // 푸른 외곽
const INNER_COLOR_RGB: (f32, f32, f32) = (0.6, 0.85, 1.0); // 밝은 푸른 내부/흰색 기조

#[derive(Clone, State)]
pub struct LaserBeamParticle {
    pub start_xy: (f32, f32),
    pub end_xy: (f32, f32),
    pub created_at: Instant,
    pub alpha: f32,
}

#[derive(Clone, State)]
pub struct LaserLineParticle {
    pub start_xy: (f32, f32),  // 현재 시작점 (이동됨)
    pub end_xy: (f32, f32),    // 현재 끝점 (이동됨)
    pub target_xy: (f32, f32), // 최종 target (clamp용)
    pub created_at: Instant,
    pub duration: Duration,
    pub thickness: f32,      // 두께 (타일)
    pub movement_speed: f32, // 초당 이동 속도 (타일)
    pub alpha: f32,
}

impl LaserBeamParticle {
    pub fn new(start_xy: (f32, f32), end_xy: (f32, f32), created_at: Instant) -> Self {
        Self {
            start_xy,
            end_xy,
            created_at,
            alpha: 1.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.alpha = self.current_alpha(now);
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 {
            return RenderingTree::Empty;
        }

        let start_px = TILE_PX_SIZE.to_xy() * Xy::new(self.start_xy.0, self.start_xy.1);
        let end_px = TILE_PX_SIZE.to_xy() * Xy::new(self.end_xy.0, self.end_xy.1);

        let color = Color::from_f01(1.0, 0.2, 0.2, self.alpha);

        let mut path = Path::new();
        path = path.move_to(start_px.x, start_px.y);
        path = path.line_to(end_px.x, end_px.y);

        let paint = Paint::new(color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(px(8.0 * self.alpha))
            .set_stroke_cap(StrokeCap::Round);

        let mut inner_path = Path::new();
        inner_path = inner_path.move_to(start_px.x, start_px.y);
        inner_path = inner_path.line_to(end_px.x, end_px.y);

        let inner_alpha = self.alpha * 0.8;
        let inner_paint = Paint::new(Color::WHITE.with_alpha((inner_alpha * 255.0) as u8))
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(px(3.0 * self.alpha))
            .set_stroke_cap(StrokeCap::Round);

        namui::render([
            namui::path(path, paint),
            namui::path(inner_path, inner_paint),
        ])
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= attack::laser::LASER_LIFETIME
    }

    fn current_alpha(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        if elapsed >= attack::laser::LASER_LIFETIME {
            return 0.0;
        }

        let progress = elapsed.as_secs_f32() / attack::laser::LASER_LIFETIME.as_secs_f32();
        1.0 - progress
    }
}

impl LaserLineParticle {
    pub fn new(
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        target_xy: (f32, f32),
        created_at: Instant,
        duration: Duration,
        thickness: f32,
        movement_speed: f32,
    ) -> Self {
        Self {
            start_xy,
            end_xy,
            target_xy,
            created_at,
            duration,
            thickness,
            movement_speed,
            alpha: 1.0,
        }
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) {
        // 1. alpha 업데이트 (ease out: 급격하게 나타나고 천천히 사라짐)
        self.alpha = self.current_alpha(now);

        // 2. target 방향으로 아주 조금 이동
        let dt_secs = dt.as_secs_f32();
        let move_distance = self.movement_speed * dt_secs;

        // 방향 벡터 (start -> target)
        let dx = self.target_xy.0 - self.start_xy.0;
        let dy = self.target_xy.1 - self.start_xy.1;
        let dist = (dx * dx + dy * dy).sqrt();

        if dist > 0.001 {
            let dir_x = dx / dist;
            let dir_y = dy / dist;

            // start_xy 이동 (target을 넘지 않도록 clamp)
            self.start_xy.0 += dir_x * move_distance;
            self.start_xy.1 += dir_y * move_distance;

            // end_xy 이동 (target을 넘지 않도록 clamp)
            self.end_xy.0 += dir_x * move_distance;
            self.end_xy.1 += dir_y * move_distance;

            // Clamp: start나 end가 target을 넘어서면 target으로 clamp
            // start가 target 보다 더 가면 target으로 고정
            let new_dist_start = {
                let sx = self.target_xy.0 - self.start_xy.0;
                let sy = self.target_xy.1 - self.start_xy.1;
                sx * dir_x + sy * dir_y // dot product로 방향 체크
            };
            if new_dist_start < 0.0 {
                self.start_xy = self.target_xy;
            }

            let new_dist_end = {
                let ex = self.target_xy.0 - self.end_xy.0;
                let ey = self.target_xy.1 - self.end_xy.1;
                ex * dir_x + ey * dir_y
            };
            if new_dist_end < 0.0 {
                self.end_xy = self.target_xy;
            }
        }
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 {
            return RenderingTree::Empty;
        }

        let start_px = TILE_PX_SIZE.to_xy() * Xy::new(self.start_xy.0, self.start_xy.1);
        let end_px = TILE_PX_SIZE.to_xy() * Xy::new(self.end_xy.0, self.end_xy.1);

        // 외곽 직선 (푸른색, Screen blend mode)
        let outer_color = Color::from_f01(
            OUTER_COLOR_RGB.0,
            OUTER_COLOR_RGB.1,
            OUTER_COLOR_RGB.2,
            self.alpha,
        );

        let mut outer_path = Path::new();
        outer_path = outer_path.move_to(start_px.x, start_px.y);
        outer_path = outer_path.line_to(end_px.x, end_px.y);

        let outer_paint = Paint::new(outer_color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(TILE_PX_SIZE.width * self.thickness)
            .set_stroke_cap(StrokeCap::Round)
            .set_blend_mode(BlendMode::Screen);

        // 내부 직선 (더 밝은 색, 더 얇게)
        let inner_alpha = self.alpha * 0.8;
        let inner_color = Color::from_f01(
            INNER_COLOR_RGB.0,
            INNER_COLOR_RGB.1,
            INNER_COLOR_RGB.2,
            inner_alpha,
        );

        let mut inner_path = Path::new();
        inner_path = inner_path.move_to(start_px.x, start_px.y);
        inner_path = inner_path.line_to(end_px.x, end_px.y);

        let inner_paint = Paint::new(inner_color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(TILE_PX_SIZE.width * self.thickness * 0.4)
            .set_stroke_cap(StrokeCap::Round)
            .set_blend_mode(BlendMode::Screen);

        namui::render([
            namui::path(outer_path, outer_paint),
            namui::path(inner_path, inner_paint),
        ])
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }

    /// Ease out alpha: 급격하게 나타나고 (0->1 빠르게), 천천히 희미해짐 (1->0 느리게)
    fn current_alpha(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        if elapsed >= self.duration {
            return 0.0;
        }

        let progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();

        // Ease out: 처음에 빠르게 나타나고, 나중에 천천히 사라짐
        // - appear phase (0~10%): 급격히 1.0에 도달
        // - fade phase (10~100%): 천천히 0으로 감소
        if progress < 0.1 {
            // 급격히 나타남 (ease out quad)
            let appear_progress = progress / 0.1;
            let inv = 1.0 - appear_progress;
            1.0 - (inv * inv)
        } else {
            // 천천히 사라짐 (linear하게 1 -> 0)
            let fade_progress = (progress - 0.1) / 0.9;
            1.0 - fade_progress
        }
    }
}
