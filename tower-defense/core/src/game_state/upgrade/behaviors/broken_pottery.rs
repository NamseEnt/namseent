use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BrokenPotteryUpgradeState {
    pub rerolled_count: usize,
}

const BROKEN_POTTERY_REROLL_INTERVAL: usize = 4;

fn card_rerolled_broken_pottery(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
) -> bool {
    let state = entry.broken_pottery_mut();
    state.rerolled_count = state.rerolled_count.saturating_add(1);
    if state.rerolled_count >= BROKEN_POTTERY_REROLL_INTERVAL {
        state.rerolled_count = 0;
        context.progress.left_dice = context.progress.left_dice.saturating_add(1);
    }
    true
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
            super::super::codec_impl::BrokenPotteryUpgradeState::default(),
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
