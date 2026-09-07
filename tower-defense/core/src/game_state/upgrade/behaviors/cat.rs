use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CatUpgradeState {
    pub gold_per_kill: usize,
}
fn monster_death_gold(
    _: &mut super::super::UpgradeTriggerContext,
    entry: &super::super::UpgradeEntry,
    gold: &mut usize,
    _: &mut i64,
) {
    *gold = gold.saturating_add(entry.cat().gold_per_kill);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Cat
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Epic
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Cat(super::super::codec_impl::CatUpgradeState {
            gold_per_kill: 1,
        })
    }
    fn monster_death(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &super::super::UpgradeEntry,
        gold: &mut usize,
        healing: &mut i64,
    ) {
        monster_death_gold(context, entry, gold, healing)
    }
}
