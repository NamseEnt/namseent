use crate::game_state::{attack, TILE_PX_SIZE};
use namui::*;

#[derive(Clone, State)]
pub struct LaserBeamParticle {
    pub start_xy: (f32, f32),
    pub end_xy: (f32, f32),
    pub created_at: Instant,
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
