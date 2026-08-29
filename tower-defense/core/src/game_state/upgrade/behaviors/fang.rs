use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, no_cache, no_limit, push_acquired_upgrade, recovery_none, scalar, scalar_ten,
    tower_bonus_none, tower_template_bonus_none,
};

fn monster_death_heal(core: &crate::CoreState, index: usize, _: &mut usize, healing: &mut i64) {
    let amount = scalar(&core.upgrades.upgrades[index], 0);
    *healing = healing.saturating_add((amount.min(i64::MAX as usize) as i64).saturating_mul(1_000));
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::Fang,
    generate_payload: scalar_ten,
    rarity: crate::Rarity::Common,
    cache: no_cache,
    acquire: push_acquired_upgrade,
    recovery: recovery_none,
    tower_bonus: tower_bonus_none,
    tower_bonus_for_template: tower_template_bonus_none,
    current_and_max: no_limit,
    triggers: UpgradeTriggerDefinition {
        monster_death: monster_death_heal,
        gold_earned: NO_TRIGGERS.gold_earned,
        card_rerolled: NO_TRIGGERS.card_rerolled,
        shop_purchase: NO_TRIGGERS.shop_purchase,
        tower_placed: NO_TRIGGERS.tower_placed,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: NO_TRIGGERS.stage_start,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
