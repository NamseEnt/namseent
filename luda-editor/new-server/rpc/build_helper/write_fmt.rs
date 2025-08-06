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
        panic!("Failed to run rustfmt: {output:?}");
    }

    write_if_changed(path, String::from_utf8(output.stdout).unwrap())
}
pub fn write_if_changed(path: impl AsRef<Path>, contents: impl ToString) {
    let contents = contents.to_string();
    if let Ok(existing) = std::fs::read_to_string(&path) {
        if existing == contents {
            return;
        }
    }
    println!("Writing {}", path.as_ref().display());
    std::fs::write(&path, contents).unwrap();
}
