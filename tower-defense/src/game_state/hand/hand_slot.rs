use crate::{
    card::Card,
    game_state::hand::{
        HAND_SLOT_WH, HAND_WH, render_card::RenderCard, xy_with_spring::xy_with_spring,
    },
};
use namui::*;
use std::sync::atomic::AtomicUsize;

static HAND_SLOT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HandSlotId(usize);
impl HandSlotId {
    pub fn new() -> Self {
        let id = HAND_SLOT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        Self(id)
    }
}
impl Default for HandSlotId {
    fn default() -> Self {
        Self::new()
    }
}
impl From<HandSlotId> for AddKey {
    fn from(val: HandSlotId) -> Self {
        AddKey::U128(val.0 as u128)
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub(super) enum HandSlotKind {
    // TODO
    // Tower,
    Card { card: Card },
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ExitAnimation {
    pub start_time: Instant,
}

impl ExitAnimation {
    pub fn new(start_time: Instant) -> Self {
        Self { start_time }
    }

    pub fn is_complete(&self, current_time: Instant) -> bool {
        let elapsed = (current_time - self.start_time).as_secs_f32();
        elapsed >= 0.5 // 0.5초 후 완료
    }
}

#[derive(Debug, Clone)]
pub(super) struct HandSlot {
    pub id: HandSlotId,
    pub slot_kind: HandSlotKind,
    pub selected: bool,
    pub xy: Xy<Px>,
    pub exit_animation: Option<ExitAnimation>,
}
impl HandSlot {
    pub fn from_card(card: Card) -> Self {
        Self {
            id: HandSlotId::new(),
            slot_kind: HandSlotKind::Card { card },
            selected: false,
            xy: HAND_WH.to_xy(),
            exit_animation: None,
        }
    }

    pub fn set_xy(&mut self, xy: Xy<Px>) {
        self.xy = xy;
    }

    pub fn start_exit_animation(&mut self, now: Instant) {
        self.exit_animation = Some(ExitAnimation::new(now));
    }

    pub fn is_exit_animation_complete(&self, now: Instant) -> bool {
        if let Some(exit_anim) = self.exit_animation {
            exit_anim.is_complete(now)
        } else {
            false
        }
    }
}

impl Component for &HandSlot {
    fn render(self, ctx: &RenderCtx) {
        // Exit 애니메이션이 있는 경우 처리
        let (target_xy, target_scale) = if self.exit_animation.is_some() {
            (Xy::new(self.xy.x, HAND_WH.height), Xy::single(0.0))
        } else {
            let xy = match self.selected {
                true => self.xy - Xy::new(px(0.0), px(32.0)),
                false => self.xy,
            };
            let scale = match self.selected {
                true => Xy::single(1.05),
                false => Xy::single(1.0),
            };
            (xy, scale)
        };

        let animated_xy = xy_with_spring(ctx, target_xy, HAND_WH.to_xy());
        let animated_scale = xy_with_spring(ctx, target_scale, Xy::single(1.0));

        let half_slot_xy = HAND_SLOT_WH.to_xy() * 0.5;

        let ctx = ctx
            .translate(animated_xy)
            .translate(half_slot_xy)
            .scale(animated_scale)
            .translate(-half_slot_xy);

        match self.slot_kind {
            HandSlotKind::Card { card } => {
                ctx.add(RenderCard {
                    wh: HAND_SLOT_WH,
                    card,
                });
            }
        }
    }
}
