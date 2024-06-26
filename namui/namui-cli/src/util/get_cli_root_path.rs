use anyhow::Result;
use std::{env::current_exe, fs::read_dir, path::PathBuf};

pub fn get_cli_root_path() -> PathBuf {
    let mut exe_path = real_current_exe_path().unwrap();
    exe_path.pop();
    for ancestor in exe_path.ancestors() {
        let cargo_toml_exist = check_cargo_toml_exist(ancestor);
        if cargo_toml_exist {
            return ancestor.into();
        }
    }
    panic!("Could not found cli_root_path");
}

fn real_current_exe_path() -> Result<PathBuf> {
    let current_exe = current_exe()?;
    let Ok(metadata) = std::fs::symlink_metadata(&current_exe) else {
        return Ok(current_exe);
    };

    if metadata.file_type().is_symlink() {
        return Ok(std::fs::read_link(current_exe)?);
    }

    Ok(current_exe)
}

fn check_cargo_toml_exist(path: impl AsRef<std::path::Path>) -> bool {
    read_dir(path)
        .unwrap()
        .any(|dirent| dirent.unwrap().file_name() == "Cargo.toml")
}
