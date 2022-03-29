use crate::util::get_electron_root_path;
use std::process::{Child, Command, Stdio};
use wsl::is_wsl;

pub fn start_electron_dev_service(port: &u16) -> Result<Child, String> {
    Command::new("npm")
        .current_dir(get_electron_root_path())
        .args([
            "run",
            match is_wsl() {
                true => "start:windows",
                false => "start",
            },
            port.to_string().as_str(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| error.to_string())
}
