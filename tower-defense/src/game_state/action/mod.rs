mod earn_gold;
mod place_tower;
mod purchase_shop_item;
mod remove_tower;
mod spend_gold;
mod take_damage;
mod upgrade;
mod upgrade_trigger;
mod use_item;

use earn_gold::earn_gold;
use place_tower::place_tower;
use purchase_shop_item::purchase_shop_item;
use remove_tower::remove_tower;
use spend_gold::spend_gold;
use take_damage::take_damage;
use use_item::use_item;

use crate::game_state::{
    GameState, action::upgrade_trigger::UpgradeTriggerEvent, item, tower::Tower, upgrade::Upgrade,
};

pub(crate) enum GameStateAction<'a> {
    GameStart,
    StageStart {
        stage: usize,
    },
    EarnGold(usize),
    SpendGold(usize),
    Upgrade(Upgrade, Option<usize>),
    PlaceTower(Box<Tower>),
    RemoveTower(usize),
    PurchaseShopItem(crate::shop::ShopSlotId),
    UseItem(&'a item::Item),
    TakeDamage(f32),
    StageEnd {
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    },
    GameOver,
}

impl GameState {
    pub(crate) fn action(&mut self, action: GameStateAction<'_>) -> bool {
        match action {
            GameStateAction::GameStart => {
                self.record_event(crate::game_state::play_history::HistoryEventType::GameStart);
                true
            }
            GameStateAction::StageStart { stage } => {
                self.left_dice = self.max_dice_chance();
                self.record_event(
                    crate::game_state::play_history::HistoryEventType::StageStart {
                        stage,
                        boss: crate::game_state::is_boss_stage(stage),
                    },
                );
                self.handle_upgrade_trigger(UpgradeTriggerEvent::StageStart { stage });
                true
            }
            GameStateAction::EarnGold(amount) => {
                earn_gold(self, amount);
                true
            }
            GameStateAction::SpendGold(amount) => {
                spend_gold(self, amount);
                true
            }
            GameStateAction::Upgrade(upgrade, cost) => {
                upgrade::upgrade(self, upgrade, cost);
                true
            }
            GameStateAction::PlaceTower(tower) => {
                place_tower(self, tower);
                true
            }
            GameStateAction::RemoveTower(tower_id) => remove_tower(self, tower_id),
            GameStateAction::PurchaseShopItem(slot_id) => {
                purchase_shop_item(self, slot_id);
                true
            }
            GameStateAction::UseItem(item) => {
                use_item(self, item);
                true
            }
            GameStateAction::TakeDamage(damage) => {
                take_damage(self, damage);
                true
            }
            GameStateAction::StageEnd {
                perfect_clear,
                gold,
                item_count,
            } => {
                if perfect_clear {
                    self.record_event(
                        crate::game_state::play_history::HistoryEventType::StagePerfectClear {
                            stage: self.stage,
                        },
                    );
                    self.metrics.current_consecutive_perfect_clears += 1;
                    self.metrics.max_consecutive_perfect_clears = self
                        .metrics
                        .max_consecutive_perfect_clears
                        .max(self.metrics.current_consecutive_perfect_clears);
                } else {
                    self.metrics.current_consecutive_perfect_clears = 0;
                }

                self.handle_upgrade_trigger(UpgradeTriggerEvent::StageEnd {
                    perfect_clear,
                    gold,
                    item_count,
                });
                true
            }
            GameStateAction::GameOver => {
                self.record_event(crate::game_state::play_history::HistoryEventType::GameOver);
                true
            }
        }
    }
}
