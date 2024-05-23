use crate::cli::Target;
use crate::*;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn check(target: Target, manifest_path: PathBuf) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    match target {
        Target::WasmUnknownWeb | Target::WasmWindowsElectron | Target::WasmLinuxElectron => {
            bail!("{} is unsupported target. TODO", target)
        }
        Target::X86_64PcWindowsMsvc => {
            let mut args = vec![];
            if cfg!(target_os = "linux") {
                args.push("xwin");
            }

            args.extend([
                "check",
                "--target",
                "x86_64-pc-windows-msvc",
                "--manifest-path",
                manifest_path.to_str().unwrap(),
                "--tests",
            ]);

            if cfg!(target_os = "linux") {
                args.extend(["--xwin-arch", "x86_64", "--xwin-version", "17"]);
            }

            Command::new("cargo")
                .args(args)
                // .envs(get_envs(build_option)) << TODO
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()?
                .wait()
                .await?;
        }
    }

    Ok(())
}
