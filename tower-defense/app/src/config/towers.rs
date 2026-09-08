use crate::game_state::tower::TowerKind;
use crate::{Damage, WorldDistance};
use namui::*;
use std::collections::BTreeMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PartialEq, State)]
pub struct TowerStats {
    pub damage: Damage,
    pub range: WorldDistance,
    pub cooldown_ms: u64,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub struct TowerConfig {
    pub stats: BTreeMap<TowerKind, TowerStats>,
}

impl TowerConfig {
    pub(crate) fn to_core_state(&self) -> td_core::TowerConfigState {
        td_core::TowerConfigState {
            entries: self
                .stats
                .iter()
                .map(|(kind, stats)| td_core::TowerConfigEntryState {
                    kind: kind.to_core_raw(),
                    damage_raw: stats.damage.raw(),
                    range_raw: stats.range.raw(),
                    cooldown_ms: stats.cooldown_ms,
                })
                .collect(),
        }
    }

    pub(crate) fn from_core_state(state: td_core::TowerConfigState) -> Option<Self> {
        let mut stats = BTreeMap::new();
        for entry in state.entries {
            let kind = TowerKind::from_core_raw(entry.kind)?;
            if stats
                .insert(
                    kind,
                    TowerStats {
                        damage: Damage::from_raw(entry.damage_raw),
                        range: WorldDistance::from_raw(entry.range_raw),
                        cooldown_ms: entry.cooldown_ms,
                    },
                )
                .is_some()
            {
                return None;
            }
        }
        Some(Self { stats })
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tower_config_raw_state_round_trips_without_host_types() {
        let config = default_tower_config();
        let state = config.to_core_state();
        let restored = TowerConfig::from_core_state(state).expect("valid tower config");

        assert_eq!(restored.stats, config.stats);
    }

    #[test]
    fn tower_config_raw_state_rejects_duplicate_kinds() {
        let config = default_tower_config();
        let mut state = config.to_core_state();
        state.entries.push(state.entries[0].clone());

        assert!(TowerConfig::from_core_state(state).is_none());
    }
}
