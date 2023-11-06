use crate::*;
use namui_user_config::get_namui_user_config;

pub fn print_namui_cfg() -> Result<()> {
    let namui_user_config = get_namui_user_config()?;
    for (key, value) in namui_user_config.cfg_map {
        println!("{}={}", key, value);
    }
    Ok(())
}
