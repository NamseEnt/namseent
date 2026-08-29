use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::super::{UpgradeCacheContribution, UpgradeEntryState};
use super::support::{
    NO_TRIGGERS, base_cache, merge_scalar_upgrade, recovery_none, scalar, scalar_one,
    tower_bonus_none, tower_template_bonus_none,
};

const MAX_SHOP_SLOT_EXPAND: usize = 2;

fn max_shop_slots(core: &crate::CoreState) -> Option<(usize, usize)> {
    Some((
        core.upgrades
            .upgrades
            .iter()
            .filter(|u| {
                u.upgrade_kind()
                    .is_ok_and(|kind| kind == crate::UpgradeKind::Backpack)
            })
            .map(|u| scalar(u, 0))
            .sum(),
        MAX_SHOP_SLOT_EXPAND,
    ))
}

fn acquire_shop_slots(core: &mut crate::CoreState, upgrade: UpgradeEntryState) -> usize {
    let add = scalar(&upgrade, 0);
    if add > 0 {
        crate::game_state::shop::add_shop_slots(core, add);
    }
    merge_scalar_upgrade(core, upgrade);
    add
}

fn cache_shop_slots(upgrade: &UpgradeEntryState) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        shop_slot_expand: scalar(upgrade, 0),
        ..base_cache()
    }
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::Backpack,
    generate_payload: scalar_one,
    rarity: crate::Rarity::Common,
    cache: cache_shop_slots,
    acquire: acquire_shop_slots,
    recovery: recovery_none,
    tower_bonus: tower_bonus_none,
    tower_bonus_for_template: tower_template_bonus_none,
    current_and_max: max_shop_slots,
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
