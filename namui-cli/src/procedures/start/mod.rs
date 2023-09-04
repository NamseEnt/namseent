use crate::cli::Target;
use crate::*;
use std::path::PathBuf;

pub async fn start(target: &Target, manifest_path: &PathBuf) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        use super::linux;
        match target {
            Target::WasmUnknownWeb => linux::wasm_unknown_web::start(&manifest_path).await,
            Target::WasmWindowsElectron => {
                linux::wasm_windows_electron::start(&manifest_path).await
            }
            Target::WasmLinuxElectron => linux::wasm_linux_electron::start(&manifest_path).await,
        }
    } else {
        Result::Err(anyhow!("{} is unsupported os", std::env::consts::OS))
    }
}
