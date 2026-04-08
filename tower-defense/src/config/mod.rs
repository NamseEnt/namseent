pub mod items;
pub mod monsters;
pub mod shop;
pub mod towers;
pub mod upgrades;

use self::items::ItemConfig;
use self::monsters::MonsterConfig;
use self::shop::ShopConfig;
use self::towers::TowerConfig;
use self::upgrades::UpgradeConfig;

#[cfg(feature = "simulator")]
use anyhow::Context;

use namui::*;

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct GameConfig {
    pub player: PlayerConfig,
    pub monsters: MonsterConfig,
    pub towers: TowerConfig,
    pub items: ItemConfig,
    pub shop: ShopConfig,
    pub upgrades: UpgradeConfig,
    pub rarity_weights: [usize; 4],
}

#[cfg_attr(feature = "simulator", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, State)]
pub struct PlayerConfig {
    pub max_hp: f32,
    pub starting_gold: usize,
    pub starting_hp: f32,
    pub base_dice_chance: usize,
    pub max_stages: usize,
    pub base_hand_slots: usize,
}

impl GameConfig {
    /// Returns the default configuration matching current hardcoded values.
    pub fn default_config() -> Self {
        Self {
            player: PlayerConfig {
                max_hp: 100.0,
                starting_gold: 100,
                starting_hp: 100.0,
                base_dice_chance: 1,
                max_stages: 50,
                base_hand_slots: 5,
            },
            monsters: monsters::default_monster_config(),
            towers: towers::default_tower_config(),
            items: items::default_item_config(),
            shop: ShopConfig {
                base_cost: 50.0,
                value_cost_multiplier: 0.5,
                slot_type_distribution: [3, 5, 2], // 30% items, 50% tower upgrades, 20% extra
            },
            upgrades: upgrades::default_upgrade_config(),
            rarity_weights: [90, 10, 1, 0],
        }
    }

    #[cfg(feature = "simulator")]
    pub fn from_toml<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path.as_ref())
            .with_context(|| format!("Failed to read config file: {}", path.as_ref().display()))?;
        let config: Self = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file: {}", path.as_ref().display()))?;
        Ok(config)
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
}
