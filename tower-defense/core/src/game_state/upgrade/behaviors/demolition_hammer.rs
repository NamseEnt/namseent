use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DemolitionHammerUpgradeState;

fn tower_removed_dice(
    context: &mut super::super::UpgradeTriggerContext,
    _: &mut super::super::UpgradeEntry,
    rerolled_count: usize,
) {
    context.progress.left_dice = context.progress.left_dice.saturating_add(rerolled_count);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::DemolitionHammer
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Legendary
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::DemolitionHammer(
            super::super::codec_impl::DemolitionHammerUpgradeState,
        )
    }
    fn tower_removed(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        rerolled_count: usize,
    ) {
        tower_removed_dice(context, entry, rerolled_count)
    }
}
