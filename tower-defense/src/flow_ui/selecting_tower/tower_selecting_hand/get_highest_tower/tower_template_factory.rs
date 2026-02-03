use crate::card::{Rank, Suit};
use crate::game_state::tower::TowerKind;
use crate::game_state::tower::TowerTemplate;

pub fn create_tower_template(kind: TowerKind, suit: Suit, rank: Rank) -> TowerTemplate {
    TowerTemplate {
        kind,
        shoot_interval: kind.shoot_interval(),
        default_attack_range_radius: kind.default_attack_range_radius(),
        default_damage: kind.default_damage(),
        suit,
        rank,
        skill_templates: vec![],
        default_status_effects: vec![],
    }
}
