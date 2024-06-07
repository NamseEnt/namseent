use crate::cli::Target;
use crate::*;
use std::path::PathBuf;

pub async fn build(target: Target, manifest_path: PathBuf, release: bool) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        {
            use super::linux;
            match target {
                Target::Wasm32WasiWeb => {
                    linux::wasm32_wasi_web::build(&manifest_path, release).await?
                }
                Target::X86_64PcWindowsMsvc => {
                    linux::x86_64_pc_windows_msvc::build(&manifest_path, release).await?
                }
            }
        }
    } else if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            use super::windows;
            match target {
                Target::Wasm32WasiWeb => {
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
