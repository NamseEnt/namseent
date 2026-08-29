use super::super::UpgradeEntryState;
use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, no_cache, no_limit, push_acquired_upgrade, recovery_none, scalar, set_scalar,
};

fn hamburger_payload(u: &mut UpgradeEntryState) {
    u.ratio_values_raw.push(3_000_000);
    u.scalar_values.push(5);
}

fn stage_end_hamburger(
    core: &mut crate::CoreState,
    index: usize,
    _: bool,
    _: usize,
    _: usize,
) -> (bool, usize) {
    let waves_remaining = scalar(&core.upgrades.upgrades[index], 0);
    if waves_remaining > 0 {
        return (
            set_scalar(
                &mut core.upgrades.upgrades[index],
                0,
                (waves_remaining - 1) as u64,
            ),
            0,
        );
    }
    (false, 0)
}

fn tower_bonus_hamburger(upgrade: &UpgradeEntryState, _: &crate::TowerState) -> i64 {
    if scalar(upgrade, 0) > 0 {
        upgrade.ratio_values_raw.first().copied().unwrap_or(0)
    } else {
        0
    }
}

fn tower_template_bonus_hamburger(
    upgrade: &UpgradeEntryState,
    _: &crate::TowerTemplateState,
) -> i64 {
    if scalar(upgrade, 0) > 0 {
        upgrade.ratio_values_raw.first().copied().unwrap_or(0)
    } else {
        0
    }
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::IceCream,
    generate_payload: hamburger_payload,
    rarity: crate::Rarity::Rare,
    cache: no_cache,
    acquire: push_acquired_upgrade,
    recovery: recovery_none,
    tower_bonus: tower_bonus_hamburger,
    tower_bonus_for_template: tower_template_bonus_hamburger,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: NO_TRIGGERS.monster_death,
        gold_earned: NO_TRIGGERS.gold_earned,
        card_rerolled: NO_TRIGGERS.card_rerolled,
        shop_purchase: NO_TRIGGERS.shop_purchase,
        tower_placed: NO_TRIGGERS.tower_placed,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: NO_TRIGGERS.stage_start,
        stage_end: stage_end_hamburger,
    },
};
