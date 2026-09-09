#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PlayerConfigState {
    pub max_hp_raw: i64,
    pub starting_gold: usize,
    pub starting_hp_raw: i64,
    pub base_dice_chance: usize,
    pub max_stages: usize,
    pub base_hand_slots: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerConfigEntryState {
    pub kind: u8,
    pub damage_raw: i64,
    pub range_raw: i64,
    pub cooldown_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TowerConfigState {
    pub entries: Vec<TowerConfigEntryState>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterConfigEntryState {
    pub kind: u8,
    pub base_hp_raw: i64,
    pub velocity_mul_raw: i64,
    pub damage_raw: i64,
    pub reward: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StageWaveEntryState {
    pub kind: u8,
    pub count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct StageWaveState {
    pub stage: usize,
    pub entries: Vec<StageWaveEntryState>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct MonsterConfigState {
    pub stats: Vec<MonsterConfigEntryState>,
    pub stage_waves: Vec<StageWaveState>,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct GameConfig {
    pub player: PlayerConfigState,
    pub towers: TowerConfigState,
    pub monsters: MonsterConfigState,
}

pub type GameConfigState = GameConfig;

impl GameConfig {
    pub fn default_config() -> Self {
        Self::from_jsonc_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../gameconfig.jsonc"
        )))
        .expect("embedded gameconfig.jsonc must be valid")
    }

    pub fn from_jsonc_str(content: &str) -> Result<Self, String> {
        let file: JsonGameConfig = json5::from_str(content)
            .map_err(|error| format!("failed to parse GameConfig: {error}"))?;
        Self::from_json_file(file)
    }

    pub fn from_jsonc<P: AsRef<std::path::Path>>(path: P) -> Result<Self, String> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)
            .map_err(|error| format!("failed to read config {}: {error}", path.display()))?;
        Self::from_jsonc_str(&content)
    }

    pub fn to_jsonc_string(&self) -> Result<String, String> {
        serde_json::to_string_pretty(&self.to_json_file()?)
            .map_err(|error| format!("failed to serialize GameConfig: {error}"))
    }

    pub fn write_jsonc<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), String> {
        let path = path.as_ref();
        std::fs::write(path, self.to_jsonc_string()?)
            .map_err(|error| format!("failed to write config {}: {error}", path.display()))
    }

    pub fn config_digest_input(&self) -> Result<String, String> {
        serde_json::to_string(&self.to_json_file()?)
            .map_err(|error| format!("failed to serialize GameConfig digest: {error}"))
    }

    pub fn from_core_state(state: Self) -> Option<Self> {
        state.validate().ok()?;
        Some(state)
    }

    pub fn to_core_state(&self) -> Self {
        self.clone()
    }

    fn from_json_file(file: JsonGameConfig) -> Result<Self, String> {
        let player = PlayerConfigState {
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
                Ok(TowerConfigEntryState {
                    kind: tower_kind_from_key(&key)?,
                    damage_raw: amount_raw(stats.damage, &format!("towers.stats.{key}.damage"))?,
                    range_raw: distance_raw(stats.range, &format!("towers.stats.{key}.range"))?,
                    cooldown_ms: stats.cooldown_ms,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        tower_entries.sort_by_key(|entry| entry.kind);

        let mut monster_stats = file
            .monsters
            .stats
            .into_iter()
            .map(|(key, stats)| {
                Ok(MonsterConfigEntryState {
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
            .collect::<Result<Vec<_>, String>>()?;
        monster_stats.sort_by_key(|entry| entry.kind);

        let stage_waves = file
            .monsters
            .stage_waves
            .into_iter()
            .map(|wave| {
                Ok(StageWaveState {
                    stage: wave.stage,
                    entries: wave
                        .entries
                        .into_iter()
                        .map(|entry| {
                            Ok(StageWaveEntryState {
                                kind: monster_kind_from_key(&entry.kind)?,
                                count: entry.count,
                            })
                        })
                        .collect::<Result<Vec<_>, String>>()?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        let config = Self {
            player,
            towers: TowerConfigState {
                entries: tower_entries,
            },
            monsters: MonsterConfigState {
                stats: monster_stats,
                stage_waves,
            },
        };
        config.validate()?;
        Ok(config)
    }

    fn to_json_file(&self) -> Result<JsonGameConfig, String> {
        let mut tower_stats = std::collections::BTreeMap::new();
        for entry in &self.towers.entries {
            let key = crate::tower_kind_key(entry.kind)
                .ok_or_else(|| format!("unknown tower kind {}", entry.kind))?;
            tower_stats.insert(
                key.to_string(),
                JsonTowerStats {
                    damage: amount_value(entry.damage_raw),
                    range: distance_value(entry.range_raw),
                    cooldown_ms: entry.cooldown_ms,
                },
            );
        }

        let mut monster_stats = std::collections::BTreeMap::new();
        for entry in &self.monsters.stats {
            let key = crate::monster_kind_key(entry.kind)
                .ok_or_else(|| format!("unknown monster kind {}", entry.kind))?;
            monster_stats.insert(
                key.to_string(),
                JsonMonsterStats {
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
                Ok(JsonStageWave {
                    stage: wave.stage,
                    entries: wave
                        .entries
                        .iter()
                        .map(|entry| {
                            Ok(JsonStageWaveEntry {
                                kind: crate::monster_kind_key(entry.kind)
                                    .ok_or_else(|| format!("unknown monster kind {}", entry.kind))?
                                    .to_string(),
                                count: entry.count,
                            })
                        })
                        .collect::<Result<Vec<_>, String>>()?,
                })
            })
            .collect::<Result<Vec<_>, String>>()?;

        Ok(JsonGameConfig {
            player: JsonPlayerConfig {
                max_hp: amount_value(self.player.max_hp_raw),
                starting_gold: self.player.starting_gold,
                starting_hp: amount_value(self.player.starting_hp_raw),
                base_dice_chance: self.player.base_dice_chance,
                max_stages: self.player.max_stages,
                base_hand_slots: self.player.base_hand_slots,
            },
            monsters: JsonMonsterConfig {
                stats: monster_stats,
                stage_waves,
            },
            towers: JsonTowerConfig { stats: tower_stats },
        })
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.player.max_stages == 0 {
            return Err("player.max_stages must be positive".to_string());
        }

        if self.player.max_stages > crate::MAX_STAGE_COUNT {
            return Err(format!(
                "player.max_stages must not exceed {}",
                crate::MAX_STAGE_COUNT
            ));
        }

        let mut tower_kinds = std::collections::HashSet::new();
        for entry in &self.towers.entries {
            if crate::tower_kind_key(entry.kind).is_none() {
                return Err(format!("unknown tower kind {}", entry.kind));
            }
            if !tower_kinds.insert(entry.kind) {
                return Err(format!("duplicate tower kind {}", entry.kind));
            }
        }

        let mut monster_stats = std::collections::HashSet::new();
        for entry in &self.monsters.stats {
            if crate::monster_kind_key(entry.kind).is_none() {
                return Err(format!("unknown monster kind {}", entry.kind));
            }
            if !monster_stats.insert(entry.kind) {
                return Err(format!("duplicate monster kind {}", entry.kind));
            }
        }

        let mut stages = std::collections::HashSet::new();
        for wave in &self.monsters.stage_waves {
            if !(1..=crate::MAX_STAGE_COUNT).contains(&wave.stage) {
                return Err(format!("stage wave {} is out of range", wave.stage));
            }
            if !stages.insert(wave.stage) {
                return Err(format!("duplicate stage wave {}", wave.stage));
            }
            if wave.entries.is_empty() {
                return Err(format!("stage wave {} has no entries", wave.stage));
            }

            let mut has_boss = false;
            for entry in &wave.entries {
                if entry.count == 0 {
                    return Err(format!("stage wave {} has a zero-count entry", wave.stage));
                }
                if !monster_stats.contains(&entry.kind) {
                    return Err(format!(
                        "stage wave {} references monster kind {} without stats",
                        wave.stage, entry.kind
                    ));
                }
                let kind_key = crate::monster_kind_key(entry.kind)
                    .expect("monster kind was checked before stage validation");
                has_boss |= kind_key.starts_with("Boss");
            }

            if crate::is_boss_stage(wave.stage) && !has_boss {
                return Err(format!("boss stage {} has no boss entry", wave.stage));
            }
            if !crate::is_boss_stage(wave.stage) && has_boss {
                return Err(format!(
                    "non-boss stage {} contains a boss entry",
                    wave.stage
                ));
            }
        }

        for stage in 1..=self.player.max_stages {
            if !stages.contains(&stage) {
                return Err(format!("missing stage wave {}", stage));
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct JsonGameConfig {
    player: JsonPlayerConfig,
    monsters: JsonMonsterConfig,
    towers: JsonTowerConfig,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct JsonPlayerConfig {
    max_hp: f64,
    starting_gold: usize,
    starting_hp: f64,
    base_dice_chance: usize,
    max_stages: usize,
    base_hand_slots: usize,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct JsonMonsterConfig {
    stats: std::collections::BTreeMap<String, JsonMonsterStats>,
    stage_waves: Vec<JsonStageWave>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct JsonMonsterStats {
    base_hp: f64,
    velocity_mul: f64,
    damage: f64,
    reward: usize,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct JsonStageWave {
    stage: usize,
    entries: Vec<JsonStageWaveEntry>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct JsonStageWaveEntry {
    kind: String,
    count: usize,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct JsonTowerConfig {
    stats: std::collections::BTreeMap<String, JsonTowerStats>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
struct JsonTowerStats {
    damage: f64,
    range: f64,
    cooldown_ms: u64,
}

const AMOUNT_SCALE: i64 = crate::AMOUNT_SCALE;
const RATIO_SCALE: i64 = crate::RATIO_SCALE;
const WORLD_UNITS_PER_TILE: i64 = crate::WORLD_UNITS_PER_TILE;

fn scaled_raw(value: f64, scale: i64, field: &str) -> Result<i64, String> {
    if !value.is_finite() || value < 0.0 {
        return Err(format!("{field} must be finite and non-negative"));
    }
    let scaled = value * scale as f64;
    if !scaled.is_finite() || scaled > i64::MAX as f64 {
        return Err(format!("{field} is out of range"));
    }
    Ok((scaled.floor() + f64::from((scaled.fract() >= 0.5) as u8)) as i64)
}

fn amount_raw(value: f64, field: &str) -> Result<i64, String> {
    scaled_raw(value, AMOUNT_SCALE, field)
}

fn ratio_raw(value: f64, field: &str) -> Result<i64, String> {
    scaled_raw(value, RATIO_SCALE, field)
}

fn distance_raw(value: f64, field: &str) -> Result<i64, String> {
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

fn tower_kind_from_key(key: &str) -> Result<u8, String> {
    crate::TOWER_KIND_KEYS
        .iter()
        .position(|candidate| *candidate == key)
        .map(|kind| kind as u8)
        .ok_or_else(|| format!("unknown tower kind {key}"))
}

fn monster_kind_from_key(key: &str) -> Result<u8, String> {
    crate::MONSTER_KIND_KEYS
        .iter()
        .position(|candidate| *candidate == key)
        .map(|kind| kind as u8)
        .ok_or_else(|| format!("unknown monster kind {key}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config_with_stage_waves(stage_waves: Vec<StageWaveState>) -> GameConfigState {
        GameConfigState {
            player: PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 100,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 10,
                base_hand_slots: 5,
            },
            towers: TowerConfigState {
                entries: Vec::new(),
            },
            monsters: MonsterConfigState {
                stats: vec![
                    MonsterConfigEntryState {
                        kind: 0,
                        base_hp_raw: 1_000,
                        velocity_mul_raw: crate::RATIO_SCALE,
                        damage_raw: 1_000,
                        reward: 1,
                    },
                    MonsterConfigEntryState {
                        kind: 50,
                        base_hp_raw: 1_000,
                        velocity_mul_raw: crate::RATIO_SCALE,
                        damage_raw: 1_000,
                        reward: 1,
                    },
                ],
                stage_waves,
            },
        }
    }

    fn valid_waves() -> Vec<StageWaveState> {
        (1..=10)
            .map(|stage| StageWaveState {
                stage,
                entries: vec![StageWaveEntryState {
                    kind: if stage == 10 { 50 } else { 0 },
                    count: 1,
                }],
            })
            .collect()
    }

    #[test]
    fn validation_rejects_missing_boss_and_duplicate_stage_waves() {
        let mut missing_boss = valid_waves();
        missing_boss[9].entries[0].kind = 0;
        assert_eq!(
            config_with_stage_waves(missing_boss)
                .validate()
                .expect_err("boss wave must contain a boss"),
            "boss stage 10 has no boss entry"
        );

        let mut duplicate = valid_waves();
        duplicate.push(duplicate[0].clone());
        assert_eq!(
            config_with_stage_waves(duplicate)
                .validate()
                .expect_err("duplicate stage must be rejected"),
            "duplicate stage wave 1"
        );
    }

    #[test]
    fn core_owns_jsonc_loading_and_round_trip() {
        let config = GameConfig::default_config();
        let encoded = config.to_jsonc_string().expect("config should serialize");
        let decoded = GameConfig::from_jsonc_str(&encoded).expect("config should parse");

        assert_eq!(decoded, config);
    }
}
