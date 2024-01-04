use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn recreate_dir_all(path: impl AsRef<Path>, excludes: Option<Vec<PathBuf>>) -> Result<()> {
    let path = path.as_ref();
    if path.exists() {
        match exclude {
            Some(exclude) => {
                walkdir::WalkDir::new(path)
                    .into_iter()
                    .filter_map(|entry| entry.ok())
                    .filter(|entry| {
                        !exclude.iter().any(|exclude| {
                            entry.path().starts_with(exclude) || exclude.starts_with(entry.path())
                        })
                    })
                    .try_for_each(|entry| {
                        if entry.file_type().is_dir() {
                            std::fs::remove_dir_all(entry.path())?;
                        } else {
                            std::fs::remove_file(entry.path())?;
                        }
                        Result::<()>::Ok(())
                    })?;
            }
            None => {
                std::fs::remove_dir_all(path)?;
            }
        }
    }
    std::fs::create_dir_all(path)?;
    Ok(())
}
