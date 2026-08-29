use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, bool_true, no_cache, no_limit, push_acquired_upgrade, recovery_none, set_bool,
    tower_bonus_none, tower_template_bonus_none,
};

fn stage_start_free_shop(core: &mut crate::CoreState, index: usize, _: usize) -> bool {
    if core.upgrades.upgrades[index]
        .bool_values
        .first()
        .copied()
        .unwrap_or(false)
    {
        core.stage_modifiers.free_shop_this_stage = true;
        return set_bool(&mut core.upgrades.upgrades[index], 0, false);
    }
    false
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::MembershipCard,
    generate_payload: bool_true,
    rarity: crate::Rarity::Rare,
    cache: no_cache,
    acquire: push_acquired_upgrade,
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
        stage_start: stage_start_free_shop,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
