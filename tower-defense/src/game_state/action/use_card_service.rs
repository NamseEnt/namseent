use crate::game_state::card_service::CardServiceBehavior;

pub(super) fn use_card_service(
    game_state: &mut crate::game_state::GameState,
    card_service: crate::game_state::card_service::CardService,
) {
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
        card_service.select_cards(game_state, selected);
    } else {
        crate::tooltip::hide_tooltip_all();
        card_service.acquire(game_state);
    }
}
