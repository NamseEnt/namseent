use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, merge_scalar_upgrade, no_cache, no_limit, recovery_none, scalar_ten,
    tower_bonus_none, tower_template_bonus_none,
};

const PIGGY_BANK_GOLD_REWARD_PER_STEP: usize = 10;

const PIGGY_BANK_GOLD_STEP: usize = 100;

fn stage_end_piggy_bank(
    _: &mut crate::CoreState,
    _: usize,
    _: bool,
    gold: usize,
    _: usize,
) -> (bool, usize) {
    (
        false,
        gold / PIGGY_BANK_GOLD_STEP * PIGGY_BANK_GOLD_REWARD_PER_STEP,
    )
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::PiggyBank,
    generate_payload: scalar_ten,
    rarity: crate::Rarity::Rare,
    cache: no_cache,
    acquire: merge_scalar_upgrade,
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
        stage_end: stage_end_piggy_bank,
    },
};
