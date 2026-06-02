use crate::*;
use std::process::Command;

pub fn test(manifest_path: impl AsRef<std::path::Path>) -> Result<()> {
    Command::new("cargo")
        .args([
            "xwin",
            "test",
            "--target",
            "aarch64-pc-windows-msvc",
            "--xwin-arch",
            "x86_64",
            "--xwin-version",
            "17",
            "--cross-compiler",
            "clang",
        ])
        .status()?;

    Ok(())
}
