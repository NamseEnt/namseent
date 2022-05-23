use directories::ProjectDirs;
use std::{error::Error, fmt::Display, path::PathBuf};

#[derive(Debug)]
pub struct GetUserConfigPathError {}
impl Display for GetUserConfigPathError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to get project directory")
    }
}
impl Error for GetUserConfigPathError {}

pub fn get_user_config_path() -> Result<PathBuf, GetUserConfigPathError> {
    match ProjectDirs::from("com", "namseent", "namui") {
        Some(project_dir) => Ok(project_dir.config_dir().join("namui_user_config.json")),
        None => Err(GetUserConfigPathError {}),
    }
}
