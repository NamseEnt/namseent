mod apply_user_status_effect;
mod card_reroll;
mod earn_gold;
mod gain_rerolls;
mod gain_shield;
mod game_over;
mod game_start;
mod grant_hand_item;
mod grant_tower_card;
mod heal;
mod place_tower;
mod purchase_shop_item;
mod remove_tower;
mod spend_gold;
mod stage_end;
mod start_defense;
mod start_placing_tower;
mod start_selecting_tower;
mod start_stage;
mod start_treasure_selection;
mod take_damage;
mod upgrade;
mod upgrade_trigger;
mod use_card_service;
mod use_item;

use crate::game_state::{
    GameState,
    card::{Rank, Suit},
    hand::{HandItem, HandSlotId},
    item,
    tower::{Tower, TowerKind},
    upgrade::Upgrade,
    user_status_effect::UserStatusEffect,
};

pub(crate) enum GameStateAction {
    GameStart,
    StartStage {
        stage: usize,
    },
    EarnGold(usize),
    Heal(f32),
    GainRerolls(usize),
    CardReroll,
    GainShield(f32),
    SpendGold(usize),
    Upgrade(Upgrade, Option<usize>),
    PlaceTower(Box<Tower>, Option<HandSlotId>),
    RemoveTower(usize),
    MonsterDeath,
    PurchaseShopItem(crate::shop::ShopSlotId),
    GrantHandItem(HandItem),
    GrantTowerCard {
        tower_kind: TowerKind,
        suit: Option<Suit>,
        rank: Option<Rank>,
    },
    // Apply a user status effect to the player. maybe useful for treasure in near future.
    #[allow(dead_code)]
    ApplyUserStatusEffect(UserStatusEffect),
    UseInventoryItem(item::ItemId),
    TakeDamage(f32),
    StageEnd {
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    },
    StartPlacingTower(crate::game_state::tower::TowerTemplate),
    StartSelectingTower,
    StartDefense,
    StartTreasureSelection,
    UseCardService(crate::game_state::card_service::CardService),
    GameOver,
}

