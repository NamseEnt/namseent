//! Card service strategies for simulator. Uses cfg(feature = "simulator") helpers from CardServiceBehavior.

use crate::game_state::GameState;
use crate::game_state::card_service::CardServiceBehavior;
use crate::game_state::modal::UserModal;
use rand::RngCore;

/// Strategy for handling card service selection in headless simulation.
pub trait CardServiceStrategy: Send + Sync {
    fn name(&self) -> &str;
    fn execute_card_service(&self, game_state: &mut GameState, rng: &mut dyn RngCore);
}

#[derive(Clone)]
pub struct HeuristicCardServiceStrategy;

impl CardServiceStrategy for HeuristicCardServiceStrategy {
    fn name(&self) -> &str {
        "heuristic_card_service"
    }

    fn execute_card_service(&self, game_state: &mut GameState, _rng: &mut dyn RngCore) {
        if let Some(UserModal::Deck(deck_modal)) = &mut game_state.opened_modals.user {
            if let Some(selection) = &deck_modal.selection {
                let service = selection.card_service.clone();
                let selected = service.heuristic_best_selection(game_state);
                service.select_cards(game_state, selected);
            }
            game_state.opened_modals.user = None;
        }
    }
}
