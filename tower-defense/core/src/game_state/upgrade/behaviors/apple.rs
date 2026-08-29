use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::super::{UpgradeCacheContribution, UpgradeEntryState};
use super::support::{
    NO_TRIGGERS, cache_hp, empty_payload, no_limit, push_acquired_upgrade, recovery_amount,
    tower_bonus_none, tower_template_bonus_none,
};

const APPLE_HEAL_AMOUNT_RAW: i64 = 6_000;

fn cache_apple(_: &UpgradeEntryState) -> UpgradeCacheContribution {
    cache_hp(4_000)
}

fn recovery_apple() -> super::super::UpgradeAcquireRecovery {
    recovery_amount(APPLE_HEAL_AMOUNT_RAW)
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::Apple,
    generate_payload: empty_payload,
    rarity: crate::Rarity::Common,
    cache: cache_apple,
    acquire: push_acquired_upgrade,
    recovery: recovery_apple,
    tower_bonus: tower_bonus_none,
    tower_bonus_for_template: tower_template_bonus_none,
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
