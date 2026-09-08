use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PiggyBankUpgradeState {
    pub gold_per_step: usize,
}

const PIGGY_BANK_GOLD_REWARD_PER_STEP: usize = 10;

const PIGGY_BANK_GOLD_STEP: usize = 100;

fn acquire_piggy_bank(
    core: &mut crate::CoreState,
    mut upgrade: super::super::UpgradeEntry,
) -> usize {
    let add = upgrade.piggy_bank().gold_per_step;
    if let Some(existing) = core
        .upgrades
        .upgrades
        .iter_mut()
        .find(|entry| entry.kind() == crate::UpgradeKind::PiggyBank)
    {
        existing.piggy_bank_mut().gold_per_step =
            existing.piggy_bank().gold_per_step.saturating_add(add);
    } else {
        upgrade.id = core.next_upgrade_id();
        core.upgrades.upgrades.push(upgrade);
    }
    0
}

fn stage_end_piggy_bank(
    _: &mut super::super::UpgradeTriggerContext,
    _: &mut super::super::UpgradeEntry,
    _: bool,
    gold: usize,
    _: usize,
) -> (bool, usize) {
    (
        false,
        gold / PIGGY_BANK_GOLD_STEP * PIGGY_BANK_GOLD_REWARD_PER_STEP,
    )
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::PiggyBank
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::PiggyBank(
            super::super::codec_impl::PiggyBankUpgradeState { gold_per_step: 10 },
        )
    }
    fn acquire(&self, core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
        acquire_piggy_bank(core, upgrade)
    }
    fn stage_end(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (bool, usize) {
        stage_end_piggy_bank(context, entry, perfect_clear, gold, item_count)
    }
}
