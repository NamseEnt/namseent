use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::support::{
    NO_TRIGGERS, merge_scalar_upgrade, no_cache, no_limit, recovery_none, scalar, scalar_ten,
    set_scalar, tower_bonus_none, tower_template_bonus_none,
};

fn stage_start_piggy_bank(core: &mut crate::CoreState, index: usize, _: usize) -> bool {
    let dice = scalar(&core.upgrades.upgrades[index], 0);
    if dice > 0 {
        core.progress.left_dice = core.progress.left_dice.saturating_add(dice);
        return set_scalar(&mut core.upgrades.upgrades[index], 0, 0);
    }
    false
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::SlotMachine,
    generate_payload: scalar_ten,
    rarity: crate::Rarity::Epic,
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
        stage_start: stage_start_piggy_bank,
        stage_end: NO_TRIGGERS.stage_end,
    },
};
