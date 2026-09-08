use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FangUpgradeState {
    pub heal_per_kill: usize,
}

fn monster_death_heal(
    _: &mut super::super::UpgradeTriggerContext,
    entry: &super::super::UpgradeEntry,
    _: &mut usize,
    healing: &mut i64,
) {
    let amount = entry.fang().heal_per_kill;
    *healing = healing.saturating_add((amount.min(i64::MAX as usize) as i64).saturating_mul(1_000));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Fang
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Fang(super::super::codec_impl::FangUpgradeState {
            heal_per_kill: 10,
        })
    }
    fn monster_death(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &super::super::UpgradeEntry,
        gold: &mut usize,
        healing: &mut i64,
    ) {
        monster_death_heal(context, entry, gold, healing)
    }
}
