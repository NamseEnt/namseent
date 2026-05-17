pub mod monsters;
pub mod towers;

use self::monsters::MonsterConfig;
use self::towers::TowerConfig;

use anyhow::Context;

use namui::*;

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
    pub max_hp: f32,
    pub starting_gold: usize,
    pub starting_hp: f32,
    pub base_dice_chance: usize,
    pub max_stages: usize,
    pub base_hand_slots: usize,
}

impl GameConfig {
    pub fn from_toml<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;
        let config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.as_ref().display()))?;
        Ok(config)
    }

    pub fn from_toml_str(toml_str: &str) -> anyhow::Result<Self> {
        toml::from_str(toml_str).context("Failed to parse GameConfig TOML")
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

#[cfg(all(test, feature = "simulator"))]
mod tests {
    use super::*;

    #[test]
    fn parse_gameconfig_example() -> anyhow::Result<()> {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("gameconfig.toml.example");
        GameConfig::from_toml(path)?;
        Ok(())
    }

    #[test]
    fn serialize_default_config_deterministically() -> anyhow::Result<()> {
        let config = GameConfig::default_config();
        let a = toml::to_string_pretty(&config)?;
        let b = toml::to_string_pretty(&config)?;
        assert_eq!(a, b);
        Ok(())
    }
}
