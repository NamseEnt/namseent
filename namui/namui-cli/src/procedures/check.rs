use crate::cli::NamuiTarget;
use crate::*;
use services::wasi_cargo_envs::wasi_cargo_envs;
use std::path::PathBuf;
use tokio::process::Command;

pub async fn check(target: NamuiTarget, manifest_path: PathBuf) -> Result<()> {
    let manifest_path = std::fs::canonicalize(manifest_path)?;

    let rust_target = match target {
        NamuiTarget::Wasm32WasiWeb => "wasm32-wasip2",
        NamuiTarget::X86_64PcWindowsMsvc => "x86_64-pc-windows-msvc",
        NamuiTarget::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
        NamuiTarget::Aarch64AppleDarwin => "aarch64-apple-darwin",
    };

    let mut args = vec![];

    if let NamuiTarget::X86_64PcWindowsMsvc = target
        && cfg!(target_os = "linux")
    {
        args.push("xwin");
    }

    args.extend([
        "check",
        "--target",
        rust_target,
        "--manifest-path",
        manifest_path.to_str().unwrap(),
        "--all-targets",
    ]);

    if let NamuiTarget::X86_64PcWindowsMsvc = target
        && cfg!(target_os = "linux")
    {
        args.push("xwin");
    }

    let env: &[_] = if let NamuiTarget::Wasm32WasiWeb = target {
        &wasi_cargo_envs()
    } else {
        &[]
    };

    Command::new("cargo")
        .args(args)
        .envs(env.iter().cloned())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?
        .wait()
        .await?;

    Ok(())
}
