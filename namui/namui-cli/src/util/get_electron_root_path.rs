use super::get_cli_root_path;
use std::path::PathBuf;

#[allow(dead_code)]
pub fn get_electron_root_path() -> PathBuf {
    get_cli_root_path().join("electron")
}
