use crate::game_state::TILE_PX_SIZE;
use namui::*;
use rand::Rng;

// Configurable parameters for sparkle particles
const SPARKLE_LIFETIME_MIN_MS: i64 = 150;
const SPARKLE_LIFETIME_MAX_MS: i64 = 300;
const SPARKLE_SIZE_TILE: f32 = 0.25; // Height of the diamond
const SPARKLE_WIDTH_RATIO: f32 = 0.4; // Width relative to height
const OFFSET_RANGE: f32 = 0.25; // 맵 좌표 단위
const VELOCITY_RANGE: f32 = 0.3; // 초기 속도 범위 (맵 좌표/초)
const SIZE_SCALE_MIN: f32 = 0.5; // 최소 크기 스케일
const SIZE_SCALE_MAX: f32 = 1.0; // 최대 크기 스케일

// Colors (RGB, 0.0..1.0) - Yellow tones
const SPARKLE_COLOR_RGB: (f32, f32, f32) = (1.0, 0.9, 0.2);
const SPARKLE_ALPHA: f32 = 0.8;

// Blinking animation configuration
const BLINK_CYCLES: f32 = 3.0; // Number of blink cycles during lifetime
const BLINK_RISE_RATIO: f32 = 0.4; // Proportion of cycle spent rising (0.0->1.0)

// Respawn configuration
const RESPAWN_CHANCE: f32 = 0.25; // 70% chance to respawn when particle dies
const MAX_RESPAWN_COUNT: u8 = 1; // Maximum number of times a particle can respawn

#[derive(Clone, State)]
pub struct SparkleParticle {
    pub xy: (f32, f32),
    pub velocity: (f32, f32),
    pub created_at: Instant,
    pub lifetime: Duration,
    pub alpha: f32,
    pub respawn_count: u8,
    pub size_scale: f32,
}

impl SparkleParticle {
    pub fn new_with_random<R: Rng + ?Sized>(
        xy: (f32, f32),
        created_at: Instant,
        rng: &mut R,
    ) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let final_xy = (xy.0 + offset_x, xy.1 + offset_y);

        // 랜덤 방향 생성
        let velocity_x = rng.gen_range(-VELOCITY_RANGE..=VELOCITY_RANGE);
        let velocity_y = rng.gen_range(-VELOCITY_RANGE..=VELOCITY_RANGE);

