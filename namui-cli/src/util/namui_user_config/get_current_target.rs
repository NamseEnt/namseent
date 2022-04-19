use super::get_namui_user_config;
use crate::cli::Target;
use std::error::Error;

pub fn get_current_target() -> Result<Target, Box<dyn Error>> {
    let namui_user_config = get_namui_user_config()?;
    Ok(namui_user_config.target)
}
