use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TrophyUpgradeState;

fn stage_end_free_card(
    context: &mut super::super::UpgradeTriggerContext,
    _: &mut super::super::UpgradeEntry,
    perfect_clear: bool,
    _: usize,
    _: usize,
) -> (bool, usize) {
    if perfect_clear {
        context.stage_modifiers.free_card_services =
            context.stage_modifiers.free_card_services.saturating_add(1);
    }
    (false, 0)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Trophy
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Legendary
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Trophy(super::super::codec_impl::TrophyUpgradeState)
    }
    fn stage_end(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (bool, usize) {
        stage_end_free_card(context, entry, perfect_clear, gold, item_count)
    }
}
