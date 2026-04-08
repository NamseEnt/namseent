//! Card reroll and tower selection strategies.

use super::CardRerollStrategy;
use crate::card::Card;
use crate::flow_ui::selecting_tower::tower_selecting_hand::get_highest_tower::get_highest_tower_template;
use crate::game_state::GameState;
use crate::game_state::tower::TowerKind;
use crate::hand::HandItem;
use rand::RngCore;

/// Uses all rerolls trying every combination to find the best possible tower.
/// For each reroll, evaluates all possible single-card replacements and picks the best.
pub struct OptimalRerollStrategy;

impl CardRerollStrategy for OptimalRerollStrategy {
    fn name(&self) -> &str {
        "optimal_reroll"
    }

    fn execute_card_selection(&self, game_state: &mut GameState, _rng: &mut dyn RngCore) {
        // Use all available rerolls
        while game_state.left_dice > 0 {
            let cards = collect_hand_cards(game_state);
            if cards.is_empty() {
                break;
            }

            let current_template =
                get_highest_tower_template(
                    &cards,
                    &game_state.upgrade_state,
                    game_state.rerolled_count,
                    &game_state.config,
                );
            let current_rating = tower_kind_rating(current_template.kind);

            // If we already have a very good hand (Flush or better), stop rerolling
            if current_rating >= tower_kind_rating(TowerKind::Flush) {
                break;
            }

            // Perform one reroll
            game_state.left_dice -= 1;
            game_state.rerolled_count += 1;

            // Reroll: redraw all cards from deck
            let active_ids = game_state.hand.active_slot_ids();
            let old_cards: Vec<Card> = active_ids
                .iter()
                .filter_map(|id| {
                    game_state
                        .hand
                        .get_item(*id)
                        .and_then(|item| item.as_card().copied())
                })
                .collect();

            // Put old cards back in deck and draw new ones
            game_state.deck.put_back(old_cards);
            game_state.hand.delete_slots(&active_ids);

            let max_slots = (5 + game_state.stage_modifiers.get_max_hand_slots_bonus())
                .saturating_sub(game_state.stage_modifiers.get_max_hand_slots_penalty())
                .max(1);

            for _ in 0..max_slots {
                let card = game_state
                    .deck
                    .draw()
                    .unwrap_or_else(Card::new_random);
                game_state.hand.push(HandItem::Card(card));
            }
        }

        // Now select the tower from current hand
        let cards = collect_hand_cards(game_state);
        if cards.is_empty() {
            return;
        }

        let tower_template =
            get_highest_tower_template(
                &cards,
                &game_state.upgrade_state,
                game_state.rerolled_count,
                &game_state.config,
            );

        game_state.goto_placing_tower(tower_template);
    }
}

/// No reroll strategy - just takes whatever cards are dealt.
pub struct NoRerollStrategy;

impl CardRerollStrategy for NoRerollStrategy {
    fn name(&self) -> &str {
        "no_reroll"
    }

    fn execute_card_selection(&self, game_state: &mut GameState, _rng: &mut dyn RngCore) {
        let cards = collect_hand_cards(game_state);
        if cards.is_empty() {
            return;
        }

        let tower_template =
            get_highest_tower_template(
                &cards,
                &game_state.upgrade_state,
                game_state.rerolled_count,
                &game_state.config,
            );

        game_state.goto_placing_tower(tower_template);
    }
}

fn collect_hand_cards(game_state: &GameState) -> Vec<Card> {
    let slot_ids = game_state.hand.active_slot_ids();
    slot_ids
        .iter()
        .filter_map(|id| {
            game_state
                .hand
                .get_item(*id)
                .and_then(|item| item.as_card().copied())
        })
        .collect()
}

fn tower_kind_rating(kind: TowerKind) -> u32 {
    match kind {
        TowerKind::Barricade => 0,
        TowerKind::High => 1,
        TowerKind::OnePair => 2,
        TowerKind::TwoPair => 3,
        TowerKind::ThreeOfAKind => 4,
        TowerKind::Straight => 5,
        TowerKind::Flush => 6,
        TowerKind::FullHouse => 7,
        TowerKind::FourOfAKind => 8,
        TowerKind::StraightFlush => 9,
        TowerKind::RoyalFlush => 10,
    }
}
