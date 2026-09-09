mod legacy;
pub(crate) use legacy::LegacyGameConfig;

pub use td_core::GameConfig;

pub const DEFAULT_BASE_DICE_CHANCE: usize = 3;
pub const GAME_CONFIG_VERSION: u32 = td_core::CORE_CONFIG_SCHEMA_VERSION;

pub fn default_config() -> GameConfig {
    GameConfig::default_config()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_default_config_deterministically() -> Result<(), String> {
        let config = GameConfig::default_config();
        let a = config.to_jsonc_string()?;
        let b = config.to_jsonc_string()?;
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
        assert_eq!(config.player.max_hp_raw, 60_000);
        assert_eq!(config.player.starting_hp_raw, 60_000);
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
        assert_eq!(
            config
                .monsters
                .stats
                .iter()
                .find(|entry| entry.kind == 1)
                .expect("Mob02 config")
                .base_hp_raw,
            80_455
        );
        assert_eq!(
            config
                .monsters
                .stats
                .iter()
                .find(|entry| entry.kind == 1)
                .expect("Mob02 config")
                .damage_raw,
            2_000
        );
    }

    #[test]
    fn jsonc_accepts_comments_and_trailing_commas() -> Result<(), String> {
        let mut content =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../gameconfig.jsonc")).to_string();
        let closing_brace = content
            .rfind("\n}")
            .expect("embedded config should end with a root object");
        content.insert_str(closing_brace, "\n  // JSONC comment");

        let parsed = GameConfig::from_jsonc_str(&content)?;
        assert_eq!(parsed, GameConfig::default_config());
        Ok(())
    }
}
