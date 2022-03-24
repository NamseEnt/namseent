use std::{env::current_exe, path::PathBuf};

pub fn get_cli_root_path() -> PathBuf {
    let mut exe_path = current_exe().expect("Current exe path not found.");
    exe_path.pop();
    exe_path.pop();
    exe_path.pop();
    exe_path
}
