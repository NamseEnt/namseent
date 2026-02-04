use crate::game_state::TILE_PX_SIZE;
use crate::game_state::projectile::ProjectileKind;
use namui::*;
use rand::Rng;

const TRASH_SIZE_TILE: f32 = 0.5;
// max absolute rotation speed in degrees per second for random particles
const ROTATION_SPEED_MAX_DEG: f32 = 360.0;

#[derive(Clone, Copy, State)]
pub enum EaseMode {
    Linear,
    EaseOutCubic,
}

#[derive(Clone, State)]
pub struct TrashParticle {
    pub kind: ProjectileKind,
    pub start_xy: (f32, f32),
    pub end_xy: (f32, f32),
    pub created_at: Instant,
    pub duration: Duration,
    pub progress: f32,
    pub ease_mode: EaseMode,
    pub rotation: Angle,
    pub rotation_speed: Angle, // per second
}

impl TrashParticle {
    pub fn new(
        kind: ProjectileKind,
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        duration: Duration,
        ease_mode: EaseMode,
    ) -> Self {
        // default rotation / speed 0
        Self {
            kind,
            start_xy,
            end_xy,
            created_at,
            duration,
            progress: 0.0,
            ease_mode,
            rotation: 0.deg(),
            rotation_speed: 0.deg(),
        }
    }

    pub fn tick(&mut self, now: Instant, dt: Duration) {
        self.progress = self.progress(now);
        self.rotation += self.rotation_speed * dt.as_secs_f32();
    }

    pub fn render(&self) -> RenderingTree {
        if self.progress >= 1.0 {
            return RenderingTree::Empty;
        }

        let eased = match self.ease_mode {
            EaseMode::Linear => self.progress,
            EaseMode::EaseOutCubic => {
                let inv = 1.0 - self.progress;
                1.0 - (inv * inv * inv)
            }
        };

        let x = self.start_xy.0 + (self.end_xy.0 - self.start_xy.0) * eased;
        let y = self.start_xy.1 + (self.end_xy.1 - self.start_xy.1) * eased;

        let px_xy = TILE_PX_SIZE.to_xy() * Xy::new(x, y);

        let trash_size_px = TILE_PX_SIZE.width * TRASH_SIZE_TILE;
        let wh = Wh::new(trash_size_px, trash_size_px);

        let image = self.kind.image();

        // alpha fades out linearly with progress so it's not eased
        let alpha = (1.0 - self.progress).max(0.0);
        let paint = Paint::new(Color::WHITE.with_alpha((alpha * 255.0) as u8));

        namui::translate(
            px_xy.x,
            px_xy.y,
            namui::rotate(
                self.rotation,
                namui::translate(
                    -wh.width * 0.5,
                    -wh.height * 0.5,
                    namui::image(ImageParam {
                        rect: Rect::from_xy_wh(Xy::new(0.px(), 0.px()), wh),
                        image,
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint: Some(paint),
                        },
                    }),
                ),
            ),
        )
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / self.duration.as_secs_f32()).min(1.0)
    }

    // helper to create with small random offset on end position
    pub fn new_with_random_end(
        kind: ProjectileKind,
        start_xy: (f32, f32),
        end_xy: (f32, f32),
        created_at: Instant,
        duration: Duration,
        ease_mode: EaseMode,
    ) -> Self {
        let mut rng = rand::thread_rng();
        let offset_x = rng.gen_range(-0.25..0.25);
        let offset_y = rng.gen_range(-0.1..0.1);
        let end = (end_xy.0 + offset_x, end_xy.1 + offset_y);
        let mut s = Self::new(kind, start_xy, end, created_at, duration, ease_mode);
        // random rotation and speed
        s.rotation = rng.gen_range(0.0..360.0).deg();
        s.rotation_speed = rng
            .gen_range(-ROTATION_SPEED_MAX_DEG..ROTATION_SPEED_MAX_DEG)
            .deg();
        s
    }
}
