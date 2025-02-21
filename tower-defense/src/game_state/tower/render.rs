use super::{Tower, TowerKind};
use crate::game_state::GameState;
use namui::*;

impl Component for &Tower {
    fn render(self, ctx: &RenderCtx) {
        let image = ctx.image(tower_image_resource_location(
            self.kind,
            self.animation.kind,
        ));

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

#[derive(Clone, Copy)]
pub enum AnimationKind {
    Idle1,
    Idle2,
    Attack,
}
impl AnimationKind {
    fn asset_id(&self) -> &str {
        match self {
            Self::Idle1 => "idle1",
            Self::Idle2 => "idle2",
            Self::Attack => "attack",
        }
    }
    fn duration(&self) -> Duration {
        match self {
            Self::Idle1 => Duration::from_millis(666),
            Self::Idle2 => Duration::from_millis(666),
            Self::Attack => Duration::from_millis(333),
        }
    }
}

pub fn tower_image_resource_location(
    tower_kind: TowerKind,
    animation_kind: AnimationKind,
) -> ResourceLocation {
    ResourceLocation::bundle(format!(
        "asset/image/tower/{}/{}.png",
        tower_kind.asset_id(),
        animation_kind.asset_id()
    ))
}
