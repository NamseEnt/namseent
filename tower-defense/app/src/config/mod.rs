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
        // Pin decimal inputs: this tests fixed-point parsing, not live balance.
        let mut source: serde_json::Value =
            serde_json::from_str(&GameConfig::default_config().to_jsonc_string().unwrap()).unwrap();
        source["monsters"]["stats"]["Mob01"]["base_hp"] = serde_json::json!(35.52);
        source["monsters"]["stats"]["Mob02"]["base_hp"] = serde_json::json!(44.25);
        let config = GameConfig::from_jsonc_str(&source.to_string()).unwrap();
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
            35_520
        );
        assert_eq!(
            config
                .monsters
                .stats
                .iter()
                .find(|entry| entry.kind == 1)
                .expect("Mob02 config")
                .base_hp_raw,
            44_250
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
