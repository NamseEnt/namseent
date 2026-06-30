use crate::{
    game_state::{
        self, GameState,
        action::upgrade_trigger::UpgradeTriggerEvent,
        card::{Card, Deck},
        flow::{GameFlow, ShoppingFlow},
    },
    hand::HandItem,
    sound::play_card_draw_sounds,
};

#[cfg(feature = "debug-tools")]
fn save_stage_snapshot(game_state: &GameState) {
    crate::game_state::debug_tools::state_snapshot::save_snapshot_from_state(game_state);
}

#[cfg(not(feature = "debug-tools"))]
fn save_stage_snapshot(_game_state: &GameState) {}

pub(super) fn reset_stage_state(game_state: &mut GameState) {
    game_state.stage_modifiers.reset_stage_state();
}

pub(super) fn renew_game_state(game_state: &mut GameState) {
    if game_state.upgrade_state.clear_shield_on_stage_start() {
        game_state.shield = 0.0;
    }
    game_state.item_used = false;
    game_state.metrics.total_rerolled_count += game_state.rerolled_count;
    game_state.metrics.total_shop_rerolled_count += game_state.shop_rerolled_count;
    game_state.rerolled_count = 0;
    game_state.shop_rerolled_count = 0;
    game_state.deck = Deck::new(game_state.upgrade_state.removed_number_rank_count());
    game_state.left_dice = game_state.max_dice_chance();
}

pub(super) fn draw_hand(game_state: &mut GameState) {
    let max_slots = (game_state.config.player.base_hand_slots
        + game_state.stage_modifiers.get_max_hand_slots_bonus())
    .saturating_sub(game_state.stage_modifiers.get_max_hand_slots_penalty())
    .max(1);
    for _ in 0..max_slots {
        let card = game_state.deck.draw().unwrap_or_else(Card::new_random);
        game_state.hand.push(HandItem::Card(card));
    }
    play_card_draw_sounds(max_slots);
}

pub(super) fn flush_hand(game_state: &mut GameState) {
    let removing_ids = game_state.hand.active_slot_ids();
    if !removing_ids.is_empty() {
        game_state.hand.delete_slots(&removing_ids);
    }
}

pub(super) fn open_panels(game_state: &mut GameState) {
    game_state.hand_panel_forced_open = true;
    game_state.shop_panel_forced_open = true;
}

pub(super) fn set_shopping_flow(game_state: &mut GameState) {
    game_state.flow = GameFlow::Shopping(ShoppingFlow::new(game_state));
}

pub(super) fn trigger_upgrade_effects(game_state: &mut GameState, stage: usize) {
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::StageStart { stage });
}

pub(super) fn record_history_event(game_state: &mut GameState, stage: usize) {
    game_state.record_event(game_state::play_history::HistoryEventType::StageStart {
        stage,
        boss: game_state::is_boss_stage(stage),
    });
}

pub(super) fn save_debug_snapshot(game_state: &mut GameState) {
    if !crate::is_headless() {
        save_stage_snapshot(game_state);
    }
}
