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
            }
        }
    } else {
        bail!("{} is unsupported os", std::env::consts::OS)
    }

    Ok(())
}
