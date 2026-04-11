use crate::card::Rank;
use crate::game_state::tower::TowerKind;
use namui::*;
use std::collections::BTreeMap;

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct TowerStats {
    pub damage: f32,
    pub range: f32,
    pub cooldown_ms: u64,
}

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct TowerConfig {
    pub stats: BTreeMap<TowerKind, TowerStats>,
    pub rank_bonus_damage: BTreeMap<Rank, usize>,
}

pub fn default_tower_config() -> TowerConfig {
    use crate::card::Rank;
    use TowerKind::*;

    let mut stats = BTreeMap::new();
    let tower_data: Vec<(TowerKind, f32, f32, u64)> = vec![
        (Barricade, 0.0, 4.0, 1000),
        (High, 5.0, 4.0, 1000),
        (OnePair, 6.0, 5.0, 1000),
        (TwoPair, 10.0, 6.0, 1000),
        (ThreeOfAKind, 12.0, 7.0, 1000),
        (Straight, 14.0, 9.0, 500),
        (Flush, 32.0, 9.0, 1000),
        (FullHouse, 50.0, 11.0, 1000),
        (FourOfAKind, 100.0, 11.0, 1000),
        (StraightFlush, 250.0, 14.0, 500),
        (RoyalFlush, 1200.0, 15.0, 1000),
    ];

    for (kind, damage, range, cooldown_ms) in tower_data {
        stats.insert(
            kind,
            TowerStats {
                damage,
                range,
                cooldown_ms,
            },
        );
    }

    let mut rank_bonus = BTreeMap::new();
    let bonus: Vec<(Rank, usize)> = vec![
        (Rank::Two, 0),
        (Rank::Three, 0),
        (Rank::Four, 0),
        (Rank::Five, 1),
        (Rank::Six, 1),
        (Rank::Seven, 1),
        (Rank::Eight, 2),
        (Rank::Nine, 3),
        (Rank::Ten, 4),
        (Rank::Jack, 6),
        (Rank::Queen, 8),
        (Rank::King, 10),
        (Rank::Ace, 15),
    ];

    for (rank, dmg) in bonus {
        rank_bonus.insert(rank, dmg);
    }

    TowerConfig {
        stats,
        rank_bonus_damage: rank_bonus,
    }
}
