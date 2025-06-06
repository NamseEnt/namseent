use crate::cli::Target;
use crate::*;
use anyhow::Ok;
use std::path::PathBuf;

pub fn test(target: Target, manifest_path: PathBuf) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        {
            use super::linux;

            match target {
                Target::Wasm32WasiWeb => linux::wasm32_wasi_web::test(&manifest_path)?,
                Target::X86_64PcWindowsMsvc => linux::x86_64_pc_windows_msvc::test(&manifest_path)?,
                Target::X86_64UnknownLinuxGnu => {
                    linux::x86_64_unknown_linux_gnu::test(&manifest_path)?
                }
                _ => unimplemented!(),
            }
        }
    } else if cfg!(target_os = "macos") {
        #[cfg(target_os = "macos")]
        {
            use super::macos;

            match target {
                Target::Wasm32WasiWeb => macos::wasm32_wasi_web::test(&manifest_path)?,
                Target::Aarch64AppleDarwin => macos::aarch64_apple_darwin::test(&manifest_path)?,
                _ => unimplemented!(),
            }
        }
    } else {
        bail!("{} is unsupported os", std::env::consts::OS)
    }

    Ok(())
}
