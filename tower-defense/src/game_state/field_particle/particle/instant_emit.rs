use crate::game_state::{TILE_PX_SIZE, attack};
use namui::*;

#[derive(Clone, State)]
pub struct InstantEmitParticle {
    pub tower_xy: (f32, f32),
    pub target_xy: (f32, f32),
    pub created_at: Instant,
    pub kind: attack::instant_effect::InstantEffectKind,
    pub progress: f32,
    pub alpha: f32,
}

impl InstantEmitParticle {
    pub fn new(
        tower_xy: (f32, f32),
        target_xy: (f32, f32),
        created_at: Instant,
        kind: attack::instant_effect::InstantEffectKind,
    ) -> Self {
        Self {
            tower_xy,
            target_xy,
            created_at,
            kind,
            progress: 0.0,
            alpha: 1.0,
        }
    }

    pub fn tick(&mut self, now: Instant, _dt: Duration) {
        self.progress = self.progress(now);
        self.alpha = if self.progress < 0.5 {
            1.0
        } else {
            1.0 - (self.progress - 0.5) * 2.0
        };
    }

    pub fn render(&self) -> RenderingTree {
        if self.alpha <= 0.0 {
            return RenderingTree::Empty;
        }

        let tower_px = TILE_PX_SIZE.to_xy() * Xy::new(self.tower_xy.0, self.tower_xy.1);
        let target_px = TILE_PX_SIZE.to_xy() * Xy::new(self.target_xy.0, self.target_xy.1);

        let current_end = tower_px + (target_px - tower_px) * (self.progress * 2.0).min(1.0);

        let color = match self.kind {
            attack::instant_effect::InstantEffectKind::Explosion => {
                Color::from_f01(1.0, 0.5, 0.0, self.alpha)
            }
            attack::instant_effect::InstantEffectKind::Lightning => {
                Color::from_f01(1.0, 1.0, 0.2, self.alpha)
            }
            attack::instant_effect::InstantEffectKind::MagicCircle => {
                Color::from_f01(0.5, 0.2, 1.0, self.alpha)
            }
        };

        let mut path = Path::new();
        path = path.move_to(tower_px.x, tower_px.y);
        path = path.line_to(current_end.x, current_end.y);

        let paint = Paint::new(color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(px(4.0))
            .set_stroke_cap(StrokeCap::Round);

        namui::path(path, paint)
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= attack::instant_effect::EFFECT_LIFETIME
    }

    fn progress(&self, now: Instant) -> f32 {
        let elapsed = now - self.created_at;
        (elapsed.as_secs_f32() / attack::instant_effect::EFFECT_LIFETIME.as_secs_f32()).min(1.0)
    }
}
