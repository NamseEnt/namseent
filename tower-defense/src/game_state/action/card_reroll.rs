use crate::game_state::GameState;
use crate::game_state::action::upgrade_trigger::UpgradeTriggerEvent;
use crate::game_state::card::Card;

pub(super) fn reroll(game_state: &mut GameState) -> usize {
    let selected_slot_ids = game_state.hand.selected_slot_ids();
    let target_slot_ids = if selected_slot_ids.is_empty() {
        game_state.hand.active_slot_ids()
    } else {
        selected_slot_ids
    };
    let target_count = target_slot_ids.len();
    if target_count == 0 {
        return 0;
    }
    let target_cards: Vec<Card> = target_slot_ids
        .iter()
        .filter_map(|id| {
            game_state
                .hand
                .get_item(*id)
                .and_then(|item| item.as_card().copied())
        })
        .collect();
    game_state.hand.delete_slots(&target_slot_ids);
    game_state.deck.put_back(target_cards);
    (0..target_count).for_each(|_| {
        let card = game_state.deck.draw().unwrap_or_else(Card::new_random);
        game_state.hand.push(crate::hand::HandItem::Card(card));
    });
    target_count
}

pub(super) fn apply_cost(game_state: &mut GameState, health_cost: usize) {
    game_state.left_dice -= 1;
    game_state.rerolled_count += 1;
    game_state.action(crate::game_state::GameStateAction::TakeDamage(
        health_cost as f32,
    ));
}

pub(super) fn trigger_upgrades(game_state: &mut GameState) {
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::CardReroll);
}
