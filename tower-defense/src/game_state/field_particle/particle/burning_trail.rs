use crate::game_state::TILE_PX_SIZE;
use namui::*;
use rand::Rng;

// Configurable parameters for burning trail particles
const BURNING_TRAIL_LIFETIME_MIN_MS: i64 = 120;
const BURNING_TRAIL_LIFETIME_MAX_MS: i64 = 240;
const OUTER_RADIUS_MIN_TILE: f32 = 0.2;
const OUTER_RADIUS_MAX_TILE: f32 = 0.35;
const INNER_RADIUS_RATIO: f32 = 0.5; // inner radius = outer * ratio
const OFFSET_RANGE: f32 = 0.06; // 맵 좌표 단위

// Colors (RGB, 0.0..1.0)
const OUTER_COLOR_RGB: (f32, f32, f32) = (1.0, 0.35, 0.05);
const INNER_COLOR_RGB: (f32, f32, f32) = (1.0, 0.85, 0.2);
const OUTER_ALPHA_MULT: f32 = 0.7;
// Alpha & radius progression config
const ALPHA_RISE_END_PROGRESS: f32 = 0.2; // progress where alpha stops rising
const ALPHA_MIN: f32 = 0.2;
const ALPHA_PEAK: f32 = 0.9;
const RADIUS_START_RATIO: f32 = 0.6; // relative to initial_radius at progress 0
const RADIUS_PEAK_RATIO: f32 = 1.0; // relative at peak (progress 0.1)
const RADIUS_END_RATIO: f32 = 0.0; // relative at end (progress 1.0)

#[derive(Clone, State)]
pub struct BurningTrailParticle {
    pub xy: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub initial_radius: Px,
    pub alpha: f32,
    pub radius: Px,
}

impl BurningTrailParticle {
    pub fn new_with_random<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let final_xy = (xy.0 + offset_x, xy.1 + offset_y);

        let lifetime_ms =
            rng.gen_range(BURNING_TRAIL_LIFETIME_MIN_MS..=BURNING_TRAIL_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let initial_radius =
            TILE_PX_SIZE.width * rng.gen_range(OUTER_RADIUS_MIN_TILE..=OUTER_RADIUS_MAX_TILE);

        Self {
            xy: final_xy,
            created_at,
            lifetime,
            initial_radius,
            alpha: 1.0,
            radius: initial_radius,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let progress = self.progress(now);

        if progress <= ALPHA_RISE_END_PROGRESS {
            let t = progress / ALPHA_RISE_END_PROGRESS;
            self.alpha = ALPHA_MIN + (ALPHA_PEAK - ALPHA_MIN) * t;
            let radius_ratio = RADIUS_START_RATIO + (RADIUS_PEAK_RATIO - RADIUS_START_RATIO) * t;
            let r = self.initial_radius.as_f32() * radius_ratio;
            self.radius = px(r);
        } else {
            let t = (progress - ALPHA_RISE_END_PROGRESS) / (1.0 - ALPHA_RISE_END_PROGRESS);
            self.alpha = ALPHA_PEAK * (1.0 - t);
            let radius_ratio = RADIUS_PEAK_RATIO + (RADIUS_END_RATIO - RADIUS_PEAK_RATIO) * t;
            let r = self.initial_radius.as_f32() * radius_ratio;
            self.radius = px(r);
        }
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 {
            return RenderingTree::Empty;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);
        let outer_val = self.radius.as_f32().max(1.0);
        let inner_val = (self.radius.as_f32() * INNER_RADIUS_RATIO).max(0.5);
        let outer = px(outer_val);
        let inner = px(inner_val);

        let outer_path = Path::new().add_oval(Rect::Ltrb {
            left: xy_px.x - outer,
            top: xy_px.y - outer,
            right: xy_px.x + outer,
            bottom: xy_px.y + outer,
        });
        let inner_path = Path::new().add_oval(Rect::Ltrb {
            left: xy_px.x - inner,
            top: xy_px.y - inner,
            right: xy_px.x + inner,
            bottom: xy_px.y + inner,
        });

        let (or_r, or_g, or_b) = OUTER_COLOR_RGB;
        let (ir_r, ir_g, ir_b) = INNER_COLOR_RGB;
        let outer_color = Color::from_f01(or_r, or_g, or_b, self.alpha * OUTER_ALPHA_MULT);
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
