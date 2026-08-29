use super::super::UpgradeEntryState;
use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, no_cache, no_limit, push_acquired_upgrade, recovery_none, scalar, set_scalar,
};

fn card_rerolled_resolution(core: &mut crate::CoreState, index: usize) -> bool {
    set_scalar(
        &mut core.upgrades.upgrades[index],
        0,
        core.progress.left_dice as u64,
    )
}

fn stage_start_resolution(core: &mut crate::CoreState, index: usize, _: usize) -> bool {
    set_scalar(
        &mut core.upgrades.upgrades[index],
        0,
        core.progress.left_dice as u64,
    )
}

fn tower_bonus_resolution(upgrade: &UpgradeEntryState, _: &crate::TowerState) -> i64 {
    upgrade
        .ratio_values_raw
        .first()
        .copied()
        .unwrap_or(0)
        .saturating_mul(scalar(upgrade, 0).min(i64::MAX as usize) as i64)
}

fn tower_template_bonus_resolution(
    upgrade: &UpgradeEntryState,
    _: &crate::TowerTemplateState,
) -> i64 {
    upgrade
        .ratio_values_raw
        .first()
        .copied()
        .unwrap_or(0)
        .saturating_mul(scalar(upgrade, 0).min(i64::MAX as usize) as i64)
}

fn resolution_payload(u: &mut UpgradeEntryState) {
    u.ratio_values_raw.push(250_000);
    u.scalar_values.push(0);
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::Resolution,
    generate_payload: resolution_payload,
    rarity: crate::Rarity::Rare,
    cache: no_cache,
    acquire: push_acquired_upgrade,
    recovery: recovery_none,
    tower_bonus: tower_bonus_resolution,
    tower_bonus_for_template: tower_template_bonus_resolution,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: NO_TRIGGERS.monster_death,
        gold_earned: NO_TRIGGERS.gold_earned,
        card_rerolled: card_rerolled_resolution,
        shop_purchase: NO_TRIGGERS.shop_purchase,
        tower_placed: NO_TRIGGERS.tower_placed,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: stage_start_resolution,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
