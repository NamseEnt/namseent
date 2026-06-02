use crate::card::{Rank, Suit};
use crate::game_state::GameState;
use crate::game_state::flow::GameFlow;
use crate::game_state::tower::{TowerKind, TowerTemplate};
use crate::hand::HandItem;

pub(super) fn apply(
    game_state: &mut GameState,
    tower_kind: TowerKind,
    suit: Option<Suit>,
    rank: Option<Rank>,
) {
    if matches!(game_state.flow, GameFlow::PlacingTower) {
        game_state
            .hand
            .push(HandItem::Tower(TowerTemplate::new_optional(
                tower_kind, suit, rank,
            )));
    } else {
        game_state
            .stage_modifiers
            .enqueue_extra_tower_card(tower_kind, suit, rank);
    }
}
