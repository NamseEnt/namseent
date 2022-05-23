use super::NamuiUserConfig;
use crate::get_user_config_path;
use std::fs;

pub fn get_namui_user_config() -> Result<NamuiUserConfig, Box<dyn std::error::Error>> {
    let namui_user_config_path = get_user_config_path()?;
    let namui_user_config: NamuiUserConfig = match namui_user_config_path.exists() {
        true => {
            let file = fs::read(&namui_user_config_path)?;
            serde_json::from_slice(&file)?
        }
        false => NamuiUserConfig::default(),
    };
    Ok(namui_user_config)
}
