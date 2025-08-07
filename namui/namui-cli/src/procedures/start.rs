use crate::cli::NamuiTarget;
use crate::*;
use std::path::PathBuf;

pub async fn start(
    target: NamuiTarget,
    manifest_path: PathBuf,
    start_option: StartOption,
) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        {
            use super::linux;
            match target {
                NamuiTarget::Wasm32WasiWeb => {
                    linux::wasm32_wasi_web::start(&manifest_path, start_option).await?
                }
                NamuiTarget::X86_64PcWindowsMsvc => {
                    linux::x86_64_pc_windows_msvc::start(&manifest_path, start_option).await?
                }
                _ => unimplemented!(),
            }
        }
    } else if cfg!(target_os = "macos") {
        #[cfg(target_os = "macos")]
        {
            use super::macos;
            match target {
                NamuiTarget::Wasm32WasiWeb => {
                    macos::wasm32_wasi_web::start(&manifest_path, start_option).await?
                }
                _ => unimplemented!(),
            }
        }
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            use super::windows;
            match target {
                NamuiTarget::Wasm32WasiWeb
                | NamuiTarget::WasmWindowsElectron
                | NamuiTarget::WasmLinuxElectron => {
                    bail!("{} is unsupported target", target)
                }
                NamuiTarget::X86_64PcWindowsMsvc => {
                    windows::x86_64_pc_windows_msvc::start(&manifest_path).await?;
                }
                _ => unimplemented!(),
            }
        }
    } else {
        bail!("{} is unsupported os", std::env::consts::OS)
    }

    Ok(())
}
