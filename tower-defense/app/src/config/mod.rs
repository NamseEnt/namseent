pub mod monsters;
pub mod towers;

use self::monsters::MonsterConfig;
use self::towers::TowerConfig;
use crate::Health;
use anyhow::Context;
use namui::*;

pub const DEFAULT_BASE_DICE_CHANCE: usize = 3;
pub const GAME_CONFIG_VERSION: u32 = 1;

const EMBEDDED_GAMECONFIG_TOML: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/gameconfig.toml"));

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub struct GameConfig {
    pub player: PlayerConfig,
    pub monsters: MonsterConfig,
    pub towers: TowerConfig,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, State)]
pub struct PlayerConfig {
    pub max_hp: Health,
    pub starting_gold: usize,
    pub starting_hp: Health,
    pub base_dice_chance: usize,
    pub max_stages: usize,
    pub base_hand_slots: usize,
}

impl PlayerConfig {
    pub(crate) fn to_core_state(&self) -> td_core::PlayerConfigState {
        td_core::PlayerConfigState {
            max_hp_raw: self.max_hp.raw(),
            starting_gold: self.starting_gold,
            starting_hp_raw: self.starting_hp.raw(),
            base_dice_chance: self.base_dice_chance,
            max_stages: self.max_stages,
            base_hand_slots: self.base_hand_slots,
        }
    }

    pub(crate) fn from_core_state(state: td_core::PlayerConfigState) -> Self {
        Self {
            max_hp: Health::from_raw(state.max_hp_raw),
            starting_gold: state.starting_gold,
            starting_hp: Health::from_raw(state.starting_hp_raw),
            base_dice_chance: state.base_dice_chance,
            max_stages: state.max_stages,
            base_hand_slots: state.base_hand_slots,
        }
    }
}

impl GameConfig {
    pub fn to_core_state(&self) -> td_core::GameConfigState {
        td_core::GameConfigState {
            player: self.player.to_core_state(),
            towers: self.towers.to_core_state(),
            monsters: self.monsters.to_core_state(),
        }
    }

    pub fn from_core_state(state: td_core::GameConfigState) -> Option<Self> {
        Some(Self {
            player: PlayerConfig::from_core_state(state.player),
            towers: TowerConfig::from_core_state(state.towers)?,
            monsters: MonsterConfig::from_core_state(state.monsters)?,
        })
    }

    pub fn from_toml<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;
        let config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.as_ref().display()))?;
        Ok(config)
    }

    pub fn from_toml_str(toml_str: &str) -> anyhow::Result<Self> {
        let mut config: Self =
            toml::from_str(toml_str).context("Failed to parse GameConfig TOML")?;
        config = GameConfig::from_core_state(config.to_core_state())
            .expect("parsed game config must have valid unique kinds");
        Ok(config)
    }

    pub fn default_config() -> Self {
        Self::from_toml_str(EMBEDDED_GAMECONFIG_TOML)
            .expect("Failed to parse embedded gameconfig.toml")
    }

    pub fn write_toml<P: AsRef<std::path::Path>>(&self, path: P) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(self).context("Failed to serialize config to TOML")?;
        std::fs::write(path.as_ref(), content)
            .with_context(|| format!("Failed to write config file: {}", path.as_ref().display()))?;
        Ok(())
    }
}

impl Default for GameConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MonsterKind;

    #[test]
    fn serialize_default_config_deterministically() -> anyhow::Result<()> {
        let config = GameConfig::default_config();
        let a = toml::to_string_pretty(&config)?;
        let b = toml::to_string_pretty(&config)?;
        assert_eq!(a, b);
        Ok(())
    }

    #[test]
    fn default_base_dice_chance_matches_config_default() {
        let config = GameConfig::default_config();
        assert_eq!(config.player.base_dice_chance, DEFAULT_BASE_DICE_CHANCE);
    }

    #[test]
    fn decimal_config_values_have_stable_fixed_point_raw_values() {
        let config = GameConfig::default_config();
        assert_eq!(config.player.max_hp.raw(), 60_000);
        assert_eq!(config.player.starting_hp.raw(), 60_000);
        assert_eq!(
            config.monsters.stats[&MonsterKind::Mob01].base_hp.raw(),
            67_657
        );
        assert_eq!(
            config.monsters.stats[&MonsterKind::Mob02].base_hp.raw(),
            80_455
        );
        assert_eq!(
            config.monsters.stats[&MonsterKind::Mob02].damage.raw(),
            2_000
        );
    }

    #[test]
    fn player_config_raw_state_round_trips_without_host_types() {
        let player = GameConfig::default_config().player;
        let state = player.to_core_state();
        let restored = PlayerConfig::from_core_state(state);

        assert_eq!(restored.max_hp, player.max_hp);
        assert_eq!(restored.starting_gold, player.starting_gold);
        assert_eq!(restored.starting_hp, player.starting_hp);
        assert_eq!(restored.base_dice_chance, player.base_dice_chance);
        assert_eq!(restored.max_stages, player.max_stages);
        assert_eq!(restored.base_hand_slots, player.base_hand_slots);
    }
}
