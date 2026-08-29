use crate::game_state::card_service::{CardServiceBehavior, CardServiceDiscriminants};

pub(super) fn use_card_service(
    game_state: &mut crate::game_state::GameState,
    card_service: crate::game_state::card_service::CardService,
    locale: crate::l10n::Locale,
) {
    let service_kind = CardServiceDiscriminants::from(&card_service).to_core_kind();
    let mut raw = game_state.raw_core.state().clone();
    raw.begin_card_service_selection(service_kind)
        .expect("card service selection definition must be available");
    game_state
        .restore_raw_core_projection(raw)
        .expect("raw card service selection must be restorable in headed adapter");
    if game_state.is_headless() {
        let selected = card_service.heuristic_best_selection(game_state);
        let service_kind = card_service.key().to_string();
        let cards_selected = selected.len();
        game_state.record_event(
            crate::game_state::play_history::HistoryEventType::CardServiceUsed {
                service_kind,
                cards_selected,
            },
        );
        let selected_card_ids = selected
            .into_iter()
            .map(|card_ids| card_ids.into_iter().map(|card_id| card_id.raw()).collect())
            .collect::<Vec<Vec<_>>>();
        let mut raw = game_state.raw_core.state().clone();
        raw.apply_card_service_selection_mutation(&selected_card_ids)
            .expect("heuristic card service selection must be valid");
        game_state
            .restore_raw_core_projection(raw)
            .expect("raw card service selection must be restorable in headed adapter");
    } else {
        card_service.acquire(game_state, locale);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headless_card_service_mutates_raw_deck_and_clears_selection() {
        let mut game_state = crate::game_state::create_initial_game_state();
        game_state.headless = true;
        let initial_card_count = game_state.raw_core.deck().all_cards.len();

        game_state.apply_compatibility_action(
            crate::game_state::CompatibilityAction::UseCardService {
                card_service: CardServiceDiscriminants::Eraser.generate(),
                locale: crate::l10n::Locale::KOREAN,
            },
        );

        assert_eq!(
            game_state.raw_core.deck().all_cards.len(),
            initial_card_count - 1
        );
        assert!(game_state.raw_core.pending_card_service_kind().is_none());
        assert!(game_state.raw_core.card_service_selection().is_none());
    }
}
