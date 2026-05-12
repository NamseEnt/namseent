mod earn_gold;
mod game_over;
mod game_start;
mod place_tower;
mod purchase_shop_item;
mod remove_tower;
mod spend_gold;
mod stage_end;
mod start_defense;
mod start_placing_tower;
mod start_stage;
mod start_treasure_selection;
mod take_damage;
mod upgrade;
mod upgrade_trigger;
mod use_item;

use crate::game_state::{GameState, hand::HandSlotId, item, tower::Tower, upgrade::Upgrade};

pub(crate) enum GameStateAction<'a> {
    GameStart,
    StartStage {
        stage: usize,
    },
    EarnGold(usize),
    SpendGold(usize),
    Upgrade(Upgrade, Option<usize>),
    PlaceTower(Box<Tower>, Option<HandSlotId>),
    RemoveTower(usize),
    PurchaseShopItem(crate::shop::ShopSlotId),
    UseItem(&'a item::Item),
    TakeDamage(f32),
    StageEnd {
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    },
    StartPlacingTower(crate::game_state::tower::TowerTemplate),
    StartDefense,
    StartTreasureSelection,
    GameOver,
}

impl GameState {
    pub(crate) fn action(&mut self, action: GameStateAction<'_>) -> bool {
        match action {
            GameStateAction::GameStart => {
                game_start::record_history_event(self);
                true
            }
            GameStateAction::StartStage { stage } => {
                start_stage::reset_stage_state(self);
                start_stage::renew_game_state(self);
                start_stage::flush_hand(self);
                start_stage::draw_hand(self);
                start_stage::open_panels(self);
                start_stage::set_selecting_tower_flow(self);
                start_stage::trigger_upgrade_effects(self, stage);
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
            GameStateAction::UseItem(item) => {
                if use_item::can_use(self) {
                    use_item::mark_as_used(self);
                    use_item::apply_effect(self, item);
                    use_item::record_history_event(self, item);
                }
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
        }
    }
}
