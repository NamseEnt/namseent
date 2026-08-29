use super::super::definition::{UpgradeDefinition, UpgradeTriggerDefinition};
use super::super::{UpgradeCacheContribution, UpgradeEntryState};
use super::support::{
    NO_TRIGGERS, base_cache, merge_scalar_upgrade, recovery_none, scalar, scalar_five,
    tower_bonus_none, tower_template_bonus_none,
};

const MAX_SHOP_ITEM_PRICE_MINUS: usize = 15;

fn max_shop_discount(core: &crate::CoreState) -> Option<(usize, usize)> {
    Some((
        core.upgrades
            .upgrades
            .iter()
            .filter(|u| {
                u.upgrade_kind()
                    .is_ok_and(|kind| kind == crate::UpgradeKind::EnergyDrink)
            })
            .map(|u| scalar(u, 0))
            .sum(),
        MAX_SHOP_ITEM_PRICE_MINUS,
    ))
}

fn acquire_discount(core: &mut crate::CoreState, upgrade: UpgradeEntryState) -> usize {
    let discount = scalar(&upgrade, 0);
    if let crate::GameFlowState::Shopping(shop) = &mut core.flow {
        for slot in &mut shop.slots {
            let cost = match &mut slot.slot {
                crate::ShopSlotState::Item { cost, .. }
                | crate::ShopSlotState::Upgrade { cost, .. }
                | crate::ShopSlotState::CardService { cost, .. } => cost,
            };
            *cost = cost.saturating_sub(discount);
        }
    }
    merge_scalar_upgrade(core, upgrade);
    0
}

fn cache_discount(upgrade: &UpgradeEntryState) -> UpgradeCacheContribution {
    UpgradeCacheContribution {
        shop_item_price_minus: scalar(upgrade, 0),
        ..base_cache()
    }
}

pub(crate) const DEFINITION: UpgradeDefinition = UpgradeDefinition {
    kind: crate::UpgradeKind::EnergyDrink,
    generate_payload: scalar_five,
    rarity: crate::Rarity::Common,
    cache: cache_discount,
    acquire: acquire_discount,
    recovery: recovery_none,
    tower_bonus: tower_bonus_none,
    tower_bonus_for_template: tower_template_bonus_none,
    current_and_max: max_shop_discount,
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
