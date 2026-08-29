use super::super::UpgradeEntryState;
use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, no_cache, no_limit, popcorn_damage_bonus_raw, popcorn_damage_bonus_raw_with_waves,
    push_acquired_upgrade, recovery_none, scalar, set_ratio, set_scalar,
};

fn popcorn_payload(u: &mut UpgradeEntryState) {
    u.ratio_values_raw.extend([5_000_000, 0]);
    u.scalar_values.extend([5, 5]);
}

fn stage_start_popcorn(core: &mut crate::CoreState, index: usize, _: usize) -> bool {
    let active = popcorn_damage_bonus_raw(&core.upgrades.upgrades[index]);
    set_ratio(&mut core.upgrades.upgrades[index], 1, active)
}

fn stage_end_popcorn(
    core: &mut crate::CoreState,
    index: usize,
    _: bool,
    _: usize,
    _: usize,
) -> (bool, usize) {
    let waves_remaining = scalar(&core.upgrades.upgrades[index], 1);
    if waves_remaining == 0 {
        return (false, 0);
    }
    let next_waves_remaining = waves_remaining - 1;
    let active =
        popcorn_damage_bonus_raw_with_waves(&core.upgrades.upgrades[index], next_waves_remaining);
    let changed = set_scalar(
        &mut core.upgrades.upgrades[index],
        1,
        next_waves_remaining as u64,
    );
    let changed = set_ratio(&mut core.upgrades.upgrades[index], 1, active) || changed;
    (changed, 0)
}

fn tower_bonus_popcorn(upgrade: &UpgradeEntryState, _: &crate::TowerState) -> i64 {
    upgrade.ratio_values_raw.get(1).copied().unwrap_or(0)
}

fn tower_template_bonus_popcorn(upgrade: &UpgradeEntryState, _: &crate::TowerTemplateState) -> i64 {
    upgrade.ratio_values_raw.get(1).copied().unwrap_or(0)
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::Popcorn,
    generate_payload: popcorn_payload,
    rarity: crate::Rarity::Rare,
    cache: no_cache,
    acquire: push_acquired_upgrade,
    recovery: recovery_none,
    tower_bonus: tower_bonus_popcorn,
    tower_bonus_for_template: tower_template_bonus_popcorn,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: NO_TRIGGERS.monster_death,
        gold_earned: NO_TRIGGERS.gold_earned,
        card_rerolled: NO_TRIGGERS.card_rerolled,
        shop_purchase: NO_TRIGGERS.shop_purchase,
        tower_placed: NO_TRIGGERS.tower_placed,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: stage_start_popcorn,
        stage_end: stage_end_popcorn,
    },
};
