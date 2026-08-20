//! Shop strategies.

use super::ShopStrategy;
use crate::game_state::flow::GameFlow;
use crate::game_state::item::ItemDiscriminants;
use crate::game_state::upgrade::Upgrade;
use crate::game_state::{GameState, PlayerCommand};
use rand::RngCore;

/// Synergy-aware shop strategy that values upgrades and items based on current economy, tower build, and future selection needs.
pub struct SynergyShopStrategy;

impl ShopStrategy for SynergyShopStrategy {
    fn name(&self) -> &str {
        "synergy_shop"
    }

    fn execute_shop(&self, game_state: &mut GameState, _rng: &mut dyn RngCore) {
        loop {
            if self.buy_priority_item(game_state) {
                continue;
            }

            if let Some(slot_id) = self.choose_best_slot(game_state, _rng) {
                if !purchase_slot(game_state, slot_id) {
                    break;
                }
                continue;
            }

            break;
        }
    }
}

impl SynergyShopStrategy {
    fn buy_priority_item(&self, game_state: &mut GameState) -> bool {
        if !matches!(game_state.flow, GameFlow::Shopping(_)) {
            return false;
        }

        if count_item_kind(game_state, ItemDiscriminants::RiceBall) < 1
            && game_state.hp
                < game_state
                    .config
                    .player
                    .max_hp
                    .scaled_by(crate::FixedRatio::from_raw(750_000))
            && let Some(slot_id) = find_item_slot(game_state, ItemDiscriminants::RiceBall)
        {
            return purchase_slot(game_state, slot_id);
        }

        if count_item_kind(game_state, ItemDiscriminants::Milk) < 1
            && game_state.shield.is_zero()
            && game_state.hp
                < game_state
                    .config
                    .player
                    .max_hp
                    .scaled_by(crate::FixedRatio::from_raw(850_000))
            && let Some(slot_id) = find_item_slot(game_state, ItemDiscriminants::Milk)
        {
            return purchase_slot(game_state, slot_id);
        }

        if count_item_kind(game_state, ItemDiscriminants::RubberCone) < 1
            && let Some(slot_id) = find_item_slot(game_state, ItemDiscriminants::RubberCone)
        {
            return purchase_slot(game_state, slot_id);
        }

        if game_state.left_dice < game_state.max_dice_chance().saturating_sub(1)
            && let Some(slot_id) = find_item_slot(game_state, ItemDiscriminants::LumpSugar)
        {
            return purchase_slot(game_state, slot_id);
        }

        false
    }

    fn choose_best_slot(
        &self,
        game_state: &GameState,
        rng: &mut dyn RngCore,
    ) -> Option<crate::shop::ShopSlotId> {
        let GameFlow::Shopping(flow) = &game_state.flow else {
            return None;
        };

        let mut total_weight = 0u32;
        let mut weighted_slots: Vec<(crate::shop::ShopSlotId, u32)> = Vec::new();

        for slot in &flow.shop.slots {
            if slot.purchased
                || slot.exit_animation.is_some()
                || !game_state.shop_purchase_status(slot.id).is_available()
            {
                continue;
            }
            let weight = match &slot.slot {
                crate::shop::ShopSlot::Item { .. } => Self::rarity_weight_common(),
                crate::shop::ShopSlot::Upgrade { upgrade, .. } => {
                    Self::rarity_weight_upgrade(*upgrade)
                }
                crate::shop::ShopSlot::CardService { .. } => Self::rarity_weight_common(),
            };
            if weight == 0 {
                continue;
            }

            total_weight += weight;
            weighted_slots.push((slot.id, weight));
        }

        if total_weight == 0 {
            return None;
        }

        let mut choice = (rng.next_u64() % total_weight as u64) as u32;
        for (slot_id, weight) in &weighted_slots {
            if choice < *weight {
                return Some(*slot_id);
            }
            choice -= *weight;
        }

        weighted_slots.last().map(|(slot_id, _)| *slot_id)
    }

    fn rarity_weight_common() -> u32 {
        5
    }

    fn rarity_weight_upgrade(upgrade: Upgrade) -> u32 {
        match upgrade.discriminant().rarity() {
            crate::Rarity::Legendary => 50,
            crate::Rarity::Epic => 25,
            crate::Rarity::Rare => 10,
            crate::Rarity::Common => 5,
        }
    }
}

fn purchase_slot(game_state: &mut GameState, slot_id: crate::shop::ShopSlotId) -> bool {
    let Some(slot_index) = (match &game_state.flow {
        GameFlow::Shopping(flow) => flow
            .shop
            .slots
            .iter()
            .filter(|slot| slot.exit_animation.is_none())
            .position(|slot| slot.id == slot_id),
        _ => None,
    }) else {
        return false;
    };

    game_state
        .apply_player_command(PlayerCommand::PurchaseShopItem { slot_index })
        .is_ok()
}

fn find_item_slot(
    game_state: &GameState,
    kind: ItemDiscriminants,
) -> Option<crate::shop::ShopSlotId> {
    let GameFlow::Shopping(flow) = &game_state.flow else {
        return None;
    };

    let mut best: Option<(crate::shop::ShopSlotId, usize)> = None;
    for slot in &flow.shop.slots {
        if slot.purchased
            || slot.exit_animation.is_some()
            || !game_state.shop_purchase_status(slot.id).is_available()
        {
            continue;
        }

        if let crate::shop::ShopSlot::Item { item, cost } = &slot.slot
            && item.discriminant() == kind
            && (best.is_none() || *cost < best.unwrap().1)
        {
            best = Some((slot.id, *cost));
        }
    }
    best.map(|(id, _)| id)
}

fn count_item_kind(game_state: &GameState, kind: ItemDiscriminants) -> usize {
    game_state
        .items
        .iter()
        .filter(|item| item.discriminant() == kind)
        .count()
}
