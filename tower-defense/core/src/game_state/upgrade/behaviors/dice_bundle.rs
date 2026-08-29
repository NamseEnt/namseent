use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::super::{UpgradeCacheContribution, UpgradeEntryState};
use super::support::{
    NO_TRIGGERS, base_cache, merge_scalar_upgrade, no_limit, recovery_none, scalar, scalar_one,
    tower_bonus_none, tower_template_bonus_none,
};

fn acquire_dice(core: &mut crate::CoreState, upgrade: UpgradeEntryState) -> usize {
    core.progress.left_dice = core.progress.left_dice.saturating_add(scalar(&upgrade, 0));
    merge_scalar_upgrade(core, upgrade);
    0
}

fn cache_dice(upgrade: &UpgradeEntryState) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        dice_chance_plus: scalar(upgrade, 0),
        ..base_cache()
    }
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::DiceBundle,
    generate_payload: scalar_one,
    rarity: crate::Rarity::Rare,
    cache: cache_dice,
    acquire: acquire_dice,
    recovery: recovery_none,
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
