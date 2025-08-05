use super::*;
use crate::{
    MapCoordF32,
    game_state::{
        item,
        play_history::HistoryEventType,
        quest::{QuestRequirement, QuestReward, QuestTriggerEvent, on_quest_trigger_event},
        tower::Tower,
        upgrade::Upgrade,
    },
};

impl GameState {
    pub fn earn_gold(&mut self, gold: usize) {
        self.gold += gold;
        on_quest_trigger_event(self, QuestTriggerEvent::EarnGold { gold });
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

        on_quest_trigger_event(self, QuestTriggerEvent::BuildTower { rank, suit, hand });
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

    pub fn purchase_item(&mut self, item: item::Item, cost: usize) {
        self.items.push(item.clone());
        self.gold -= cost;

        self.record_event(HistoryEventType::ItemPurchased { item, cost });
        on_quest_trigger_event(self, QuestTriggerEvent::SpendGold { gold: cost });
    }

    pub fn use_item(&mut self, item: &item::Item, xy: Option<MapCoordF32>) {
        self.item_used = true;
        let effect_kind = item.kind.effect_kind(xy, self.now());
        item::effect_processor::process_item_effect(self, effect_kind);

        self.record_event(HistoryEventType::ItemUsed {
            item_kind: item.kind,
        });

        on_quest_trigger_event(self, QuestTriggerEvent::UseItem);
    }

    pub fn complete_quest(&mut self, requirement: QuestRequirement, reward: QuestReward) {
        self.record_event(HistoryEventType::QuestCompleted {
            requirement,
            reward: reward.clone(),
        });

        match reward {
            QuestReward::Money { amount } => {
                self.earn_gold(amount);
            }
        }
    }
}
