//! Headless configuration loading and conversion.
//!
//! The headed crate uses Namui-backed numeric wrappers and enum map keys for
//! its TOML representation.  The simulator only needs the raw, serialized
//! `td-core` state, so this module owns the small compatibility parser and
//! never exposes the headed `GameConfig` type to runtime code.

use anyhow::{Context, bail};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};
use std::path::Path;

pub const GAME_CONFIG_VERSION: u32 = td_core::CORE_CONFIG_SCHEMA_VERSION;
pub const CONFIG_DIGEST_VERSION: u32 = 1;
const AMOUNT_SCALE: i64 = 1_000;
const RATIO_SCALE: i64 = td_core::RATIO_SCALE;
const WORLD_UNITS_PER_TILE: i64 = td_core::WORLD_UNITS_PER_TILE;
const EMBEDDED_GAMECONFIG_TOML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../app/gameconfig.toml"
));

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GameConfig(td_core::GameConfigState);

impl Deref for GameConfig {
    type Target = td_core::GameConfigState;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for GameConfig {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl GameConfig {
    pub fn to_core_state(&self) -> td_core::GameConfigState {
        self.0.clone()
    }

    pub fn from_core_state(state: td_core::GameConfigState) -> Option<Self> {
        if !valid_unique_kinds(
            state
                .towers
                .entries
                .iter()
                .map(|entry| (entry.kind, td_core::tower_kind_key(entry.kind))),
        ) {
            return None;
        }
        if !valid_unique_kinds(
            state
                .monsters
                .stats
                .iter()
                .map(|entry| (entry.kind, td_core::monster_kind_key(entry.kind))),
        ) {
            return None;
        }
        if state.monsters.stage_waves.iter().any(|wave| {
            wave.entries
                .iter()
                .any(|entry| td_core::monster_kind_key(entry.kind).is_none())
        }) {
            return None;
        }
        Some(Self(state))
    }

    pub fn from_toml<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("failed to read config file: {}", path.as_ref().display()))?;
        Self::from_toml_str(&content)
    }

    pub fn from_toml_str(content: &str) -> anyhow::Result<Self> {
        let file: FileGameConfig =
            toml::from_str(content).context("failed to parse simulator GameConfig TOML")?;
        Self::from_file(file)
    }

    pub fn default_config() -> Self {
        Self::from_toml_str(EMBEDDED_GAMECONFIG_TOML)
            .expect("embedded gameconfig.toml must be valid")
    }

