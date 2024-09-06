use anyhow::*;
use clap::CommandFactory;
use clap_complete::{generate_to, shells::Bash};
use std::env;
use std::fs::create_dir_all;
use std::process::Command;

include!("src/cli.rs");

fn main() -> Result<()> {
    generate_completions()?;
    generate_symlink()?;
    download_wasi_sdk()?;
    download_emsdk()?;

    Ok(())
}

fn generate_symlink() -> Result<()> {
    // NOTE: This is very temporary solution.
    let Some(cargo_home) = env::var_os("CARGO_HOME") else {
        return Ok(());
    };

    let symlink_path = PathBuf::from(cargo_home)
        .join("bin")
        .join(if cfg!(target_os = "windows") {
            "namui.exe"
        } else {
            "namui"
        });

    if std::fs::symlink_metadata(&symlink_path).is_ok() {
        std::fs::remove_file(&symlink_path)?;
    }
    let bin_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap())
        .join("target")
        .join(env::var_os("PROFILE").unwrap())
        .join(if cfg!(target_os = "windows") {
            "namui-cli.exe"
        } else {
            "namui-cli"
        });

    if cfg!(target_os = "windows") {
        #[cfg(target_os = "windows")]
        {
            std::os::windows::fs::symlink_file(bin_path, symlink_path).map_err(|err| {
                anyhow!(
                    "Failed to create symlink to namui executable. \
                            Please turn on windows developer mode. {}",
                    err
                )
            })?;
        }
    } else {
        #[cfg(not(target_os = "windows"))]
        {
            std::os::unix::fs::symlink(bin_path, symlink_path)?;
        }
    }

    Ok(())
}

fn generate_completions() -> Result<()> {
    let outdir = match env::var_os("CARGO_MANIFEST_DIR") {
        None => return Ok(()),
        Some(outdir) => PathBuf::from(outdir).join("target").join("completions"),
    };

    create_dir_all(&outdir)?;

    let mut cmd = Cli::command();
    generate_to(Bash, &mut cmd, "namui", outdir)?;

    Ok(())
}

fn download_wasi_sdk() -> Result<()> {
    const VERSION: &str = "23";

    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let dist = root.join("wasi-sdk");
    let temp = root.join("wasi-sdk-temp");

    let version_file_path = dist.join("VERSION");
    let expected_version_file_content = format!("{VERSION}.0");

    if dist.exists() {
        if let std::io::Result::Ok(version_file) = std::fs::read_to_string(&version_file_path) {
            println!("WASI-SDK {version_file} Installed");

            if version_file == expected_version_file_content {
                return Ok(());
            }
        }

        std::fs::remove_dir_all(&dist)?;
    }

    println!("DOWNLOADING WASI-SDK {VERSION}.0");
    let url = format!("https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-{VERSION}/wasi-sdk-{VERSION}.0-x86_64-linux.tar.gz");

    let response = reqwest::blocking::get(url)?.error_for_status()?;

    let mut d = flate2::read::GzDecoder::new(response);
    let mut archive = tar::Archive::new(&mut d);
    archive.unpack(&temp)?;
    std::fs::rename(
        temp.join(format!("wasi-sdk-{VERSION}.0-x86_64-linux")),
        dist,
    )?;
    std::fs::remove_dir(temp)?;

    std::fs::write(version_file_path, expected_version_file_content)?;

    Ok(())
}

fn download_emsdk() -> Result<()> {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let dist = root.join("emscripten");
    if dist.exists() {
        return Ok(());
    }

    println!("DOWNLOADING EMSCRIPTEN");

    assert!(Command::new("git")
        .current_dir(&root)
        .args([
            "clone",
            "--filter=blob:none",
            "--no-checkout",
            "https://github.com/emscripten-core/emscripten",
        ])
        .output()?
        .status
        .success());

    assert!(Command::new("git")
        .current_dir(&dist)
        .args(["sparse-checkout", "set", "--cone"])
        .output()?
        .status
        .success());

    assert!(Command::new("git")
        .current_dir(&dist)
        .args(["checkout", "3.1.61"])
        .output()?
        .status
        .success());

    assert!(Command::new("git")
        .current_dir(&dist)
        .args(["sparse-checkout", "set", "system/include"])
        .output()?
        .status
        .success());

    Ok(())
}
