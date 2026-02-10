use crate::game_state::TILE_PX_SIZE;
use namui::*;
use rand::Rng;

const LIGHTNING_BOLT_MIN_POINTS: usize = 4;
const LIGHTNING_BOLT_MAX_POINTS: usize = 6;
const LIGHTNING_BOLT_OFFSET_RANGE: f32 = 0.2;

const LIGHTNING_BOLT_SPAWN_LIFETIME_MIN_MS: i64 = 50;
const LIGHTNING_BOLT_SPAWN_LIFETIME_MAX_MS: i64 = 150;
const LIGHTNING_BOLT_SPAWN_CHANCE_REDUCTION: f32 = 0.8;
const LIGHTNING_BOLT_END_OFFSET_RANGE: f32 = 0.2;

const LIGHTNING_BOLT_LINE_THICKNESS_FULL: f32 = 0.01;
const LIGHTNING_BOLT_LINE_THICKNESS_MID: f32 = 0.03;
const LIGHTNING_BOLT_LINE_THICKNESS_CENTER: f32 = 0.05;
const LIGHTNING_BOLT_LINE_INNER_THICKNESS_RATIO: f32 = 0.4;
const LIGHTNING_BOLT_ALPHA_APPEAR_PHASE: f32 = 0.2;

const OUTER_COLOR_RGB: (f32, f32, f32) = (0.2, 0.5, 1.0);
const INNER_COLOR_RGB: (f32, f32, f32) = (0.6, 0.85, 1.0);
const INNER_COLOR_ALPHA_RATIO: f32 = 0.8;

#[derive(Clone)]
pub struct LightningBoltParticle {
    pub points: Vec<(f32, f32)>,
    pub created_at: Instant,
    pub duration: Duration,
    pub alpha: f32,
    pub spawn_chance: f32,
    pub has_spawned: bool,
}

impl LightningBoltParticle {
    pub fn new(
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        duration: Duration,
        spawn_chance: f32,
    ) -> Self {
        let points = Self::generate_points(start_xy, end_xy);

        Self {
            points,
            created_at,
            duration,
            alpha: 1.0,
            spawn_chance,
            has_spawned: false,
        }
    }

    fn generate_points(start_xy: (f32, f32), end_xy: (f32, f32)) -> Vec<(f32, f32)> {
        let mut rng = rand::thread_rng();
        let point_count = rng.gen_range(LIGHTNING_BOLT_MIN_POINTS..=LIGHTNING_BOLT_MAX_POINTS);

        let dx = end_xy.0 - start_xy.0;
        let dy = end_xy.1 - start_xy.1;
        let length = (dx * dx + dy * dy).sqrt();

        let perp_x = -dy / length.max(0.001);
        let perp_y = dx / length.max(0.001);

        let mut points = Vec::with_capacity(point_count);

        for i in 0..point_count {
            let t = i as f32 / (point_count - 1) as f32;

            let base_x = start_xy.0 + dx * t;
            let base_y = start_xy.1 + dy * t;

            let offset = if i == 0 || i == point_count - 1 {
                0.0
            } else {
                rng.gen_range(-LIGHTNING_BOLT_OFFSET_RANGE..LIGHTNING_BOLT_OFFSET_RANGE)
            };

            let x = base_x + perp_x * offset;
            let y = base_y + perp_y * offset;

            points.push((x, y));
        }

        points
    }

    pub fn tick_impl(&mut self, now: Instant, _dt: Duration) {
        self.alpha = self.current_alpha(now);

        if !self.has_spawned && self.is_done(now) && self.points.len() >= 2 {
            self.has_spawned = true;
            if let Some(child) = self.try_spawn_child(now) {
                crate::game_state::field_particle::LIGHTNING_BOLTS.spawn(child);
            }
        }
    }

