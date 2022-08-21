use crate::{cli::Target, services::electron_package_service};
use std::path::PathBuf;

pub fn build(
    target: &Target,
    manifest_path: &PathBuf,
    arch: Option<electron_package_service::Arch>,
) -> Result<(), crate::Error> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        use super::linux;
        match target {
            Target::WasmUnknownWeb => linux::wasm_unknown_web::build(&manifest_path),
            Target::WasmWindowsElectron => {
                linux::wasm_windows_electron::build(&manifest_path, arch)
            }
            Target::WasmLinuxElectron => linux::wasm_linux_electron::build(&manifest_path, arch),
        }
    } else {
        Result::Err(format!("{} is unsupported os", std::env::consts::OS).into())
    }
}
