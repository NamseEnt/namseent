use super::*;
use crate::game_state::camera::ShakeIntensity;
use crate::{
    game_state::{
        contract::sign_contract, effect::run_effect, item, play_history::HistoryEventType,
        tower::Tower, upgrade::Upgrade,
    },
    shop::ShopSlot,
};

impl GameState {
    pub fn earn_gold(&mut self, gold: usize) {
        self.gold += gold;
    }
    /// WARNING: `gold` must be less than or equal to self.gold
    pub fn spend_gold(&mut self, gold: usize) {
        self.gold -= gold;
    }

    pub fn upgrade(&mut self, upgrade: Upgrade) {
        self.upgrade_state.upgrade(upgrade);
        self.record_event(HistoryEventType::UpgradeSelected { upgrade });
    }

    pub fn place_tower(&mut self, tower: Tower) {
        let rank = tower.rank;
        let suit = tower.suit;
        let hand = tower.kind;
        let left_top = tower.left_top;

        self.towers.place_tower(tower);
        self.route = calculate_routes(&self.towers.coords(), &TRAVEL_POINTS, MAP_SIZE).unwrap();

        self.record_event(HistoryEventType::TowerPlaced {
            tower_kind: hand,
            rank,
            suit,
            left_top,
        });
    }

    pub fn take_damage(&mut self, damage: f32) {
        let mut actual_damage = damage;

        // Camera shake based on damage
        let intensity = match actual_damage {
            d if d < 10.0 => ShakeIntensity::Light,
            d if d < 25.0 => ShakeIntensity::Medium,
            _ => ShakeIntensity::Heavy,
        };
        self.camera.shake(intensity);

        // Shield absorption
        if self.shield > 0.0 {
            let absorbed = damage.min(self.shield);
            actual_damage -= absorbed;
            self.shield -= absorbed;
        }

        // Apply damage
        self.hp -= actual_damage;

        // Record event
        if actual_damage > 0.0 {
            self.record_event(HistoryEventType::DamageTaken {
                amount: actual_damage,
            });
        }

        // Check game over
        if self.hp <= 0.0 {
            self.goto_result();
        }
    }

    pub fn purchase_shop_item(&mut self, slot_id: crate::shop::ShopSlotId) {
        let GameFlow::SelectingTower(flow) = &mut self.flow else {
            unreachable!()
        };

        let Some(slot_data) = flow.shop.get_slot_by_id_mut(slot_id) else {
            return;
        };

        if slot_data.purchased {
            return;
        }

        match &slot_data.slot {
            ShopSlot::Locked => {}
            ShopSlot::Item { item, cost } => {
                if self.gold < *cost {
                    return;
                }
                if self.items.len() >= MAX_INVENTORY_SLOT {
                    return;
                }

                // 아이템/업그레이드 구매 불가 효과 체크
                if self
                    .stage_modifiers
                    .is_item_and_upgrade_purchases_disabled()
                {
                    return; // 구매 불가 상태에서는 아무것도 하지 않음
                }

                // Store values before borrowing self mutably
                let item_clone = item.clone();
                let cost_value = *cost;

                slot_data.purchased = true;
                slot_data.start_exit_animation(Instant::now());
                self.items.push(item_clone.clone());
                self.record_event(HistoryEventType::ItemPurchased {
                    item: item_clone,
                    cost: cost_value,
                });
                self.spend_gold(cost_value);
            }
            ShopSlot::Upgrade { upgrade, cost } => {
                if self.gold < *cost {
                    return;
                }

                // 아이템/업그레이드 구매 불가 효과 체크
                if self
                    .stage_modifiers
                    .is_item_and_upgrade_purchases_disabled()
                {
                    return; // 구매 불가 상태에서는 아무것도 하지 않음
                }

                // Store values before borrowing self mutably
                let upgrade_value = *upgrade;
                let cost_value = *cost;

                slot_data.purchased = true;
                slot_data.start_exit_animation(Instant::now());
                self.upgrade_state.upgrade(upgrade_value);
                self.record_event(HistoryEventType::UpgradePurchased {
                    upgrade: upgrade_value,
                    cost: cost_value,
                });
                self.spend_gold(cost_value);
            }
            ShopSlot::Contract { contract, cost } => {
                if self.gold < *cost {
                    return;
                }

                // Store values before borrowing self mutably
                let contract_value = contract.clone();
                let cost_value = *cost;

                slot_data.purchased = true;
                slot_data.start_exit_animation(Instant::now());
                sign_contract(self, contract_value.clone());
                self.record_event(HistoryEventType::ContractPurchased {
                    contract: contract_value,
                    cost: cost_value,
                });
                self.spend_gold(cost_value);
            }
        }
    }

    pub fn use_item(&mut self, item: &item::Item) {
        // 아이템 사용 불가 효과 체크
        if self.stage_modifiers.is_item_use_disabled() {
            return; // 아이템 사용 불가 상태에서는 아무것도 하지 않음
        }

        self.item_used = true;
        run_effect(self, &item.effect);
        self.record_event(HistoryEventType::ItemUsed {
            item_effect: item.effect.clone(),
        });
    }

    pub fn can_purchase_shop_item(&self, slot_id: crate::shop::ShopSlotId) -> bool {
        let GameFlow::SelectingTower(flow) = &self.flow else {
            return false;
        };

        let Some(slot_data) = flow.shop.get_slot_by_id(slot_id) else {
            return false;
        };

        if slot_data.purchased {
            return false;
        }

        match &slot_data.slot {
            ShopSlot::Locked => false,
            ShopSlot::Item { cost, .. } => {
                self.gold >= *cost
                    && self.items.len() < MAX_INVENTORY_SLOT
                    && !self
                        .stage_modifiers
                        .is_item_and_upgrade_purchases_disabled()
            }
            ShopSlot::Upgrade { cost, .. } => {
                self.gold >= *cost
                    && !self
                        .stage_modifiers
                        .is_item_and_upgrade_purchases_disabled()
            }
            ShopSlot::Contract { cost, .. } => self.gold >= *cost,
        }
    }
}