    fn try_spawn_child(&self, now: Instant) -> Option<LightningBoltParticle> {
        let mut rng = rand::thread_rng();

        if rng.gen_range(0.0..1.0) >= self.spawn_chance {
            return None;
        }

        let mid_idx = self.points.len() / 2;
        let start_xy = self.points[mid_idx];

        let last_xy = self.points[self.points.len() - 1];
        let offset_x =
            rng.gen_range(-LIGHTNING_BOLT_END_OFFSET_RANGE..LIGHTNING_BOLT_END_OFFSET_RANGE);
        let offset_y =
            rng.gen_range(-LIGHTNING_BOLT_END_OFFSET_RANGE..LIGHTNING_BOLT_END_OFFSET_RANGE);
        let end_xy = (last_xy.0 + offset_x, last_xy.1 + offset_y);

        let duration = Duration::from_millis(
            rng.gen_range(LIGHTNING_BOLT_SPAWN_LIFETIME_MIN_MS..LIGHTNING_BOLT_SPAWN_LIFETIME_MAX_MS),
        );

        let new_spawn_chance = self.spawn_chance * LIGHTNING_BOLT_SPAWN_CHANCE_REDUCTION;

        Some(LightningBoltParticle::new(
            start_xy,
            end_xy,
            now,
            duration,
            new_spawn_chance,
        ))
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 || self.points.len() < 2 {
            return RenderingTree::Empty;
        }

        let mut renders = Vec::new();

        let line_configs = [(0, self.points.len(), LIGHTNING_BOLT_LINE_THICKNESS_FULL)];
        self.render_line_with_configs(&line_configs, &mut renders);

        if self.points.len() > 2 {
            let line_configs = [(1, self.points.len() - 1, LIGHTNING_BOLT_LINE_THICKNESS_MID)];
            self.render_line_with_configs(&line_configs, &mut renders);
        }

        if self.points.len() > 4 {
            let line_configs = [(
                2,
                self.points.len() - 2,
                LIGHTNING_BOLT_LINE_THICKNESS_CENTER,
            )];
            self.render_line_with_configs(&line_configs, &mut renders);
        }

        namui::render(renders)
    }

    fn render_line_with_configs(
        &self,
        configs: &[(usize, usize, f32)],
        renders: &mut Vec<RenderingTree>,
    ) {
        for &(start_idx, end_idx, thickness) in configs {
            let outer_color = Color::from_f01(
                OUTER_COLOR_RGB.0,
                OUTER_COLOR_RGB.1,
                OUTER_COLOR_RGB.2,
                self.alpha,
            );

            let mut outer_path = Path::new();
            let first_point = self.points[start_idx];
            let first_px = TILE_PX_SIZE.to_xy() * Xy::new(first_point.0, first_point.1);
            outer_path = outer_path.move_to(first_px.x, first_px.y);

            for i in (start_idx + 1)..end_idx {
                let point = self.points[i];
                let point_px = TILE_PX_SIZE.to_xy() * Xy::new(point.0, point.1);
                outer_path = outer_path.line_to(point_px.x, point_px.y);
            }

            let outer_paint = Paint::new(outer_color)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(TILE_PX_SIZE.width * thickness)
                .set_stroke_cap(StrokeCap::Round)
                .set_stroke_join(StrokeJoin::Round)
                .set_blend_mode(BlendMode::Screen);

            renders.push(namui::path(outer_path, outer_paint));

            let inner_alpha = self.alpha * INNER_COLOR_ALPHA_RATIO;
            let inner_color = Color::from_f01(
                INNER_COLOR_RGB.0,
                INNER_COLOR_RGB.1,
                INNER_COLOR_RGB.2,
                inner_alpha,
            );

            let mut inner_path = Path::new();
            inner_path = inner_path.move_to(first_px.x, first_px.y);

            for i in (start_idx + 1)..end_idx {
                let point = self.points[i];
                let point_px = TILE_PX_SIZE.to_xy() * Xy::new(point.0, point.1);
                inner_path = inner_path.line_to(point_px.x, point_px.y);
            }

            let inner_paint = Paint::new(inner_color)
                .set_style(PaintStyle::Stroke)
                .set_stroke_width(
                    TILE_PX_SIZE.width * thickness * LIGHTNING_BOLT_LINE_INNER_THICKNESS_RATIO,
                )
                .set_stroke_cap(StrokeCap::Round)
                .set_stroke_join(StrokeJoin::Round)
                .set_blend_mode(BlendMode::Screen);

            renders.push(namui::path(inner_path, inner_paint));
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }

    fn current_alpha(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        if elapsed >= self.duration {
            return 0.0;
        }

        let progress = elapsed.as_secs_f32() / self.duration.as_secs_f32();

        if progress < LIGHTNING_BOLT_ALPHA_APPEAR_PHASE {
            let appear_progress = progress / LIGHTNING_BOLT_ALPHA_APPEAR_PHASE;
            let inv = 1.0 - appear_progress;
            1.0 - (inv * inv)
        } else {
            let fade_progress = (progress - LIGHTNING_BOLT_ALPHA_APPEAR_PHASE)
                / (1.0 - LIGHTNING_BOLT_ALPHA_APPEAR_PHASE);
            1.0 - fade_progress
        }
    }
}

impl namui::particle::Particle for LightningBoltParticle {
    fn tick(&mut self, now: Instant, dt: Duration) {
        self.tick_impl(now, dt);
    }

    fn render(&self) -> RenderingTree {
        LightningBoltParticle::render(self)
    }

    fn is_done(&self, now: Instant) -> bool {
        LightningBoltParticle::is_done(self, now)
    }
}
