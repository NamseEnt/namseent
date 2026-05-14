use crate::game_state::{GameState, flow::GameFlow, tower::TowerTemplate};
use crate::hand::HandItem;

pub(super) fn collect_tower_hand(
    game_state: &mut GameState,
    initial_tower: TowerTemplate,
) -> Vec<TowerTemplate> {
    let mut hand_items = vec![initial_tower];
    for (tower_kind, suit, rank) in game_state.stage_modifiers.drain_extra_tower_cards() {
        hand_items.push(TowerTemplate::new_with_config(
            tower_kind,
            suit,
            rank,
            &game_state.config,
        ));
    }
    hand_items
}

pub(super) fn flush_hand(game_state: &mut GameState) {
    let removing_ids = game_state.hand.active_slot_ids();
    if !removing_ids.is_empty() {
        game_state.hand.delete_slots(&removing_ids);
    }
}

pub(super) fn fill_tower_hand(game_state: &mut GameState, towers: Vec<TowerTemplate>) {
    for tower in towers {
        game_state.hand.push(HandItem::Tower(tower));
    }
}

pub(super) fn select_first_tower(game_state: &mut GameState) {
    if let Some(first_slot_id) = game_state.hand.get_slot_id_by_index(0)
        && game_state
            .hand
            .get_item(first_slot_id)
            .and_then(|item| item.as_tower())
            .is_some()
    {
        game_state.hand.select_slot(first_slot_id);
    }
}

pub(super) fn set_placing_flow(game_state: &mut GameState) {
    game_state.flow = GameFlow::PlacingTower;
}
