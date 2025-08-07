use crate::cli::NamuiTarget;
use crate::*;
use anyhow::Ok;
use std::path::PathBuf;

pub fn test(target: NamuiTarget, manifest_path: PathBuf) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        {
            use super::linux;

            match target {
                NamuiTarget::Wasm32WasiWeb => linux::wasm32_wasi_web::test(&manifest_path)?,
                NamuiTarget::X86_64PcWindowsMsvc => {
                    linux::x86_64_pc_windows_msvc::test(&manifest_path)?
                }
                NamuiTarget::X86_64UnknownLinuxGnu => {
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
                NamuiTarget::Wasm32WasiWeb => macos::wasm32_wasi_web::test(&manifest_path)?,
                NamuiTarget::Aarch64AppleDarwin => {
                    macos::aarch64_apple_darwin::test(&manifest_path)?
                }
                _ => unimplemented!(),
            }
        }
    } else {
        bail!("{} is unsupported os", std::env::consts::OS)
    }

    Ok(())
}
