use crate::cli::Target;
use crate::*;
use std::path::PathBuf;

pub async fn start(target: &Target, manifest_path: &PathBuf) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        {
            use super::linux;
            match target {
                Target::WasmUnknownWeb => linux::wasm_unknown_web::start(&manifest_path).await?,
                Target::WasmWindowsElectron => {
                    linux::wasm_windows_electron::start(&manifest_path).await?
                }
                Target::WasmLinuxElectron => {
                    linux::wasm_linux_electron::start(&manifest_path).await?
                }
                Target::X86_64PcWindowsMsvc => bail!(
                    "{} doesn't support start directly. Try build and run manually.",
                    target
                ),
            }
        }
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            use super::windows;
            match target {
                Target::WasmUnknownWeb
                | Target::WasmWindowsElectron
                | Target::WasmLinuxElectron => {
                    bail!("{} is unsupported target", target)
                }
                Target::X86_64PcWindowsMsvc => {
                    windows::x86_64_pc_windows_msvc::start(&manifest_path).await?;
                }
            }
        }
    } else {
        bail!("{} is unsupported os", std::env::consts::OS)
    }

    Ok(())
}
