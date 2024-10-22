use crate::*;
use anyhow::*;
use std::{
    fs::{self, create_dir_all},
    path::Path,
};

pub fn set_user_config(target: Target) -> Result<()> {
    let user_config_path = get_user_config_path()?;
    ensure_user_config_dir(&user_config_path)?;
    let cfg_map: NamuiCfgMap = match target {
        Target::Wasm32WasiWeb => [
            ("target_os", "unknown"),
            ("target_env", "web"),
            ("target_arch", "wasm"),
        ],
        Target::X86_64PcWindowsMsvc => [
            ("target_os", "windows"),
            ("target_env", "msvc"),
            ("target_arch", "x86_64"),
        ],
        Target::X86_64UnknownLinuxGnu => [
            ("target_os", "linux"),
            ("target_env", "gnu"),
            ("target_arch", "x86_64"),
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
        serde_json::to_string_pretty(&namui_user_config)
            .map_err(|error| anyhow!("namui user config stringify error: {}", error))?,
    )?;
    Ok(())
}

fn ensure_user_config_dir(user_config_path: &Path) -> Result<(), std::io::Error> {
    let mut user_config_dir = user_config_path.to_path_buf();
    user_config_dir.pop();
    create_dir_all(user_config_dir)
}
