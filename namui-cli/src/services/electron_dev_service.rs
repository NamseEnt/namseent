use crate::util::get_electron_root_path;
use crate::*;
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
    deep_link_schemes: &Vec<String>,
) -> Result<Child> {
    let mut args = Vec::new();
    args.push("run".to_string());
    args.push(match cross_platform {
        CrossPlatform::WslToWindows => "start:windows".to_string(),
        CrossPlatform::None => "start".to_string(),
    });
    args.push(format!("port={}", port.to_string()));
    args.push(format!(
        "applicationRoot={}",
        project_root_path.to_str().unwrap_or("")
    ));
    for deep_link_scheme in deep_link_schemes {
        args.push(format!("deepLink={}", deep_link_scheme));
    }

    Command::new("npm")
        .current_dir(get_electron_root_path())
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|error| error.into())
}
