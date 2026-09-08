use super::UpgradeBehavior;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolutionUpgradeState {
    pub reroll_damage_raw: i64,
    pub saved_rerolls: usize,
}

fn card_rerolled_resolution(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
) -> bool {
    let state = entry.resolution_mut();
    let saved_rerolls = context.progress.left_dice;
    let changed = state.saved_rerolls != saved_rerolls;
    state.saved_rerolls = saved_rerolls;
    changed
}

fn stage_start_resolution(
    context: &mut super::super::UpgradeTriggerContext,
    entry: &mut super::super::UpgradeEntry,
    _: usize,
) -> bool {
    card_rerolled_resolution(context, entry)
}

fn resolution_bonus(upgrade: &super::super::UpgradeEntry) -> i64 {
    let state = upgrade.resolution();
    state
        .reroll_damage_raw
        .saturating_mul(state.saved_rerolls.min(i64::MAX as usize) as i64)
}

fn tower_bonus_resolution(upgrade: &super::super::UpgradeEntry, _: &crate::TowerState) -> i64 {
    resolution_bonus(upgrade)
}

fn tower_template_bonus_resolution(
    upgrade: &super::super::UpgradeEntry,
    _: &crate::TowerTemplateState,
) -> i64 {
    resolution_bonus(upgrade)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Behavior;

impl UpgradeBehavior for Behavior {
    fn kind(&self) -> crate::UpgradeKind {
        crate::UpgradeKind::Resolution
    }
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Rare
    }
    fn generate(&self) -> super::super::UpgradeRuntimeState {
        super::super::UpgradeRuntimeState::Resolution(
            super::super::codec_impl::ResolutionUpgradeState {
                reroll_damage_raw: 250_000,
                saved_rerolls: 0,
            },
        )
    }
    fn tower_bonus(&self, entry: &super::super::UpgradeEntry, tower: &crate::TowerState) -> i64 {
        tower_bonus_resolution(entry, tower)
    }
    fn tower_bonus_for_template(
        &self,
        entry: &super::super::UpgradeEntry,
        template: &crate::TowerTemplateState,
    ) -> i64 {
        tower_template_bonus_resolution(entry, template)
    }
    fn card_rerolled(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
    ) -> bool {
        card_rerolled_resolution(context, entry)
    }
    fn stage_start(
        &self,
        context: &mut super::super::UpgradeTriggerContext,
        entry: &mut super::super::UpgradeEntry,
        stage: usize,
    ) -> bool {
        stage_start_resolution(context, entry, stage)
    }
}
