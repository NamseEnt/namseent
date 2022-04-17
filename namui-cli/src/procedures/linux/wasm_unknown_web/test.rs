use std::{error::Error, process::Command};

pub fn test() -> Result<(), Box<dyn Error>> {
    let status = Command::new("wasm-pack")
        .args(["test", "--headless", "--chrome"])
        .status()?;

    if !status.success() {
        return Err(format!("test failed").into());
    }
    Ok(())
}
