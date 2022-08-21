use crate::cli::Target;
use namui_user_config::get_namui_user_config;

pub fn get_current_target() -> Result<Target, crate::Error> {
    let namui_user_config = get_namui_user_config()?;
    Ok(namui_user_config.target.into())
}
