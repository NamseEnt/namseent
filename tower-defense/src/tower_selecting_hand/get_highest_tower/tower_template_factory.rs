use crate::card::{Rank, Suit};
use crate::game_state::projectile::ProjectileKind;
use crate::game_state::tower::TowerKind;
use crate::game_state::tower::TowerTemplate;
use namui::{DurationExt, Per};

pub fn create_tower_template(kind: TowerKind, suit: Suit, rank: Rank) -> TowerTemplate {
    let shoot_interval = match kind {
        TowerKind::Barricade => 8192.0,
        TowerKind::High => 1.0,
        TowerKind::OnePair => 1.0,
        TowerKind::TwoPair => 1.0,
        TowerKind::ThreeOfAKind => 1.0,
        TowerKind::Straight => 1.0,
        TowerKind::Flush => 0.5,
        TowerKind::FullHouse => 1.0,
        TowerKind::FourOfAKind => 1.0,
        TowerKind::StraightFlush => 0.5,
        TowerKind::RoyalFlush => 1.0 / 3.0,
    }
    .sec();

    let default_attack_range_radius = match kind {
        TowerKind::Barricade => 0.0,
        TowerKind::High => 5.0,
        TowerKind::OnePair => 5.0,
        TowerKind::TwoPair => 5.0,
        TowerKind::ThreeOfAKind => 5.0,
        TowerKind::Straight => 10.0,
        TowerKind::Flush => 5.0,
        TowerKind::FullHouse => 5.0,
        TowerKind::FourOfAKind => 5.0,
        TowerKind::StraightFlush => 10.0,
        TowerKind::RoyalFlush => 15.0,
    };

    let default_damage = match kind {
        TowerKind::Barricade => 0.0,
        TowerKind::High => 1.0,
        TowerKind::OnePair => 5.0,
        TowerKind::TwoPair => 10.0,
        TowerKind::ThreeOfAKind => 25.0,
        TowerKind::Straight => 50.0,
        TowerKind::Flush => 75.0,
        TowerKind::FullHouse => 200.0,
        TowerKind::FourOfAKind => 250.0,
        TowerKind::StraightFlush => 1500.0,
        TowerKind::RoyalFlush => 3000.0,
    };

    TowerTemplate {
        kind,
        shoot_interval,
        default_attack_range_radius,
        projectile_kind: ProjectileKind::Ball,
        projectile_speed: Per::new(16.0, 1.sec()),
        default_damage,
        suit,
        rank,
        skill_templates: vec![],
        default_status_effects: vec![],
    }
}
