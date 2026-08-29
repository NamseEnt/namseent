use super::super::UpgradeEntryState;
use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, merge_ratio_upgrade, no_cache, no_limit, ratio_half, recovery_none,
};

fn tower_bonus_no_reroll(upgrade: &UpgradeEntryState, tower: &crate::TowerState) -> i64 {
    if tower.template.rerolled_count == 0 {
        upgrade.ratio_values_raw.first().copied().unwrap_or(0)
    } else {
        0
    }
}

fn tower_template_bonus_no_reroll(
    upgrade: &UpgradeEntryState,
    template: &crate::TowerTemplateState,
) -> i64 {
    if template.rerolled_count == 0 {
        upgrade.ratio_values_raw.first().copied().unwrap_or(0)
    } else {
        0
    }
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::PerfectPottery,
    generate_payload: ratio_half,
    rarity: crate::Rarity::Common,
    cache: no_cache,
    acquire: merge_ratio_upgrade,
    recovery: recovery_none,
    tower_bonus: tower_bonus_no_reroll,
    tower_bonus_for_template: tower_template_bonus_no_reroll,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: NO_TRIGGERS.monster_death,
        gold_earned: NO_TRIGGERS.gold_earned,
        card_rerolled: NO_TRIGGERS.card_rerolled,
        shop_purchase: NO_TRIGGERS.shop_purchase,
        tower_placed: NO_TRIGGERS.tower_placed,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: NO_TRIGGERS.stage_start,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
