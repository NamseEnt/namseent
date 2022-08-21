use namui_user_config::get_namui_user_config;

pub fn print_namui_target() -> Result<(), crate::Error> {
    let namui_user_config = get_namui_user_config()?;
    println!("{}", namui_user_config.target);
    Ok(())
}
