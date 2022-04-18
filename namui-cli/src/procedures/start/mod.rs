use crate::cli::Target;
use std::{error::Error, path::PathBuf};

pub fn start(target: &Target, manifest_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        use super::linux;
        match target {
            Target::WasmUnknownWeb => linux::wasm_unknown_web::start(&manifest_path),
            Target::WasmWindowsElectron => linux::wasm_windows_electron::start(&manifest_path),
            Target::WasmLinuxElectron => linux::wasm_linux_electron::start(&manifest_path),
        }
    } else {
        Result::Err(format!("{} is unsupported os", std::env::consts::OS).into())
    }
}
