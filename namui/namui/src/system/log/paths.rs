use std::path::PathBuf;

#[cfg(target_os = "macos")]
#[allow(dead_code)]
pub(crate) fn default_log_dir(app_name: &str) -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join("Library/Logs").join(app_name))
}

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "wasi")))]
#[allow(dead_code)]
pub(crate) fn default_log_dir(app_name: &str) -> Option<PathBuf> {
    if let Some(state) = std::env::var_os("XDG_STATE_HOME") {
        return Some(PathBuf::from(state).join(app_name).join("logs"));
    }
    let home = std::env::var_os("HOME")?;
    Some(
        PathBuf::from(home)
            .join(".local/state")
            .join(app_name)
            .join("logs"),
    )
}

#[cfg(windows)]
#[allow(dead_code)]
pub(crate) fn default_log_dir(app_name: &str) -> Option<PathBuf> {
    let base = std::env::var_os("LOCALAPPDATA")?;
    Some(PathBuf::from(base).join(app_name).join("logs"))
}

#[cfg(target_os = "wasi")]
#[allow(dead_code)]
pub(crate) fn default_log_dir(_app_name: &str) -> Option<PathBuf> {
    None
}
