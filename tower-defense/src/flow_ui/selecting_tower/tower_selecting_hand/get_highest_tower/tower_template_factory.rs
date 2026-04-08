use crate::card::{Rank, Suit};
use crate::config::GameConfig;
use crate::game_state::tower::TowerKind;
use crate::game_state::tower::TowerTemplate;

pub fn create_tower_template(
    kind: TowerKind,
    suit: Suit,
    rank: Rank,
    config: &GameConfig,
) -> TowerTemplate {
    TowerTemplate::new_with_config(kind, suit, rank, config)
}
