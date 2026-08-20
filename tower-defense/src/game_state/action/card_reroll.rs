use crate::Damage;
use crate::card::Card;
use crate::game_state::GameState;
use crate::game_state::action::upgrade_trigger::UpgradeTriggerEvent;

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
    game_state.deck.discard(target_cards);

    let mut rng = game_state.rng.next_rng(
        crate::deterministic_rng::domain::CARD_REROLL,
        &[game_state.stage as u64, game_state.rerolled_count as u64],
    );
    let cards = game_state.deck.draw(&mut rng, target_count);
    let draw_count = cards.len();
    for card in cards {
        game_state.hand.push(crate::hand::HandItem::Card(card));
    }

    draw_count
}

pub(super) fn apply_cost(game_state: &mut GameState, health_cost: usize) {
    if game_state.left_dice > 0 {
        game_state.left_dice -= 1;
    }
    game_state.rerolled_count += 1;
    game_state.action(crate::game_state::GameStateAction::TakeDamage(
        Damage::from_usize(health_cost),
    ));
}

pub(super) fn trigger_upgrades(game_state: &mut GameState) {
    game_state.handle_upgrade_trigger(UpgradeTriggerEvent::CardReroll);
}

#[cfg(test)]
mod tests {
    use crate::Health;
    use crate::game_state::{GameStateAction, create_game_state_with_seed};

    #[test]
    fn health_paid_reroll_does_not_consume_missing_die() {
        let mut game_state = create_game_state_with_seed(0xC4D0_7E77);
        game_state.left_dice = 0;
        let health_before = game_state.hp;
        let health_cost = game_state.stage_modifiers.get_reroll_health_cost();

        game_state.action(GameStateAction::CardReroll);

        assert_eq!(game_state.left_dice, 0);
        assert_eq!(
            game_state.hp,
            health_before.saturating_sub(Health::from_usize(health_cost))
        );
    }
}
