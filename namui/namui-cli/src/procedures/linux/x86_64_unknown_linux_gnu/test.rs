use crate::*;
use std::process::Command;

pub fn test(manifest_path: impl AsRef<std::path::Path>) -> Result<()> {
    Command::new("cargo")
        .args(["test", "--target", "x86_64-unknown-linux-gnu"])
        .status()?;

    Ok(())
}
