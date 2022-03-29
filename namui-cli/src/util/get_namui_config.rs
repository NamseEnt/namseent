use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Deserialize)]
pub struct NamuiConfig {
    pub resources: Vec<(String, String)>,
}

pub fn get_namui_config(project_root_path: &PathBuf) -> Result<NamuiConfig, String> {
    let namui_config_path = project_root_path.join("namui.config.json");
    fs::read(namui_config_path)
        .map_err(|error| format!("namui config read error: {}", error))
        .and_then(|file| {
            let namui_config: NamuiConfig = serde_json::from_slice(&file)
                .map_err(|error| format!("namui config parse error: {}", error))?;
            Ok(namui_config)
        })
}
