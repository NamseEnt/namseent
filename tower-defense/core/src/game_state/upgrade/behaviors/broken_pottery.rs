use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BrokenPotteryUpgradeState;

const BROKEN_POTTERY_REROLL_INTERVAL: usize = 4;

fn card_rerolled_broken_pottery(
    context: &mut super::super::UpgradeTriggerContext,
    _: &mut super::super::UpgradeEntry,
) -> bool {
    if context.progress.rerolled_count > 0
        && context
            .progress
            .rerolled_count
            .is_multiple_of(BROKEN_POTTERY_REROLL_INTERVAL)
    {
        context.progress.left_dice = context.progress.left_dice.saturating_add(1);
    }
    false
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::BrokenPottery
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Legendary
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::BrokenPottery(
            super::super::codec_impl::BrokenPotteryUpgradeState,
        )
    }
    fn card_rerolled(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
    ) -> bool {
        card_rerolled_broken_pottery(context, entry)
    }
}
