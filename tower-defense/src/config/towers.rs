use crate::game_state::tower::TowerKind;
use crate::{Damage, WorldDistance};
use namui::*;
use std::collections::BTreeMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub struct TowerStats {
    pub damage: Damage,
    pub range: WorldDistance,
    pub cooldown_ms: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub struct TowerConfig {
    pub stats: BTreeMap<TowerKind, TowerStats>,
}

pub fn default_tower_config() -> TowerConfig {
    use TowerKind::*;

    let mut stats = BTreeMap::new();
    let tower_data: Vec<(TowerKind, i64, i64, u64)> = vec![
        (RubberCone, 0, 4, 1000),
        (High, 5, 4, 1000),
        (OnePair, 6, 5, 1000),
        (TwoPair, 10, 6, 1000),
        (ThreeOfAKind, 12, 7, 1000),
        (Straight, 14, 9, 500),
        (Flush, 32, 9, 1000),
        (FullHouse, 50, 11, 1000),
        (FourOfAKind, 100, 11, 1000),
        (StraightFlush, 250, 14, 500),
        (RoyalFlush, 1200, 15, 1000),
    ];

    for (kind, damage, range, cooldown_ms) in tower_data {
        stats.insert(
            kind,
            TowerStats {
                damage: Damage::from_integer(damage),
                range: WorldDistance::from_tiles(range),
                cooldown_ms,
            },
        );
    }

    TowerConfig { stats }
}
