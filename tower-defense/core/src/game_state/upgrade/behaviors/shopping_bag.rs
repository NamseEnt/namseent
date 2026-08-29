use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, empty_payload, no_cache, no_limit, push_acquired_upgrade, recovery_none,
    tower_bonus_none, tower_template_bonus_none,
};

fn shop_purchase_dice(core: &mut crate::CoreState, _: usize, item_purchase: bool) -> bool {
    if item_purchase {
        core.progress.left_dice = core.progress.left_dice.saturating_add(1);
    }
    false
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::ShoppingBag,
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
        card_rerolled: NO_TRIGGERS.card_rerolled,
        shop_purchase: shop_purchase_dice,
        tower_placed: NO_TRIGGERS.tower_placed,
        tower_removed: NO_TRIGGERS.tower_removed,
        stage_start: NO_TRIGGERS.stage_start,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
