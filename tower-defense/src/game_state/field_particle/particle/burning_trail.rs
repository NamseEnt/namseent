use crate::game_state::TILE_PX_SIZE;
use namui::*;

const BURNING_TRAIL_LIFETIME: Duration = Duration::from_millis(220);
const OUTER_RADIUS_PX: f32 = 12.0;

#[derive(Clone, State)]
pub struct BurningTrailParticle {
    pub xy: (f32, f32),
    pub created_at: Instant,
    pub alpha: f32,
    pub radius: f32,
}

impl BurningTrailParticle {
    pub fn new(xy: (f32, f32), created_at: Instant) -> Self {
        Self {
            xy,
            created_at,
            alpha: 1.0,
            radius: OUTER_RADIUS_PX,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let progress = self.progress(now);
        self.alpha = (1.0 - progress).max(0.0);
        self.radius = OUTER_RADIUS_PX * (1.0 - 0.5 * progress);
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 {
            return RenderingTree::Empty;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);
        let outer = self.radius.max(1.0);
        let inner = (self.radius * 0.5).max(0.5);

        let outer_path = Path::new().add_oval(Rect::Ltrb {
            left: xy_px.x - px(outer),
            top: xy_px.y - px(outer),
            right: xy_px.x + px(outer),
            bottom: xy_px.y + px(outer),
        });
        let inner_path = Path::new().add_oval(Rect::Ltrb {
            left: xy_px.x - px(inner),
            top: xy_px.y - px(inner),
            right: xy_px.x + px(inner),
            bottom: xy_px.y + px(inner),
        });

        let outer_color = Color::from_f01(1.0, 0.35, 0.05, self.alpha * 0.7);
        let inner_color = Color::from_f01(1.0, 0.85, 0.2, self.alpha);

        let outer_paint = Paint::new(outer_color).set_style(PaintStyle::Fill);
        let inner_paint = Paint::new(inner_color).set_style(PaintStyle::Fill);

        namui::render([
            namui::path(outer_path, outer_paint),
            namui::path(inner_path, inner_paint),
        ])
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= BURNING_TRAIL_LIFETIME
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / BURNING_TRAIL_LIFETIME.as_secs_f32()).min(1.0)
    }
}
