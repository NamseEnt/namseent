use crate::game_state::{
    GameState, compatibility_action, item, presentation_event::PresentationEvent, tower::Tower,
    upgrade::Upgrade,
};
use crate::{Damage, TowerId};

pub(crate) fn apply_damage(game_state: &mut GameState, damage: Damage, actual_damage: Damage) {
    compatibility_action::take_damage::apply_presentation_effects(
        game_state,
        damage,
        actual_damage,
    );
}

pub(crate) fn apply_card_reroll(
    game_state: &mut GameState,
    rerolled: usize,
    damage: Damage,
    actual_damage: Damage,
) {
    game_state.push_presentation_event(PresentationEvent::PlayCardDrawSounds {
        card_count: rerolled,
    });
    apply_damage(game_state, damage, actual_damage);
}

pub(crate) fn apply_shop_purchase(
    game_state: &mut GameState,
    slot_id: crate::shop::ShopSlotId,
    slot: crate::shop::ShopSlot,
    cost_value: usize,
) {
    compatibility_action::purchase_shop_item::apply_purchase_presentation_effects(
        game_state, slot_id, slot, cost_value,
    );
}

pub(crate) fn apply_place_tower(game_state: &mut GameState, tower: &Tower) {
    compatibility_action::place_tower::record_history_event(game_state, tower);
    compatibility_action::place_tower::play_placement_sound(game_state);
}

pub(crate) fn apply_remove_tower(game_state: &mut GameState, tower_id: TowerId) {
    compatibility_action::remove_tower::record_history_event(game_state, tower_id);
    compatibility_action::remove_tower::play_removal_sound(game_state);
}

pub(crate) fn apply_inventory_item(game_state: &mut GameState, item: &item::ItemWithId) {
    compatibility_action::use_item::apply_presentation_effects(game_state, &item.item);
}

pub(crate) fn apply_upgrade(game_state: &mut GameState, upgrade: Upgrade, cost: Option<usize>) {
    game_state.discover_treasure(upgrade);
    compatibility_action::upgrade::record_history_event(game_state, upgrade, cost);
}

pub(crate) fn apply_stage_start(game_state: &mut GameState, stage: usize, card_count: usize) {
    compatibility_action::start_stage::apply_presentation_effects(game_state, stage, card_count);
}

pub(crate) fn apply_stage_end(game_state: &mut GameState, stage: usize, perfect_clear: bool) {
    if perfect_clear {
        game_state.record_event(
            crate::game_state::play_history::HistoryEventType::StagePerfectClear { stage },
        );
    }
}

pub(crate) fn apply_game_over(game_state: &mut GameState) {
    compatibility_action::game_over::record_history_event(game_state);
}

pub(crate) fn apply_earn_gold_sound(game_state: &mut GameState, amount: usize) {
    compatibility_action::earn_gold::play_earn_sound(game_state, amount);
}
