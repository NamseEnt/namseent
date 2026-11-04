use sha2::{Digest, Sha256};

pub use td_core::GameConfig;

pub const GAME_CONFIG_VERSION: u32 = td_core::CORE_CONFIG_SCHEMA_VERSION;
pub const CONFIG_DIGEST_VERSION: u32 = 1;

pub fn load_jsonc<P: AsRef<std::path::Path>>(path: P) -> anyhow::Result<GameConfig> {
    GameConfig::from_jsonc(path).map_err(anyhow::Error::msg)
}

pub fn load_default_config() -> anyhow::Result<GameConfig> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../gameconfig.jsonc");
    load_jsonc(path)
}

pub fn config_digest(config: &GameConfig) -> String {
    let serialized = config
        .config_digest_input()
        .expect("validated GameConfig must be serializable");
    Sha256::digest(serialized.as_bytes())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

#[cfg(test)]
pub(crate) trait GameConfigTestExt {
    fn default_config() -> Self;
}

#[cfg(test)]
impl GameConfigTestExt for GameConfig {
    fn default_config() -> Self {
        GameConfig::from_jsonc_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../gameconfig.jsonc"
        )))
        .expect("test gameconfig.jsonc must be valid")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_config_converts_to_core_state_and_back() {
        let config = GameConfig::default_config();
        assert_eq!(config.player.max_hp_raw, 60_000);
        assert!(
            config
                .monsters
                .stats
                .iter()
                .all(|entry| entry.base_hp_raw > 0)
        );
        assert_eq!(config.towers.entries.len(), td_core::TOWER_KIND_KEYS.len());

        let restored =
            GameConfig::from_core_state(config.to_core_state()).expect("config should restore");
        assert_eq!(restored, config);
    }

    #[test]
    fn default_config_is_loaded_from_the_project_file() {
        assert_eq!(
            load_default_config().expect("project config should load"),
            GameConfig::default_config()
        );
    }

    #[test]
    fn unknown_jsonc_kind_is_rejected() {
        let source = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../gameconfig.jsonc"));
        let content = source.replacen("\"RubberCone\"", "\"FutureTower\"", 1);
        let error = GameConfig::from_jsonc_str(&content).expect_err("unknown kind should fail");
        assert!(error.contains("unknown tower kind"));
    }

    #[test]
    fn jsonc_accepts_comments_and_trailing_commas() {
        let mut content =
            include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../gameconfig.jsonc")).to_string();
        let closing_brace = content
            .rfind("\n}")
            .expect("embedded config should end with a root object");
        content.insert_str(closing_brace, "\n  // JSONC comment");

        let parsed = GameConfig::from_jsonc_str(&content).expect("JSONC should parse");
        assert_eq!(parsed, GameConfig::default_config());
    }
}
