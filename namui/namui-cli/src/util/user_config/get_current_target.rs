use crate::cli::NamuiTarget;
use crate::*;
use namui_user_config::get_namui_user_config;

pub fn get_current_target() -> Result<NamuiTarget> {
    let namui_user_config = get_namui_user_config()?;
    Ok(namui_user_config.target.into())
}
