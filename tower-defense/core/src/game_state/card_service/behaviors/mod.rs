use super::{
    CardServicePurchaseBlockReason, CardServiceSelectionState, CardServiceSelectionStepState,
    DeckState,
};
use enum_dispatch::enum_dispatch;

pub(super) mod battery;
pub(super) mod brush;
pub(super) mod cactus;
pub(super) mod club_sword;
pub(super) mod copier;
pub(super) mod eraser;
pub(super) mod fountain_pen;
pub(super) mod long_sword;
pub(super) mod mace;
pub(super) mod magic_wand;
pub(super) mod magnet;
pub(super) mod pliers;
pub(super) mod screwdriver;
pub(super) mod spinning_top;
pub(super) mod staff;
pub(super) mod support;
pub(super) mod tricycle;

#[enum_dispatch]
pub(crate) trait CardServiceBehavior {
    fn kind(&self) -> crate::CardServiceKind;
    fn selection_steps(&self) -> Vec<CardServiceSelectionStepState>;
    fn purchase_block_reasons(&self, deck: &DeckState) -> Vec<CardServicePurchaseBlockReason>;
    fn validate(
        &self,
        selection: &CardServiceSelectionState,
        deck: &DeckState,
        selected_card_ids: &[Vec<usize>],
    ) -> Result<(), crate::CommandError>;
    fn apply(
        &self,
        state: &mut crate::CoreState,
        selected_card_ids: &[Vec<usize>],
    ) -> Result<(), crate::CommandError>;
}

#[enum_dispatch(CardServiceBehavior)]
#[derive(Clone, Copy)]
pub(crate) enum CardServiceBehaviorImpl {
    Battery(battery::Behavior),
    Brush(brush::Behavior),
    Cactus(cactus::Behavior),
    ClubSword(club_sword::Behavior),
    Copier(copier::Behavior),
    Eraser(eraser::Behavior),
    FountainPen(fountain_pen::Behavior),
    LongSword(long_sword::Behavior),
    Mace(mace::Behavior),
    MagicWand(magic_wand::Behavior),
    Magnet(magnet::Behavior),
    Pliers(pliers::Behavior),
    Screwdriver(screwdriver::Behavior),
    SpinningTop(spinning_top::Behavior),
    Staff(staff::Behavior),
    Tricycle(tricycle::Behavior),
}