    pub fn write_toml<P: AsRef<Path>>(&self, path: P) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(&self.to_file()?)
            .context("failed to serialize simulator GameConfig TOML")?;
        std::fs::write(path.as_ref(), content)
            .with_context(|| format!("failed to write config file: {}", path.as_ref().display()))
    }

    fn from_file(file: FileGameConfig) -> anyhow::Result<Self> {
        let player = td_core::PlayerConfigState {
            max_hp_raw: amount_raw(file.player.max_hp, "player.max_hp")?,
            starting_gold: file.player.starting_gold,
            starting_hp_raw: amount_raw(file.player.starting_hp, "player.starting_hp")?,
            base_dice_chance: file.player.base_dice_chance,
            max_stages: file.player.max_stages,
            base_hand_slots: file.player.base_hand_slots,
        };

        let mut tower_entries = file
            .towers
            .stats
            .into_iter()
            .map(|(key, stats)| {
                Ok(td_core::TowerConfigEntryState {
                    kind: tower_kind_from_key(&key)?,
                    damage_raw: amount_raw(stats.damage, &format!("towers.stats.{key}.damage"))?,
                    range_raw: distance_raw(stats.range, &format!("towers.stats.{key}.range"))?,
                    cooldown_ms: stats.cooldown_ms,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        tower_entries.sort_by_key(|entry| entry.kind);
        let towers = td_core::TowerConfigState {
            entries: tower_entries,
        };

        let mut monster_stats = file
            .monsters
            .stats
            .into_iter()
            .map(|(key, stats)| {
                Ok(td_core::MonsterConfigEntryState {
                    kind: monster_kind_from_key(&key)?,
                    base_hp_raw: amount_raw(
                        stats.base_hp,
                        &format!("monsters.stats.{key}.base_hp"),
                    )?,
                    velocity_mul_raw: ratio_raw(
                        stats.velocity_mul,
                        &format!("monsters.stats.{key}.velocity_mul"),
                    )?,
                    damage_raw: amount_raw(stats.damage, &format!("monsters.stats.{key}.damage"))?,
                    reward: stats.reward,
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;
        monster_stats.sort_by_key(|entry| entry.kind);
        let monsters = td_core::MonsterConfigState {
            stats: monster_stats,
            stage_waves: file
                .monsters
                .stage_waves
                .into_iter()
                .map(|wave| {
                    Ok(td_core::StageWaveState {
                        stage: wave.stage,
                        entries: wave
                            .entries
                            .into_iter()
                            .map(|entry| {
                                Ok(td_core::StageWaveEntryState {
                                    kind: monster_kind_from_key(&entry.kind)?,
                                    count: entry.count,
                                })
                            })
                            .collect::<anyhow::Result<_>>()?,
                    })
                })
                .collect::<anyhow::Result<_>>()?,
        };

        Self::from_core_state(td_core::GameConfigState {
            player,
            towers,
            monsters,
        })
        .ok_or_else(|| anyhow::anyhow!("config contains duplicate or unknown content kinds"))
    }

    fn to_file(&self) -> anyhow::Result<FileGameConfig> {
        let player = FilePlayerConfig {
            max_hp: amount_value(self.player.max_hp_raw),
            starting_gold: self.player.starting_gold,
            starting_hp: amount_value(self.player.starting_hp_raw),
            base_dice_chance: self.player.base_dice_chance,
            max_stages: self.player.max_stages,
            base_hand_slots: self.player.base_hand_slots,
        };
        let mut tower_stats = IndexMap::new();
        for entry in &self.towers.entries {
            let key = td_core::tower_kind_key(entry.kind)
                .ok_or_else(|| anyhow::anyhow!("unknown tower kind {}", entry.kind))?;
            tower_stats.insert(
                key.to_string(),
                FileTowerStats {
                    damage: amount_value(entry.damage_raw),
                    range: distance_value(entry.range_raw),
                    cooldown_ms: entry.cooldown_ms,
                },
            );
        }
        let mut monster_stats = IndexMap::new();
        for entry in &self.monsters.stats {
            let key = td_core::monster_kind_key(entry.kind)
                .ok_or_else(|| anyhow::anyhow!("unknown monster kind {}", entry.kind))?;
            monster_stats.insert(
                key.to_string(),
                FileMonsterStats {
                    base_hp: amount_value(entry.base_hp_raw),
                    velocity_mul: ratio_value(entry.velocity_mul_raw),
                    damage: amount_value(entry.damage_raw),
                    reward: entry.reward,
                },
            );
        }
        let stage_waves = self
            .monsters
            .stage_waves
            .iter()
            .map(|wave| {
                Ok(FileStageWave {
                    stage: wave.stage,
                    entries: wave
                        .entries
                        .iter()
                        .map(|entry| {
                            Ok(FileStageWaveEntry {
                                kind: td_core::monster_kind_key(entry.kind)
                                    .ok_or_else(|| {
                                        anyhow::anyhow!("unknown monster kind {}", entry.kind)
                                    })?
                                    .to_string(),
                                count: entry.count,
                            })
                        })
                        .collect::<anyhow::Result<_>>()?,
                })
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(FileGameConfig {
            player,
            monsters: FileMonsterConfig {
                stats: monster_stats,
                stage_waves,
            },
            towers: FileTowerConfig { stats: tower_stats },
        })
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

pub fn config_digest(config: &GameConfig) -> String {
    let file = config
        .to_file()
        .expect("simulator config must be serializable");
    let serialized = toml::to_string(&file).expect("simulator config TOML must serialize");
    Sha256::digest(serialized.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn valid_unique_kinds<I>(entries: I) -> bool
where
    I: IntoIterator<Item = (u8, Option<&'static str>)>,
{
    let mut seen = HashSet::new();
    entries
        .into_iter()
        .all(|(raw, key)| key.is_some() && seen.insert(raw))
}

fn tower_kind_from_key(key: &str) -> anyhow::Result<u8> {
    td_core::TOWER_KIND_KEYS
        .iter()
        .position(|candidate| *candidate == key)
        .map(|kind| kind as u8)
        .ok_or_else(|| anyhow::anyhow!("unknown tower kind {key}"))
}

fn monster_kind_from_key(key: &str) -> anyhow::Result<u8> {
    td_core::MONSTER_KIND_KEYS
        .iter()
        .position(|candidate| *candidate == key)
        .map(|kind| kind as u8)
        .ok_or_else(|| anyhow::anyhow!("unknown monster kind {key}"))
}

fn scaled_raw(value: f64, scale: i64, field: &str) -> anyhow::Result<i64> {
    if !value.is_finite() || value < 0.0 {
        bail!("{field} must be finite and non-negative");
    }
    let scaled = value * scale as f64;
    if !scaled.is_finite() || scaled > i64::MAX as f64 {
        bail!("{field} is out of range");
    }
    Ok((scaled.floor() + f64::from((scaled.fract() >= 0.5) as u8)) as i64)
}

fn amount_raw(value: f64, field: &str) -> anyhow::Result<i64> {
    scaled_raw(value, AMOUNT_SCALE, field)
}

fn ratio_raw(value: f64, field: &str) -> anyhow::Result<i64> {
    scaled_raw(value, RATIO_SCALE, field)
}

fn distance_raw(value: f64, field: &str) -> anyhow::Result<i64> {
    scaled_raw(value, WORLD_UNITS_PER_TILE, field)
}

fn amount_value(raw: i64) -> f64 {
    raw as f64 / AMOUNT_SCALE as f64
}

fn ratio_value(raw: i64) -> f64 {
    raw as f64 / RATIO_SCALE as f64
}

fn distance_value(raw: i64) -> f64 {
    raw as f64 / WORLD_UNITS_PER_TILE as f64
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FileGameConfig {
    player: FilePlayerConfig,
    monsters: FileMonsterConfig,
    towers: FileTowerConfig,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FilePlayerConfig {
    max_hp: f64,
    starting_gold: usize,
    starting_hp: f64,
    base_dice_chance: usize,
    max_stages: usize,
    base_hand_slots: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FileMonsterConfig {
    stats: IndexMap<String, FileMonsterStats>,
    stage_waves: Vec<FileStageWave>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FileMonsterStats {
    base_hp: f64,
    velocity_mul: f64,
    damage: f64,
    reward: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FileStageWave {
    stage: usize,
    entries: Vec<FileStageWaveEntry>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FileStageWaveEntry {
    kind: String,
    count: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FileTowerConfig {
    stats: IndexMap<String, FileTowerStats>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FileTowerStats {
    damage: f64,
    range: f64,
    cooldown_ms: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_config_converts_to_raw_state_and_back() {
        let config = GameConfig::default_config();
        assert_eq!(config.player.max_hp_raw, 60_000);
        assert_eq!(
            config
                .monsters
                .stats
                .iter()
                .find(|entry| entry.kind == 0)
                .expect("Mob01 config")
                .base_hp_raw,
            67_657
        );
        assert_eq!(config.towers.entries.len(), td_core::TOWER_KIND_KEYS.len());

        let restored =
            GameConfig::from_core_state(config.to_core_state()).expect("config should restore");
        assert_eq!(restored, config);
    }

    #[test]
    fn unknown_toml_kind_is_rejected() {
        let content = format!(
            "{EMBEDDED_GAMECONFIG_TOML}\n[towers.stats.FutureTower]\ndamage = 1.0\nrange = 1.0\ncooldown_ms = 1\n"
        );
        let error = GameConfig::from_toml_str(&content).expect_err("unknown kind should fail");
        assert!(error.to_string().contains("unknown tower kind"));
    }
}
