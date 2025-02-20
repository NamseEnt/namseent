use super::Tower;
use crate::game_state::GameState;
use namui::*;

impl Component for &Tower {
    fn render(self, ctx: &RenderCtx) {
        let animation_name = match self.animation.kind {
            AnimationKind::Idle1 => "idle1",
            AnimationKind::Idle2 => "idle2",
            AnimationKind::Attack => "attack",
        };
        let image = ctx.image(ResourceLocation::bundle(format!(
            "asset/image/tower/{}/{animation_name}.png",
            self.kind.asset_id(),
        )));

        if let Some(Ok(image)) = image.as_ref() {
            ctx.add(namui::image(ImageParam {
                rect: Rect::from_xy_wh(Xy::zero(), image.info.wh()),
                image: image.clone(),
                style: ImageStyle {
                    fit: ImageFit::None,
                    paint: None,
                },
            }));
        }
    }
}

pub fn tower_animation_tick(game_state: &mut GameState, now: Instant) {
    game_state.towers.iter_mut().for_each(|tower| {
        let animation = &mut tower.animation;

        if now - animation.start_at < animation.duration() {
            return;
        }

        animation.transition(match animation.kind {
            AnimationKind::Idle1 => AnimationKind::Idle2,
            AnimationKind::Idle2 => AnimationKind::Idle1,
            AnimationKind::Attack => AnimationKind::Idle1,
        });
    });
}
pub(super) struct Animation {
    kind: AnimationKind,
    start_at: Instant,
}

impl Animation {
    pub(super) fn new() -> Self {
        Self {
            kind: AnimationKind::Idle1,
            start_at: Instant::now(),
        }
    }

    pub(super) fn transition(&mut self, kind: AnimationKind) {
        self.kind = kind;
        self.start_at = Instant::now();
    }

    fn duration(&self) -> Duration {
        self.kind.duration()
    }
}

pub(super) enum AnimationKind {
    Idle1,
    Idle2,
    Attack,
}

impl AnimationKind {
    fn duration(&self) -> Duration {
        match self {
            Self::Idle1 => Duration::from_millis(666),
            Self::Idle2 => Duration::from_millis(666),
            Self::Attack => Duration::from_millis(333),
        }
    }
}
