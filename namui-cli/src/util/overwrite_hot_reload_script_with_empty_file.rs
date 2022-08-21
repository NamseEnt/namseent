use std::{fs::write, path::PathBuf};

pub fn overwrite_hot_reload_script_with_empty_file(
    release_path: &PathBuf,
) -> Result<(), crate::Error> {
    const HOT_RELOAD_SCRIPT_NAME: &str = "hotReload.js";
    let hot_reload_script_path = release_path.join(&HOT_RELOAD_SCRIPT_NAME);
    if hot_reload_script_path.exists() {
        write(hot_reload_script_path, "")
            .map_err(|error| format!("overwrite_hot_reload_script fail: {}", error))?;
    }
    Ok(())
}