        let lifetime_ms = rng.gen_range(SPARKLE_LIFETIME_MIN_MS..=SPARKLE_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let size_scale = rng.gen_range(SIZE_SCALE_MIN..=SIZE_SCALE_MAX);

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            created_at,
            lifetime,
            alpha: SPARKLE_ALPHA,
            respawn_count: 0,
            size_scale,
        }
    }

    fn respawn_from<R: Rng + ?Sized>(&self, now: Instant, rng: &mut R) -> Self {
        let offset_x = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let offset_y = rng.gen_range(-OFFSET_RANGE..=OFFSET_RANGE);
        let final_xy = (self.xy.0 + offset_x, self.xy.1 + offset_y);

        // 랜덤 방향 생성
        let velocity_x = rng.gen_range(-VELOCITY_RANGE..=VELOCITY_RANGE);
        let velocity_y = rng.gen_range(-VELOCITY_RANGE..=VELOCITY_RANGE);

        let lifetime_ms = rng.gen_range(SPARKLE_LIFETIME_MIN_MS..=SPARKLE_LIFETIME_MAX_MS);
        let lifetime = Duration::from_millis(lifetime_ms);

        let size_scale = rng.gen_range(SIZE_SCALE_MIN..=SIZE_SCALE_MAX);

        Self {
            xy: final_xy,
            velocity: (velocity_x, velocity_y),
            created_at: now,
            lifetime,
            alpha: SPARKLE_ALPHA,
            respawn_count: self.respawn_count + 1,
            size_scale,
        }
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) -> Option<SparkleParticle> {
        let progress = self.progress(now);

        // 위치 업데이트
        let dt_sec = dt.as_secs_f32();
        self.xy.0 += self.velocity.0 * dt_sec;
        self.xy.1 += self.velocity.1 * dt_sec;

        // Blinking animation: multiple cycles of 0->1->0
        let cycle_progress = (progress * BLINK_CYCLES) % 1.0;

        if cycle_progress <= BLINK_RISE_RATIO {
            // Rising phase: 0 -> 1
            let t = cycle_progress / BLINK_RISE_RATIO;
            self.alpha = SPARKLE_ALPHA * t;
        } else {
            // Falling phase: 1 -> 0
            let t = (cycle_progress - BLINK_RISE_RATIO) / (1.0 - BLINK_RISE_RATIO);
            self.alpha = SPARKLE_ALPHA * (1.0 - t);
        }

        // Check if particle should respawn
        if self.is_done(now) && self.respawn_count < MAX_RESPAWN_COUNT {
            let mut rng = rand::thread_rng();
            if rng.gen_range(0.0..1.0) < RESPAWN_CHANCE {
                return Some(self.respawn_from(now, &mut rng));
            }
        }

        None
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.01 {
            return RenderingTree::Empty;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);

        // Create a vertical diamond shape using ovals
        // Diamond: narrow on sides (x), tall on top/bottom (y)
        let height_px = TILE_PX_SIZE.height * SPARKLE_SIZE_TILE * self.size_scale;
        let width_px = height_px * SPARKLE_WIDTH_RATIO;

        let (r, g, b) = SPARKLE_COLOR_RGB;
        let color = Color::from_f01(r, g, b, self.alpha);

        let paint = Paint::new(color)
            .set_style(PaintStyle::Fill)
            .set_blend_mode(BlendMode::Screen);

        // Create a diamond shape with 4 inward-curved sides
        // Define the 4 corner points of the diamond
        let top = Xy::new(xy_px.x, xy_px.y - height_px * 0.5);
        let right = Xy::new(xy_px.x + width_px * 0.5, xy_px.y);
        let bottom = Xy::new(xy_px.x, xy_px.y + height_px * 0.5);
        let left = Xy::new(xy_px.x - width_px * 0.5, xy_px.y);

        // Control factor: how much the curves pull inward (0.0=straight, 1.0=to center)
        let curve_factor = 0.6;

        // Calculate control points for inward curves
        let ctrl_top_right = Xy::new(
            xy_px.x + width_px * 0.5 * (1.0 - curve_factor),
            xy_px.y - height_px * 0.5 * (1.0 - curve_factor),
        );
        let ctrl_right_bottom = Xy::new(
            xy_px.x + width_px * 0.5 * (1.0 - curve_factor),
            xy_px.y + height_px * 0.5 * (1.0 - curve_factor),
        );
        let ctrl_bottom_left = Xy::new(
            xy_px.x - width_px * 0.5 * (1.0 - curve_factor),
            xy_px.y + height_px * 0.5 * (1.0 - curve_factor),
        );
        let ctrl_left_top = Xy::new(
            xy_px.x - width_px * 0.5 * (1.0 - curve_factor),
            xy_px.y - height_px * 0.5 * (1.0 - curve_factor),
        );

        // Create path with 4 cubic bezier curves (simulating quadratic curves)
        // Convert quadratic bezier to cubic: ctrl1 = start + 2/3*(ctrl-start), ctrl2 = end + 2/3*(ctrl-end)

        // Top to Right curve
        let ctrl1_tr = top + (ctrl_top_right - top) * (2.0 / 3.0);
        let ctrl2_tr = right + (ctrl_top_right - right) * (2.0 / 3.0);

        // Right to Bottom curve
        let ctrl1_rb = right + (ctrl_right_bottom - right) * (2.0 / 3.0);
        let ctrl2_rb = bottom + (ctrl_right_bottom - bottom) * (2.0 / 3.0);

        // Bottom to Left curve
        let ctrl1_bl = bottom + (ctrl_bottom_left - bottom) * (2.0 / 3.0);
        let ctrl2_bl = left + (ctrl_bottom_left - left) * (2.0 / 3.0);

        // Left to Top curve
        let ctrl1_lt = left + (ctrl_left_top - left) * (2.0 / 3.0);
        let ctrl2_lt = top + (ctrl_left_top - top) * (2.0 / 3.0);

        let diamond = Path::new()
            .move_to(top.x, top.y)
            .cubic_to(ctrl1_tr, ctrl2_tr, right)
            .cubic_to(ctrl1_rb, ctrl2_rb, bottom)
            .cubic_to(ctrl1_bl, ctrl2_bl, left)
            .cubic_to(ctrl1_lt, ctrl2_lt, top)
            .close();

        namui::path(diamond, paint)
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.lifetime
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / self.lifetime.as_secs_f32()).min(1.0)
    }
}
