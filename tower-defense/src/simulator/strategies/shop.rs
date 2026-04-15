//! Shop strategies.

use super::ShopStrategy;
use crate::game_state::GameState;
use crate::game_state::flow::GameFlow;
use crate::game_state::item::{Item, ItemKind};
use crate::game_state::tower::Tower;
use crate::game_state::upgrade::{Upgrade, UpgradeKind, UpgradeState};
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

            if let Some(slot_id) = self.choose_best_slot(game_state) {
                game_state.purchase_shop_item(slot_id);
                continue;
            }

            break;
        }
    }
}

impl SynergyShopStrategy {
    fn buy_priority_item(&self, game_state: &mut GameState) -> bool {
        let GameFlow::SelectingTower(flow) = &game_state.flow else {
            return false;
        };

        if count_item_kind(game_state, ItemKind::RiceBall) < 1
            && game_state.hp < game_state.config.player.max_hp * 0.75
            && let Some(slot_id) = find_item_slot(flow, ItemKind::RiceBall, game_state.gold)
        {
            game_state.purchase_shop_item(slot_id);
            return true;
        }

        if count_item_kind(game_state, ItemKind::Shield) < 1
            && game_state.shield <= 0.0
            && game_state.hp < game_state.config.player.max_hp * 0.85
            && let Some(slot_id) = find_item_slot(flow, ItemKind::Shield, game_state.gold)
        {
            game_state.purchase_shop_item(slot_id);
            return true;
        }

        if count_item_kind(game_state, ItemKind::GrantBarricades) < 1
            && let Some(slot_id) = find_item_slot(flow, ItemKind::GrantBarricades, game_state.gold)
        {
            game_state.purchase_shop_item(slot_id);
            return true;
        }

        if game_state.left_dice < game_state.max_dice_chance().saturating_sub(1)
            && let Some(slot_id) = find_item_slot(flow, ItemKind::LumpSugar, game_state.gold)
        {
            game_state.purchase_shop_item(slot_id);
            return true;
        }

        false
    }

    fn choose_best_slot(&self, game_state: &GameState) -> Option<crate::shop::ShopSlotId> {
        let GameFlow::SelectingTower(flow) = &game_state.flow else {
            return None;
        };

        let mut best_score = 0.0;
        let mut best_slot = None;

        for slot in &flow.shop.slots {
            if slot.purchased || slot.exit_animation.is_some() {
                continue;
            }
            let cost = match &slot.slot {
                crate::shop::ShopSlot::Item { cost, .. } => *cost,
                crate::shop::ShopSlot::Upgrade { cost, .. } => *cost,
            };
            if cost > game_state.gold {
                continue;
            }

            let value = match &slot.slot {
                crate::shop::ShopSlot::Item { item, .. } => {
                    self.evaluate_item_slot(game_state, item)
                }
                crate::shop::ShopSlot::Upgrade { upgrade, .. } => {
                    self.evaluate_upgrade_slot(game_state, *upgrade)
                }
            };

            if value <= 0.0 {
                continue;
            }

            let score = value / (cost as f32).max(1.0);
            if score > best_score {
                best_score = score;
                best_slot = Some(slot.id);
            }
        }

        best_slot
    }

    fn evaluate_item_slot(&self, game_state: &GameState, item: &Item) -> f32 {
        match item.kind {
            ItemKind::GrantBarricades => {
                if game_state.towers.iter().count() < 3
                    || game_state.hp < game_state.config.player.max_hp * 0.85
                {
                    7.0
                } else {
                    4.0
                }
            }
            ItemKind::RiceBall => {
                if game_state.hp < game_state.config.player.max_hp * 0.6 {
                    6.0
                } else {
                    2.5
                }
            }
            ItemKind::Shield => {
                if game_state.shield <= 0.0
                    && game_state.hp < game_state.config.player.max_hp * 0.85
                {
                    5.5
                } else {
                    2.0
                }
            }
            ItemKind::LumpSugar => {
                let missing_dice = game_state
                    .max_dice_chance()
                    .saturating_sub(game_state.left_dice) as f32;
                3.0 + missing_dice * 1.5
            }
            ItemKind::Painkiller => {
                if game_state.hp < game_state.config.player.max_hp * 0.7 {
                    4.0
                } else {
                    2.0
                }
            }
            ItemKind::GrantCard { .. } => {
                let hand_count = game_state.hand.active_slot_ids().len() as f32;
                if hand_count <= 2.0 {
                    6.0
                } else if hand_count <= 4.0 {
                    3.5
                } else {
                    1.5
                }
            }
        }
    }

    fn evaluate_upgrade_slot(&self, game_state: &GameState, upgrade: Upgrade) -> f32 {
        if upgrade.kind.is_tower_damage_upgrade() {
            let current_score = total_tower_score(game_state, &game_state.upgrade_state);
            let mut upgraded_state = game_state.upgrade_state.clone();
            upgraded_state.upgrade(upgrade);
            let next_score = total_tower_score(game_state, &upgraded_state);
            let delta = next_score - current_score;
            return delta
                .max(0.0)
                .max(self.evaluate_treasure_upgrade(upgrade.kind));
        }

        self.evaluate_treasure_upgrade(upgrade.kind)
    }

    fn evaluate_treasure_upgrade(&self, kind: UpgradeKind) -> f32 {
        match kind {
            UpgradeKind::Cat => 7.0,
            UpgradeKind::Backpack => 6.5,
            UpgradeKind::DiceBundle => 7.5,
            UpgradeKind::EnergyDrink => 6.0,
            UpgradeKind::FourLeafClover => 5.0,
            UpgradeKind::Rabbit => 5.0,
            UpgradeKind::BlackWhite => 5.5,
            UpgradeKind::Eraser => 6.0,
            _ => 3.0,
        }
    }
}

fn find_item_slot(
    flow: &crate::game_state::flow::SelectingTowerFlow,
    kind: ItemKind,
    max_cost: usize,
) -> Option<crate::shop::ShopSlotId> {
    let mut best: Option<(crate::shop::ShopSlotId, usize)> = None;
    for slot in &flow.shop.slots {
        if slot.purchased || slot.exit_animation.is_some() {
            continue;
        }

        if let crate::shop::ShopSlot::Item { item, cost } = &slot.slot
            && item.kind == kind
            && *cost <= max_cost
            && (best.is_none() || *cost < best.unwrap().1)
        {
            best = Some((slot.id, *cost));
        }
    }
    best.map(|(id, _)| id)
}

fn count_item_kind(game_state: &GameState, kind: ItemKind) -> usize {
    game_state
        .items
        .iter()
        .filter(|item| item.kind == kind)
        .count()
}

fn total_tower_score(game_state: &GameState, upgrade_state: &UpgradeState) -> f32 {
    game_state
        .towers
        .iter()
        .map(|tower| tower_score(tower, upgrade_state))
        .sum()
}

fn tower_score(tower: &Tower, upgrade_state: &UpgradeState) -> f32 {
    let tower_upgrade_states = upgrade_state.tower_upgrades(tower);
    let damage = tower.calculate_projectile_damage(&tower_upgrade_states, 1.0);
    if damage <= 0.0 {
        return 0.0;
    }
    let interval = tower.shoot_interval.as_secs_f32().max(0.001);
    let dps = damage / interval;
    let range = tower.attack_range_radius(&tower_upgrade_states, 1.0);
    let range_factor = (range / 4.0).max(0.5);
    dps * range_factor
}
