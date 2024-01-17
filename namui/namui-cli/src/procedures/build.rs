use crate::*;
use crate::{cli::Target, services::electron_package_service};
use std::path::PathBuf;

pub async fn build(
    target: &Target,
    manifest_path: &PathBuf,
    arch: Option<electron_package_service::Arch>,
) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        {
            use super::linux;
            match target {
                Target::WasmUnknownWeb => linux::wasm_unknown_web::build(&manifest_path).await?,
                Target::WasmWindowsElectron => {
                    linux::wasm_windows_electron::build(&manifest_path, arch).await?
                }
                Target::WasmLinuxElectron => {
                    linux::wasm_linux_electron::build(&manifest_path, arch).await?
                }
                Target::X86_64PcWindowsMsvc => {
                    linux::x86_64_pc_windows_msvc::build(&manifest_path).await?
                }
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
                    windows::x86_64_pc_windows_msvc::build(&manifest_path).await?;
                }
            }
        }
    } else {
        bail!("{} is unsupported os", std::env::consts::OS)
    }

    Ok(())
}
