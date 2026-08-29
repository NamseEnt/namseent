use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::super::{UpgradeCacheContribution, UpgradeEntryState};
use super::support::{
    NO_TRIGGERS, base_cache, empty_payload, push_acquired_upgrade, recovery_none, tower_bonus_none,
    tower_template_bonus_none,
};

fn max_straight_flush(core: &crate::CoreState) -> Option<(usize, usize)> {
    Some((
        usize::from(core.upgrades.upgrades.iter().any(|u| {
            u.upgrade_kind()
                .is_ok_and(|kind| kind == crate::UpgradeKind::FourLeafClover)
        })),
        1,
    ))
}

fn cache_straight_flush(_: &UpgradeEntryState) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        shorten_straight_flush_to_4_cards: true,
        ..base_cache()
    }
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::FourLeafClover,
    generate_payload: empty_payload,
    rarity: crate::Rarity::Rare,
    cache: cache_straight_flush,
    acquire: push_acquired_upgrade,
    recovery: recovery_none,
    tower_bonus: tower_bonus_none,
    tower_bonus_for_template: tower_template_bonus_none,
    current_and_max: max_straight_flush,
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
