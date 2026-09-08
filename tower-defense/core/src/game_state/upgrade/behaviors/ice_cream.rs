use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IceCreamUpgradeState {
    pub damage_bonus_raw: i64,
    pub waves_remaining: usize,
}

fn stage_end_hamburger(
    _: &mut super::super::UpgradeTriggerContext,
    upgrade: &mut super::super::UpgradeEntry,
    _: bool,
    _: usize,
    _: usize,
) -> (bool, usize) {
    let state = upgrade.ice_cream_mut();
    if state.waves_remaining > 0 {
        state.waves_remaining -= 1;
        return (true, 0);
    }
    (false, 0)
}

fn tower_bonus_hamburger(upgrade: &super::super::UpgradeEntry, _: &crate::TowerState) -> i64 {
    let state = upgrade.ice_cream();
    if state.waves_remaining > 0 {
        state.damage_bonus_raw
    } else {
        0
    }
}

fn tower_template_bonus_hamburger(
    upgrade: &super::super::UpgradeEntry,
    _: &crate::TowerTemplateState,
) -> i64 {
    let state = upgrade.ice_cream();
    if state.waves_remaining > 0 {
        state.damage_bonus_raw
    } else {
        0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::IceCream
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::IceCream(
            super::super::codec_impl::IceCreamUpgradeState {
                damage_bonus_raw: 3_000_000,
                waves_remaining: 5,
            },
        )
    }
    fn tower_bonus(&self, entry: &super::super::UpgradeEntry, tower: &crate::TowerState) -> i64 {
        tower_bonus_hamburger(entry, tower)
    }
    fn tower_bonus_for_template(
        &self,
        entry: &super::super::UpgradeEntry,
        template: &crate::TowerTemplateState,
    ) -> i64 {
        tower_template_bonus_hamburger(entry, template)
    }
    fn stage_end(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (bool, usize) {
        stage_end_hamburger(context, entry, perfect_clear, gold, item_count)
    }
}
