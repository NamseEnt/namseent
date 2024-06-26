use std::{io::Write, path::Path};

pub fn write_fmt(path: impl AsRef<Path>, code: impl ToString) {
    let mut fmt = std::process::Command::new("rustfmt")
        .args(["--edition", "2021"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    fmt.stdin
        .take()
        .unwrap()
        .write_all(code.to_string().as_bytes())
        .unwrap();
    let output = fmt.wait_with_output().unwrap();
    if !output.status.success() {
        panic!("Failed to run rustfmt: {:?}", output);
    }
    std::fs::write(path, output.stdout).unwrap();
}
