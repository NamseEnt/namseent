use crate::game_state::tower::TowerKind;
use namui::*;
use std::collections::BTreeMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub struct TowerStats {
    pub damage: f32,
    pub range: f32,
    pub cooldown_ms: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub struct TowerConfig {
    pub stats: BTreeMap<TowerKind, TowerStats>,
}

pub fn default_tower_config() -> TowerConfig {
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

    TowerConfig { stats }
}
