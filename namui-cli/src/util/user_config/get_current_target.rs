use crate::cli::Target;
use crate::*;
use namui_user_config::get_namui_user_config;

pub fn get_current_target() -> Result<Target> {
    let namui_user_config = get_namui_user_config()?;
    Ok(namui_user_config.target.into())
}
