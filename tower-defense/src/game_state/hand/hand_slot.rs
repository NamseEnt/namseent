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

#[derive(Debug, Clone)]
pub(super) struct HandSlot {
    pub id: HandSlotId,
    pub slot_kind: HandSlotKind,
    pub selected: bool,
    pub xy: Xy<Px>,
}
impl HandSlot {
    pub fn from_card(card: Card) -> Self {
        Self {
            id: HandSlotId::new(),
            slot_kind: HandSlotKind::Card { card },
            selected: false,
            xy: HAND_WH.to_xy(),
        }
    }

    pub fn set_xy(&mut self, xy: Xy<Px>) {
        self.xy = xy;
    }
}

impl Component for &HandSlot {
    fn render(self, ctx: &RenderCtx) {
        let target_xy = match self.selected {
            true => self.xy - Xy::new(px(0.0), px(32.0)),
            false => self.xy,
        };
        let animated_xy = xy_with_spring(ctx, target_xy, HAND_WH.to_xy());

        let ctx = ctx.translate(animated_xy);
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
