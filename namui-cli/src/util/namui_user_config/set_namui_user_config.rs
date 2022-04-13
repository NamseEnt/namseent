use super::NamuiUserConfig;
use crate::{
    cli::Target,
    util::{get_cli_root_path, NamuiCfgMap},
};
use std::fs;

pub fn set_namui_user_config(target: &Target) -> Result<(), Box<dyn std::error::Error>> {
    let namui_user_config_path = get_cli_root_path().join("namui_user_config.json");
    let cfg_map: NamuiCfgMap = match target {
        Target::WasmUnknownWeb => [
            ("target_os", "unknown"),
            ("target_env", "web"),
            ("target_arch", "wasm"),
        ],
        Target::WasmWindowsElectron => [
            ("target_os", "windows"),
            ("target_env", "electron"),
            ("target_arch", "wasm"),
        ],
        Target::WasmLinuxElectron => [
            ("target_os", "linux"),
            ("target_env", "electron"),
            ("target_arch", "wasm"),
        ],
    }
    .iter()
    .map(|(key, value)| (key.to_string(), value.to_string()))
    .collect();
    let namui_user_config = NamuiUserConfig { cfg_map };

    fs::write(
        &namui_user_config_path,
        &serde_json::to_string_pretty(&namui_user_config)
            .map_err(|error| format!("namui user config stringify error: {}", error))?,
    )?;

    println!("Settings have been saved.");
    Ok(())
}
