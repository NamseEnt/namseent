use crate::cli::Target;
use crate::*;
use anyhow::Ok;
use std::path::PathBuf;

pub fn test(target: &Target, manifest_path: &PathBuf) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    if cfg!(target_os = "linux") {
        #[cfg(target_os = "linux")]
        {
            use super::linux;

            match target {
                Target::WasmUnknownWeb => linux::wasm_unknown_web::test(&manifest_path)?,
                _ => unimplemented!(),
            }
        }
    } else {
        bail!("{} is unsupported os", std::env::consts::OS)
    }

    Ok(())
}
