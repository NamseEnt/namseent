use crate::cli::Target;
use crate::services::wasi_cargo_envs::WasiType;
use crate::*;
use services::wasi_cargo_envs::wasi_cargo_envs;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn clippy(target: Target, manifest_path: PathBuf) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    match target {
        Target::Wasm32WasiWeb => {
            let mut args = vec![];

            args.extend([
                "clippy",
                "--target",
                "wasm32-wasip1-threads",
                "--manifest-path",
                manifest_path.to_str().unwrap(),
                "--all-targets",
            ]);

            Command::new("cargo")
                .args(args)
                // .envs(get_envs(build_option)) << TODO
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .envs(wasi_cargo_envs(WasiType::App))
                .spawn()?
                .wait()
                .await?;
        }
        Target::X86_64PcWindowsMsvc => {
            let mut args = vec![];
            if cfg!(target_os = "linux") {
                args.push("xwin");
            }

            args.extend([
                "clippy",
                "--target",
                "x86_64-pc-windows-msvc",
                "--manifest-path",
                manifest_path.to_str().unwrap(),
                "--tests",
            ]);

            if cfg!(target_os = "linux") {
                args.extend([
                    "--xwin-arch",
                    "x86_64",
                    "--xwin-version",
                    "17",
                    "--cross-compiler",
                    "clang",
                ]);
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
        Target::X86_64UnknownLinuxGnu => {
            let mut args = vec![];
            args.extend([
                "clippy",
                "--target",
                "x86_64-unknown-linux-gnu",
                "--manifest-path",
                manifest_path.to_str().unwrap(),
                "--tests",
            ]);

            Command::new("cargo")
                .args(args)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()?
                .wait()
                .await?;
        }
        Target::Aarch64AppleDarwin => {
            let mut args = vec![];
            args.extend([
                "clippy",
                "--target",
                "aarch64-apple-darwin",
                "--manifest-path",
                manifest_path.to_str().unwrap(),
                "--tests",
            ]);

            Command::new("cargo")
                .args(args)
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit())
                .spawn()?
                .wait()
                .await?;
        }
    }

    Ok(())
}
