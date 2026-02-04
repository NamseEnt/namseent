use crate::game_state::{TILE_PX_SIZE, attack};
use namui::*;

#[derive(Clone, State)]
pub struct InstantHitParticle {
    pub xy: (f32, f32),
    pub created_at: Instant,
    pub kind: attack::instant_effect::InstantEffectKind,
    pub scale: f32,
    pub progress: f32,
    pub current_scale: f32,
    pub alpha: f32,
}

impl InstantHitParticle {
    pub fn new(
        xy: (f32, f32),
        created_at: Instant,
        kind: attack::instant_effect::InstantEffectKind,
        scale: f32,
    ) -> Self {
        Self {
            xy,
            created_at,
            kind,
            scale,
            progress: 0.0,
            current_scale: scale,
            alpha: 1.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.progress = self.progress(now);
        self.current_scale = self.current_scale(now);
        self.alpha = self.current_alpha(now);
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 {
            return RenderingTree::Empty;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(self.xy.0, self.xy.1);

        match self.kind {
            attack::instant_effect::InstantEffectKind::Explosion => {
                let radius = 32.0 * self.current_scale;
                let num_points = 16;
                let mut path = Path::new();

                for i in 0..=num_points {
                    let angle = (i as f32 / num_points as f32) * std::f32::consts::PI * 2.0;
                    let x = xy_px.x + px(radius * angle.cos());
                    let y = xy_px.y + px(radius * angle.sin());
                    if i == 0 {
                        path = path.move_to(x, y);
                    } else {
                        path = path.line_to(x, y);
                    }
                }

                let color = Color::from_f01(1.0, 0.5, 0.0, self.alpha);
                let paint = Paint::new(color).set_style(PaintStyle::Fill);

                namui::path(path, paint)
            }
            attack::instant_effect::InstantEffectKind::Lightning => {
                let size = 24.0 * self.current_scale;
                let color = Color::from_f01(1.0, 1.0, 0.2, self.alpha);

                let mut path = Path::new();
                path = path.move_to(xy_px.x - px(size), xy_px.y);
                path = path.line_to(xy_px.x + px(size), xy_px.y);
                path = path.move_to(xy_px.x, xy_px.y - px(size));
                path = path.line_to(xy_px.x, xy_px.y + px(size));

                let paint = Paint::new(color)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(px(4.0 * self.current_scale))
                    .set_stroke_cap(StrokeCap::Round);

                namui::path(path, paint)
            }
            attack::instant_effect::InstantEffectKind::MagicCircle => {
                let radius = 28.0 * self.current_scale;
                let num_points = 16;
                let mut path = Path::new();

                for i in 0..=num_points {
                    let angle = (i as f32 / num_points as f32) * std::f32::consts::PI * 2.0;
                    let x = xy_px.x + px(radius * angle.cos());
                    let y = xy_px.y + px(radius * angle.sin());
                    if i == 0 {
                        path = path.move_to(x, y);
                    } else {
                        path = path.line_to(x, y);
                    }
                }

                let color = Color::from_f01(0.5, 0.2, 1.0, self.alpha);
                let paint = Paint::new(color)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(px(3.0));

                namui::path(path, paint)
            }
            attack::instant_effect::InstantEffectKind::FullHouseRain => {
                // FullHouse Rain 이펙트는 emitter에서 처리되므로 여기서는 렌더링하지 않음
                RenderingTree::Empty
            }
            attack::instant_effect::InstantEffectKind::FullHouseBurst => {
                // FullHouse Burst 이펙트는 emitter에서 처리되므로 여기서는 렌더링하지 않음
                RenderingTree::Empty
            }
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= attack::instant_effect::EFFECT_LIFETIME
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / attack::instant_effect::EFFECT_LIFETIME.as_secs_f32()).min(1.0)
    }

    fn current_scale(&self, now: Instant) -> f32 {
        let progress = self.progress(now);
        if progress < 0.2 {
            self.scale * (progress / 0.2)
        } else {
            self.scale * (1.0 - (progress - 0.2) / 0.8)
        }
    }

    fn current_alpha(&self, now: Instant) -> f32 {
        let progress = self.progress(now);
        if progress < 0.1 {
            progress / 0.1
        } else {
            1.0 - (progress - 0.1) / 0.9
        }
    }
}
