use crate::*;
use anyhow::*;
use std::{
    fs::{self, create_dir_all},
    path::PathBuf,
};

pub fn set_user_config(target: &Target) -> Result<()> {
    let user_config_path = get_user_config_path()?;
    ensure_user_config_dir(&user_config_path)?;
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
    let namui_user_config = NamuiUserConfig {
        cfg_map,
        target: target.clone(),
    };

    fs::write(
        &user_config_path,
        &serde_json::to_string_pretty(&namui_user_config)
            .map_err(|error| anyhow!("namui user config stringify error: {}", error))?,
    )?;
    Ok(())
}

fn ensure_user_config_dir(user_config_path: &PathBuf) -> Result<(), std::io::Error> {
    let mut user_config_dir = user_config_path.clone();
    user_config_dir.pop();
    create_dir_all(user_config_dir)
}
