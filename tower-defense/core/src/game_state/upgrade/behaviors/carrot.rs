use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::super::{UpgradeCacheContribution, UpgradeEntryState};
use super::support::{
    NO_TRIGGERS, cache_hp, empty_payload, no_limit, push_acquired_upgrade, tower_bonus_none,
    tower_template_bonus_none,
};

fn cache_carrot(_: &UpgradeEntryState) -> UpgradeCacheContribution {
    cache_hp(6_000)
}

fn recovery_full() -> super::super::UpgradeAcquireRecovery {
    super::super::UpgradeAcquireRecovery::ToFull
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::Carrot,
    generate_payload: empty_payload,
    rarity: crate::Rarity::Legendary,
    cache: cache_carrot,
    acquire: push_acquired_upgrade,
    recovery: recovery_full,
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
