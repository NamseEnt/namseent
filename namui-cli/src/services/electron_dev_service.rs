use crate::util::get_electron_root_path;
use std::{
    path::PathBuf,
    process::{Child, Command, Stdio},
};

pub enum CrossPlatform {
    WslToWindows,
    None,
}

pub fn start_electron_dev_service(
    port: &u16,
    cross_platform: CrossPlatform,
    project_root_path: &PathBuf,
) -> Result<Child, String> {
    Command::new("npm")
        .current_dir(get_electron_root_path())
        .args([
            "run",
            match cross_platform {
                CrossPlatform::WslToWindows => "start:windows",
                CrossPlatform::None => "start",
            },
            format!("port={}", port.to_string()).as_str(),
            format!("resourceRoot={}", project_root_path.to_str().unwrap_or("")).as_str(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| error.to_string())
}
