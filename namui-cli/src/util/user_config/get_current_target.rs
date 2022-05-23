use crate::cli::Target;
use namui_user_config::get_namui_user_config;
use std::error::Error;

pub fn get_current_target() -> Result<Target, Box<dyn Error>> {
    let namui_user_config = get_namui_user_config()?;
    Ok(namui_user_config.target.into())
}
