use crate::game_state::TILE_PX_SIZE;
use namui::*;
use rand::Rng;

const WIND_CURVE_LIFETIME_MIN_MS: i64 = 90;
const WIND_CURVE_LIFETIME_MAX_MS: i64 = 170;
const WIND_CURVE_LENGTH_MIN_TILE: f32 = 0.4;
const WIND_CURVE_LENGTH_MAX_TILE: f32 = 0.8;
const WIND_CURVE_AMPLITUDE_MIN_TILE: f32 = 0.1;
const WIND_CURVE_AMPLITUDE_MAX_TILE: f32 = 0.25;
const WIND_CURVE_THICKNESS_MIN_TILE: f32 = 0.02;
const WIND_CURVE_THICKNESS_MAX_TILE: f32 = 0.05;
const OFFSET_RANGE_TILE: f32 = 0.1;

const OUTER_COLOR_RGB: (f32, f32, f32) = (0.60, 0.72, 0.78);
const INNER_COLOR_RGB: (f32, f32, f32) = (0.85, 0.92, 0.95);
const OUTER_ALPHA_MULT: f32 = 0.55;
const FADE_START_PROGRESS: f32 = 0.15;

#[derive(Clone, State)]
pub struct WindCurveTrailParticle {
    pub center_xy: (f32, f32),
    pub movement_direction: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub length_tile: f32,
    pub amplitude_tile: f32,
    pub thickness_tile: f32,
    pub alpha: f32,
}

impl WindCurveTrailParticle {
    pub fn new_with_random<R: Rng + ?Sized>(
        xy: (f32, f32),
        movement_direction: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE_TILE..=OFFSET_RANGE_TILE);
        let offset_y = rng.gen_range(-OFFSET_RANGE_TILE..=OFFSET_RANGE_TILE);
        let center_xy = (xy.0 + offset_x, xy.1 + offset_y);

        let lifetime_ms = rng.gen_range(WIND_CURVE_LIFETIME_MIN_MS..=WIND_CURVE_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        Self {
            center_xy,
            movement_direction: normalize_or_default(movement_direction),
            created_at,
            lifetime,
            length_tile: rng.gen_range(WIND_CURVE_LENGTH_MIN_TILE..=WIND_CURVE_LENGTH_MAX_TILE),
            amplitude_tile: rng
                .gen_range(WIND_CURVE_AMPLITUDE_MIN_TILE..=WIND_CURVE_AMPLITUDE_MAX_TILE),
            thickness_tile: rng
                .gen_range(WIND_CURVE_THICKNESS_MIN_TILE..=WIND_CURVE_THICKNESS_MAX_TILE),
            alpha: 1.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        let progress = self.progress(now);
        if progress <= FADE_START_PROGRESS {
            self.alpha = 1.0;
        } else {
            let t = (progress - FADE_START_PROGRESS) / (1.0 - FADE_START_PROGRESS);
            self.alpha = 1.0 - t;
        }
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 {
            return RenderingTree::Empty;
        }

        let movement = Xy::new(self.movement_direction.0, self.movement_direction.1);
        let perpendicular = Xy::new(-movement.y, movement.x);
        let center_tile = Xy::new(self.center_xy.0, self.center_xy.1);

        let half_len_tile = self.length_tile * 0.5;
        let amp_tile = self.amplitude_tile;

        let start_tile = center_tile - movement * half_len_tile;
        let end_tile = center_tile + movement * half_len_tile;

        let ctrl1_tile = start_tile + movement * (half_len_tile * 0.45) + perpendicular * amp_tile;
        let ctrl2_tile = end_tile - movement * (half_len_tile * 0.45) - perpendicular * amp_tile;

        let start = TILE_PX_SIZE.to_xy() * start_tile;
        let end = TILE_PX_SIZE.to_xy() * end_tile;
        let ctrl1 = TILE_PX_SIZE.to_xy() * ctrl1_tile;
        let ctrl2 = TILE_PX_SIZE.to_xy() * ctrl2_tile;

        let path = Path::new()
            .move_to(start.x, start.y)
            .cubic_to(ctrl1, ctrl2, end);

        let outer_color = Color::from_f01(
            OUTER_COLOR_RGB.0,
            OUTER_COLOR_RGB.1,
            OUTER_COLOR_RGB.2,
            self.alpha * OUTER_ALPHA_MULT,
        );
        let inner_color = Color::from_f01(
            INNER_COLOR_RGB.0,
            INNER_COLOR_RGB.1,
            INNER_COLOR_RGB.2,
            self.alpha,
        );

        let outer_paint = Paint::new(outer_color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(TILE_PX_SIZE.width * self.thickness_tile)
            .set_stroke_cap(StrokeCap::Round)
            .set_stroke_join(StrokeJoin::Round)
            .set_blend_mode(BlendMode::Screen);

        let inner_paint = Paint::new(inner_color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(TILE_PX_SIZE.width * self.thickness_tile * 0.45)
            .set_stroke_cap(StrokeCap::Round)
            .set_stroke_join(StrokeJoin::Round)
            .set_blend_mode(BlendMode::Screen);

        namui::render([
            namui::path(path.clone(), outer_paint),
            namui::path(path, inner_paint),
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

fn normalize_or_default(direction: (f32, f32)) -> (f32, f32) {
    let len = (direction.0 * direction.0 + direction.1 * direction.1).sqrt();
    if len > 0.0001 {
        (direction.0 / len, direction.1 / len)
    } else {
        (0.0, -1.0)
    }
}
