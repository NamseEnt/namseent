use crate::util::get_electron_root_path;
use std::process::Command;

#[test]
fn test_electron() {
    let electron_root_path = get_electron_root_path();
    let test_output = Command::new("npm")
        .current_dir(electron_root_path)
        .arg("test")
        .output()
        .unwrap();
    let is_test_successful = test_output
        .status
        .code()
        .map(|code| code == 0)
        .unwrap_or(false);
    if is_test_successful {
        return;
    }
    panic!("{}", String::from_utf8(test_output.stdout).unwrap());
}
