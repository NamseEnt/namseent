use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, empty_payload, no_cache, no_limit, push_acquired_upgrade, recovery_none,
    tower_bonus_none, tower_template_bonus_none,
};

const BROKEN_POTTERY_REROLL_INTERVAL: usize = 4;

fn card_rerolled_broken_pottery(core: &mut crate::CoreState, _: usize) -> bool {
    if core.progress.rerolled_count > 0
        && core
            .progress
            .rerolled_count
            .is_multiple_of(BROKEN_POTTERY_REROLL_INTERVAL)
    {
        core.progress.left_dice = core.progress.left_dice.saturating_add(1);
    }
    false
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::BrokenPottery,
    generate_payload: empty_payload,
    rarity: crate::Rarity::Legendary,
    cache: no_cache,
    acquire: push_acquired_upgrade,
    recovery: recovery_none,
    tower_bonus: tower_bonus_none,
    tower_bonus_for_template: tower_template_bonus_none,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: NO_TRIGGERS.monster_death,
        gold_earned: NO_TRIGGERS.gold_earned,
        card_rerolled: card_rerolled_broken_pottery,
        shop_purchase: NO_TRIGGERS.shop_purchase,
        tower_placed: NO_TRIGGERS.tower_placed,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: NO_TRIGGERS.stage_start,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
