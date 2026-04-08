//! Shop strategies.

use super::ShopStrategy;
use crate::game_state::GameState;
use crate::game_state::flow::GameFlow;
use crate::game_state::item::ItemKind;
use crate::game_state::tower::Tower;
use crate::game_state::upgrade::{Upgrade, UpgradeState};
use rand::RngCore;

/// Buys the cheapest affordable item from the shop.
pub struct BuyCheapestStrategy;

impl ShopStrategy for BuyCheapestStrategy {
    fn name(&self) -> &str {
        "buy_cheapest"
    }

    fn execute_shop(&self, game_state: &mut GameState, _rng: &mut dyn RngCore) {
        loop {
            let cheapest_slot_id = {
                let GameFlow::SelectingTower(flow) = &game_state.flow else {
                    return;
                };

                let mut cheapest: Option<(crate::shop::ShopSlotId, usize)> = None;
                for slot in &flow.shop.slots {
                    if slot.purchased || slot.exit_animation.is_some() {
                        continue;
                    }
                    let cost = match &slot.slot {
                        crate::shop::ShopSlot::Item { cost, .. } => *cost,
                        crate::shop::ShopSlot::Upgrade { cost, .. } => *cost,
                    };
                    if cost <= game_state.gold && (cheapest.is_none() || cost < cheapest.unwrap().1)
                    {
                        cheapest = Some((slot.id, cost));
                    }
                }
                cheapest.map(|(id, _)| id)
            };

            match cheapest_slot_id {
                Some(slot_id) => game_state.purchase_shop_item(slot_id),
                None => return,
            }
        }
    }
}

/// Heuristic shop strategy that buys essential items and evaluates upgrades by placed tower contribution.
pub struct HeuristicShopStrategy;

impl ShopStrategy for HeuristicShopStrategy {
    fn name(&self) -> &str {
        "heuristic_shop"
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

impl HeuristicShopStrategy {
    fn buy_priority_item(&self, game_state: &mut GameState) -> bool {
        let GameFlow::SelectingTower(flow) = &game_state.flow else {
            return false;
        };

        if count_item_kind(game_state, ItemKind::RiceCake) < 1 {
            if let Some(slot_id) = find_item_slot(flow, ItemKind::RiceCake, game_state.gold) {
                game_state.purchase_shop_item(slot_id);
                return true;
            }
        }

        if count_item_kind(game_state, ItemKind::Shield) < 1 {
            if let Some(slot_id) = find_item_slot(flow, ItemKind::Shield, game_state.gold) {
                game_state.purchase_shop_item(slot_id);
                return true;
            }
        }

        if let Some(slot_id) = find_item_slot(flow, ItemKind::GrantBarricades, game_state.gold) {
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

    fn evaluate_item_slot(
        &self,
        game_state: &GameState,
        item: &crate::game_state::item::Item,
    ) -> f32 {
        match item.kind {
            ItemKind::GrantBarricades => 8.0,
            ItemKind::RiceCake => {
                if game_state.hp < game_state.config.player.max_hp * 0.75 {
                    6.0
                } else {
                    3.0
                }
            }
            ItemKind::Shield => {
                if game_state.shield <= 0.0 {
                    5.0
                } else {
                    2.0
                }
            }
            ItemKind::EmergencyDice => 2.5,
            ItemKind::Painkiller => 2.0,
            ItemKind::GrantCard { .. } => 1.5,
        }
    }

    fn evaluate_upgrade_slot(&self, game_state: &GameState, upgrade: Upgrade) -> f32 {
        let current_score = total_tower_score(game_state, &game_state.upgrade_state);
        let mut upgraded_state = game_state.upgrade_state.clone();
        upgraded_state.upgrade(upgrade);
        let next_score = total_tower_score(game_state, &upgraded_state);
        let delta = next_score - current_score;
        if delta > 0.0 {
            delta
        } else {
            self.heuristic_upgrade_value(upgrade.kind)
        }
    }

    fn heuristic_upgrade_value(&self, kind: crate::game_state::upgrade::UpgradeKind) -> f32 {
        match kind {
            crate::game_state::upgrade::UpgradeKind::Magnet => 2.0,
            crate::game_state::upgrade::UpgradeKind::Backpack => 2.0,
            crate::game_state::upgrade::UpgradeKind::DiceBundle => 2.5,
            crate::game_state::upgrade::UpgradeKind::EnergyDrink => 1.5,
            crate::game_state::upgrade::UpgradeKind::FourLeafClover
            | crate::game_state::upgrade::UpgradeKind::Rabbit
            | crate::game_state::upgrade::UpgradeKind::BlackWhite
            | crate::game_state::upgrade::UpgradeKind::Eraser => 1.0,
            crate::game_state::upgrade::UpgradeKind::Spoon { .. }
            | crate::game_state::upgrade::UpgradeKind::PerfectPottery { .. }
            | crate::game_state::upgrade::UpgradeKind::BrokenPottery { .. } => 0.5,
            _ => 0.5,
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

        if let crate::shop::ShopSlot::Item { item, cost } = &slot.slot {
            if item.kind == kind && *cost <= max_cost {
                if best.is_none() || *cost < best.unwrap().1 {
                    best = Some((slot.id, *cost));
                }
            }
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
