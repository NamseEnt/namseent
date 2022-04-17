use std::{error::Error, path::PathBuf, process::Command};

pub fn test(manifest_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let directory = manifest_path.parent().expect("No parent directory found");
    let result = Command::new("wasm-pack")
        .args([
            "test",
            "--headless",
            "--chrome",
            directory.to_str().unwrap(),
        ])
        .status()?;

    if !result.success() {
        return Err(format!("test failed").into());
    }
    Ok(())
}
