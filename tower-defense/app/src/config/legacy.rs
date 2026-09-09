use crate::game_state::monster::MonsterKind;
use crate::game_state::tower::TowerKind;
use crate::{Damage, FixedRatio, Health, WorldDistance};
use namui::*;
use std::collections::BTreeMap;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub(crate) struct LegacyGameConfig {
    pub(crate) player: LegacyPlayerConfig,
    pub(crate) monsters: LegacyMonsterConfig,
    pub(crate) towers: LegacyTowerConfig,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub(crate) struct LegacyPlayerConfig {
    pub(crate) max_hp: Health,
    pub(crate) starting_gold: usize,
    pub(crate) starting_hp: Health,
    pub(crate) base_dice_chance: usize,
    pub(crate) max_stages: usize,
    pub(crate) base_hand_slots: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub(crate) struct LegacyMonsterConfig {
    pub(crate) stats: BTreeMap<MonsterKind, LegacyMonsterStats>,
    pub(crate) stage_waves: Vec<LegacyStageWave>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub(crate) struct LegacyMonsterStats {
    pub(crate) base_hp: Health,
    pub(crate) velocity_mul: FixedRatio,
    pub(crate) damage: Damage,
    pub(crate) reward: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub(crate) struct LegacyStageWave {
    pub(crate) stage: usize,
    pub(crate) entries: Vec<LegacyStageWaveEntry>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub(crate) struct LegacyStageWaveEntry {
    pub(crate) kind: MonsterKind,
    pub(crate) count: usize,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub(crate) struct LegacyTowerConfig {
    pub(crate) stats: BTreeMap<TowerKind, LegacyTowerStats>,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub(crate) struct LegacyTowerStats {
    pub(crate) damage: Damage,
    pub(crate) range: WorldDistance,
    pub(crate) cooldown_ms: u64,
}

impl LegacyGameConfig {
    pub(crate) fn from_core_state(state: td_core::GameConfig) -> Option<Self> {
        let player = LegacyPlayerConfig {
            max_hp: Health::from_raw(state.player.max_hp_raw),
            starting_gold: state.player.starting_gold,
            starting_hp: Health::from_raw(state.player.starting_hp_raw),
            base_dice_chance: state.player.base_dice_chance,
            max_stages: state.player.max_stages,
            base_hand_slots: state.player.base_hand_slots,
        };

        let mut monster_stats = BTreeMap::new();
        for entry in state.monsters.stats {
            monster_stats.insert(
                MonsterKind::from_core_raw(entry.kind)?,
                LegacyMonsterStats {
                    base_hp: Health::from_raw(entry.base_hp_raw),
                    velocity_mul: FixedRatio::from_raw(entry.velocity_mul_raw),
                    damage: Damage::from_raw(entry.damage_raw),
                    reward: entry.reward,
                },
            );
        }
        let stage_waves = state
            .monsters
            .stage_waves
            .into_iter()
            .map(|wave| {
                Some(LegacyStageWave {
                    stage: wave.stage,
                    entries: wave
                        .entries
                        .into_iter()
                        .map(|entry| {
                            Some(LegacyStageWaveEntry {
                                kind: MonsterKind::from_core_raw(entry.kind)?,
                                count: entry.count,
                            })
                        })
                        .collect::<Option<Vec<_>>>()?,
                })
            })
            .collect::<Option<Vec<_>>>()?;

        let mut tower_stats = BTreeMap::new();
        for entry in state.towers.entries {
            tower_stats.insert(
                TowerKind::from_core_raw(entry.kind)?,
                LegacyTowerStats {
                    damage: Damage::from_raw(entry.damage_raw),
                    range: WorldDistance::from_raw(entry.range_raw),
                    cooldown_ms: entry.cooldown_ms,
                },
            );
        }

        Some(Self {
            player,
            monsters: LegacyMonsterConfig {
                stats: monster_stats,
                stage_waves,
            },
            towers: LegacyTowerConfig { stats: tower_stats },
        })
    }

    pub(crate) fn to_core_state(&self) -> td_core::GameConfig {
        td_core::GameConfig {
            player: td_core::PlayerConfigState {
                max_hp_raw: self.player.max_hp.raw(),
                starting_gold: self.player.starting_gold,
                starting_hp_raw: self.player.starting_hp.raw(),
                base_dice_chance: self.player.base_dice_chance,
                max_stages: self.player.max_stages,
                base_hand_slots: self.player.base_hand_slots,
            },
            towers: td_core::TowerConfigState {
                entries: self
                    .towers
                    .stats
                    .iter()
                    .map(|(kind, stats)| td_core::TowerConfigEntryState {
                        kind: kind.to_core_raw(),
                        damage_raw: stats.damage.raw(),
                        range_raw: stats.range.raw(),
                        cooldown_ms: stats.cooldown_ms,
                    })
                    .collect(),
            },
            monsters: td_core::MonsterConfigState {
                stats: self
                    .monsters
                    .stats
                    .iter()
                    .map(|(kind, stats)| td_core::MonsterConfigEntryState {
                        kind: kind.to_core_raw(),
                        base_hp_raw: stats.base_hp.raw(),
                        velocity_mul_raw: stats.velocity_mul.raw(),
                        damage_raw: stats.damage.raw(),
                        reward: stats.reward,
                    })
                    .collect(),
                stage_waves: self
                    .monsters
                    .stage_waves
                    .iter()
                    .map(|wave| td_core::StageWaveState {
                        stage: wave.stage,
                        entries: wave
                            .entries
                            .iter()
                            .map(|entry| td_core::StageWaveEntryState {
                                kind: entry.kind.to_core_raw(),
                                count: entry.count,
                            })
                            .collect(),
                    })
                    .collect(),
            },
        }
    }
}