impl GameState {
    pub(crate) fn action(&mut self, action: GameStateAction) -> bool {
        match action {
            GameStateAction::GameStart => {
                game_start::record_history_event(self);
                true
            }
            GameStateAction::StartStage { stage } => {
                start_stage::reset_stage_state(self);
                start_stage::renew_game_state(self, stage);
                start_stage::flush_hand(self);
                start_stage::draw_hand(self);
                start_stage::open_panels(self);
                start_stage::trigger_upgrade_effects(self, stage);
                start_stage::set_shopping_flow(self);
                start_stage::record_history_event(self, stage);
                start_stage::save_debug_snapshot(self);
                true
            }
            GameStateAction::EarnGold(amount) => {
                earn_gold::add_to_balance(self, amount);
                earn_gold::trigger_upgrades(self, amount);
                earn_gold::play_earn_sound(self, amount);
                true
            }
            GameStateAction::Heal(amount) => {
                heal::apply(self, amount);
                true
            }
            GameStateAction::GainRerolls(amount) => {
                gain_rerolls::apply(self, amount);
                true
            }
            GameStateAction::GainShield(amount) => {
                gain_shield::apply(self, amount);
                true
            }
            GameStateAction::CardReroll => {
                let health_cost = self.stage_modifiers.get_reroll_health_cost();
                if (self.left_dice > 0) || (self.hp - health_cost as f32) > 1.0 {
                    let rerolled = card_reroll::reroll(self);
                    crate::sound::play_card_draw_sounds(rerolled);
                    card_reroll::apply_cost(self, health_cost);
                    card_reroll::trigger_upgrades(self);
                }
                true
            }
            GameStateAction::SpendGold(amount) => {
                spend_gold::deduct_from_balance(self, amount);
                spend_gold::trigger_upgrades(self, amount);
                spend_gold::play_spend_sound(self, amount);
                true
            }
            GameStateAction::Upgrade(upgrade, cost) => {
                upgrade::trigger_upgrades(self, upgrade);
                upgrade::record_history_event(self, upgrade, cost);
                true
            }
            GameStateAction::MonsterDeath => {
                self.handle_upgrade_trigger(
                    crate::game_state::action::upgrade_trigger::UpgradeTriggerEvent::MonsterDeath,
                );
                true
            }
            GameStateAction::PlaceTower(mut tower, placing_tower_slot_id) => {
                place_tower::prepare_tower_stats(&mut tower, &self.upgrade_state);
                if place_tower::place_tower(self, &tower) {
                    if let Some(slot_id) = placing_tower_slot_id {
                        self.hand.delete_slots(&[slot_id]);
                        place_tower::auto_select_first_tower(self);
                    }
                    place_tower::recalculate_route(self);
                    place_tower::trigger_upgrades(self, &tower);
                    place_tower::record_history_event(self, &tower);
                    place_tower::play_placement_sound(self);
                }
                true
            }
            GameStateAction::RemoveTower(tower_id) => {
                let removed = remove_tower::remove_tower(self, tower_id);
                if removed {
                    remove_tower::recalculate_route(self);
                    remove_tower::trigger_upgrades(self);
                    remove_tower::record_history_event(self, tower_id);
                    remove_tower::play_removal_sound(self);
                }
                true
            }
            GameStateAction::PurchaseShopItem(slot_id) => {
                purchase_shop_item::try_purchase(self, slot_id);
                true
            }
            GameStateAction::GrantHandItem(item) => {
                grant_hand_item::apply(self, item);
                true
            }
            GameStateAction::GrantTowerCard {
                tower_kind,
                suit,
                rank,
            } => {
                grant_tower_card::apply(self, tower_kind, suit, rank);
                true
            }
            GameStateAction::ApplyUserStatusEffect(status_effect) => {
                apply_user_status_effect::apply(self, status_effect);
                true
            }
            GameStateAction::UseInventoryItem(item_id) => {
                let Some((item_index, item)) = self
                    .items
                    .iter()
                    .enumerate()
                    .find(|(_index, item)| item.id == item_id)
                else {
                    return true;
                };

                if !use_item::can_use(self, item) {
                    return true;
                }

                let item = self.items.remove(item_index);

                use_item::mark_as_used(self);
                use_item::apply_effect(self, &item);
                use_item::record_history_event(self, &item);
                true
            }
            GameStateAction::TakeDamage(damage) => {
                take_damage::shake_camera(self, damage);
                let actual_damage = take_damage::apply_shield_and_damage(self, damage);
                take_damage::play_damage_sounds(self, damage);
                take_damage::record_history_event(self, damage, actual_damage);
                take_damage::check_game_over(self);
                true
            }
            GameStateAction::StageEnd {
                perfect_clear,
                gold,
                item_count,
            } => {
                stage_end::update_clear_metrics(self, perfect_clear);
                stage_end::trigger_upgrades(self, perfect_clear, gold, item_count);
                true
            }
            GameStateAction::StartPlacingTower(tower_template) => {
                let towers = start_placing_tower::collect_tower_hand(self, tower_template);
                start_placing_tower::flush_hand(self);
                start_placing_tower::fill_tower_hand(self, towers);
                start_placing_tower::select_first_tower(self);
                start_placing_tower::set_placing_flow(self);
                true
            }
            GameStateAction::StartSelectingTower => {
                start_selecting_tower::set_selecting_tower_flow(self);
                true
            }
            GameStateAction::StartDefense => {
                start_defense::set_defense_flow(self);
                start_defense::play_fanfare_sound(self);
                start_defense::begin_monster_spawn(self);
                true
            }
            GameStateAction::StartTreasureSelection => {
                start_treasure_selection::set_treasure_selection_flow(self);
                true
            }
            GameStateAction::GameOver => {
                game_over::clear_active_entities(self);
                game_over::record_history_event(self);
                game_over::set_result_flow(self);
                true
            }
            GameStateAction::UseCardService(card_service) => {
                use_card_service::use_card_service(self, card_service);
                true
            }
        }
    }
}
