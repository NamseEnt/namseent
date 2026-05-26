use crate::{Error, context::CrashContext};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub fn root_dir(app_name: &str) -> Result<PathBuf, Error> {
    let base = dirs::data_local_dir().ok_or(Error::NoUserDataDir)?;
    Ok(base.join(app_name).join("crashes"))
}

pub fn queue_dir(app_name: &str) -> Result<PathBuf, Error> {
    Ok(root_dir(app_name)?.join("queue"))
}

#[derive(Serialize, Deserialize)]
pub struct PendingEntry {
    pub stack_hash: String,
    pub context: CrashContext,
}

pub fn sidecar_path(dump_path: &Path) -> PathBuf {
    dump_path.with_extension("json")
}

pub fn write_sidecar(dump_path: &Path, entry: &PendingEntry) -> Result<(), Error> {
    let json = serde_json::to_vec_pretty(entry)?;
    std::fs::write(sidecar_path(dump_path), json)?;
    Ok(())
}

pub fn list_pending(app_name: &str) -> Result<Vec<PathBuf>, Error> {
    let dir = queue_dir(app_name)?;
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut dumps = Vec::new();
    for entry in std::fs::read_dir(&dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("dmp")
            && sidecar_path(&path).exists()
        {
            dumps.push(path);
        }
    }
    Ok(dumps)
}

pub fn load_sidecar(dump_path: &Path) -> Result<PendingEntry, Error> {
    let bytes = std::fs::read(sidecar_path(dump_path))?;
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn delete_entry(dump_path: &Path) -> Result<(), Error> {
    let sidecar = sidecar_path(dump_path);
    let _ = std::fs::remove_file(&sidecar);
    let _ = std::fs::remove_file(dump_path);
    Ok(())
}
