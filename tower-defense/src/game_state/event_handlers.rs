use super::*;
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

    pub fn purchase_shop_item(&mut self, slot_index: usize) {
        let GameFlow::SelectingTower(flow) = &mut self.flow else {
            unreachable!()
        };

        let Some(slot) = flow.shop.slots.get_mut(slot_index) else {
            return;
        };

        match slot {
            ShopSlot::Locked => {}
            ShopSlot::Item {
                item,
                cost,
                purchased,
            } => {
                if *purchased {
                    return;
                }
                if self.gold < *cost {
                    return;
                }
                if self.items.len() >= MAX_INVENTORY_SLOT {
                    return;
                }

                // Store values before borrowing self mutably
                let item_clone = item.clone();
                let cost_value = *cost;

                *purchased = true;
                self.items.push(item_clone.clone());
                self.record_event(HistoryEventType::ItemPurchased {
                    item: item_clone,
                    cost: cost_value,
                });
                self.spend_gold(cost_value);
            }
            ShopSlot::Upgrade {
                upgrade,
                cost,
                purchased,
            } => {
                if *purchased {
                    return;
                }
                if self.gold < *cost {
                    return;
                }

                // Store values before borrowing self mutably
                let upgrade_value = *upgrade;
                let cost_value = *cost;

                *purchased = true;
                self.upgrade_state.upgrade(upgrade_value);
                self.record_event(HistoryEventType::UpgradePurchased {
                    upgrade: upgrade_value,
                    cost: cost_value,
                });
                self.spend_gold(cost_value);
            }
            ShopSlot::Contract {
                contract,
                cost,
                purchased,
            } => {
                if *purchased {
                    return;
                }
                if self.gold < *cost {
                    return;
                }

                // Store values before borrowing self mutably
                let contract_value = contract.clone();
                let cost_value = *cost;

                *purchased = true;
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
        self.item_used = true;
        run_effect(self, &item.effect);
        self.record_event(HistoryEventType::ItemUsed {
            item_effect: item.effect.clone(),
        });
    }
}
