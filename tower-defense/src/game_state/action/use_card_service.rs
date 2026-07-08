use crate::game_state::card_service::CardServiceBehavior;

pub(super) fn use_card_service(
    game_state: &mut crate::game_state::GameState,
    card_service: crate::game_state::card_service::CardService,
) {
    card_service.acquire(game_state);
}
