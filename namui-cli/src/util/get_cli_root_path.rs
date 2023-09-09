use std::{
    env::current_exe,
    fs::read_dir,
    path::{Path, PathBuf},
};

pub fn get_cli_root_path() -> PathBuf {
    let mut exe_path = current_exe().expect("Current exe path not found.");
    println!("exe_path: {:?}", exe_path);
    exe_path.pop();
    for ancestor in exe_path.ancestors() {
        let cargo_toml_exist = check_cargo_toml_exist(ancestor);
        if cargo_toml_exist {
            return ancestor.into();
        }
    }
    panic!("Could not found cli_root_path");
}

fn check_cargo_toml_exist(path: &Path) -> bool {
    read_dir(path)
        .unwrap()
        .any(|dirent| dirent.unwrap().file_name() == "Cargo.toml")
}
