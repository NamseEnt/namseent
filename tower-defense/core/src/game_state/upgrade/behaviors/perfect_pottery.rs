use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PerfectPotteryUpgradeState {
    pub damage_bonus_raw: i64,
}
use super::support::push_acquired_upgrade;

fn tower_bonus_no_reroll(upgrade: &super::super::UpgradeEntry, tower: &crate::TowerState) -> i64 {
    let state = upgrade.perfect_pottery();
    if tower.template.rerolled_count == 0 {
        state.damage_bonus_raw
    } else {
        0
    }
}

fn tower_template_bonus_no_reroll(
    upgrade: &super::super::UpgradeEntry,
    template: &crate::TowerTemplateState,
) -> i64 {
    let state = upgrade.perfect_pottery();
    if template.rerolled_count == 0 {
        state.damage_bonus_raw
    } else {
        0
    }
}

fn acquire_perfect_pottery(
    core: &mut crate::CoreState,
    upgrade: super::super::UpgradeEntry,
) -> usize {
    let add = upgrade.perfect_pottery().damage_bonus_raw;
    if let Some(existing) = core
        .upgrades
        .upgrades
        .iter_mut()
        .find(|entry| entry.kind() == crate::UpgradeKind::PerfectPottery)
    {
        existing.perfect_pottery_mut().damage_bonus_raw = existing
            .perfect_pottery()
            .damage_bonus_raw
            .saturating_add(add);
        0
    } else {
        push_acquired_upgrade(core, upgrade)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::PerfectPottery
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::PerfectPottery(
            super::super::codec_impl::PerfectPotteryUpgradeState {
                damage_bonus_raw: 500_000,
            },
        )
    }
    fn acquire(&self, core: &mut crate::CoreState, upgrade: super::super::UpgradeEntry) -> usize {
        acquire_perfect_pottery(core, upgrade)
    }
    fn tower_bonus(&self, entry: &super::super::UpgradeEntry, tower: &crate::TowerState) -> i64 {
        tower_bonus_no_reroll(entry, tower)
    }
    fn tower_bonus_for_template(
        &self,
        entry: &super::super::UpgradeEntry,
        template: &crate::TowerTemplateState,
    ) -> i64 {
        tower_template_bonus_no_reroll(entry, template)
    }
}
