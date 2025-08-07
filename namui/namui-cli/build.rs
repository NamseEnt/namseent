use anyhow::*;
use clap::CommandFactory;
use clap_complete::{generate_to, shells::Bash};
use std::env;
use std::fs::create_dir_all;
use tokio::process::Command;

include!("src/cli.rs");

#[tokio::main]
async fn main() -> Result<()> {
    generate_completions()?;
    generate_symlink()?;

    tokio::try_join!(download_wasi_sdk(), download_emsdk(), download_binaryen(),)?;

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

async fn download_wasi_sdk() -> Result<()> {
    const VERSION: &str = "27.0";

    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let dist = root.join("wasi-sdk");
    let temp = root.join("wasi-sdk-temp");

    let version_file_path = dist.join("VERSION");

    if dist.exists() {
        if let std::io::Result::Ok(version_file) = std::fs::read_to_string(&version_file_path) {
            println!("WASI-SDK {version_file} Installed");

            if version_file == VERSION {
                return Ok(());
            }
        }

        std::fs::remove_dir_all(&dist)?;
    }

    let platform = if cfg!(target_os = "windows") {
        "x86_64-windows"
    } else if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            "arm64-macos"
        } else {
            "x86_64-macos"
        }
    } else if cfg!(target_os = "linux") {
        if cfg!(target_arch = "aarch64") {
            "arm64-linux"
        } else {
            "x86_64-linux"
        }
    } else {
        return Err(anyhow::anyhow!("Unsupported platform"));
    };

    println!("DOWNLOADING WASI-SDK {VERSION}");
    let version_without_dot = VERSION.split(".").next().unwrap();
    let url = format!(
        "https://github.com/WebAssembly/wasi-sdk/releases/download/wasi-sdk-{version_without_dot}/wasi-sdk-{VERSION}-{platform}.tar.gz"
    );

    let response = reqwest::get(url).await?.error_for_status()?;
    let bytes = response.bytes().await?;

    let mut d = flate2::read::GzDecoder::new(bytes.as_ref());
    let mut archive = tar::Archive::new(&mut d);
    archive.unpack(&temp)?;
    std::fs::rename(temp.join(format!("wasi-sdk-{VERSION}-{platform}")), dist)?;
    std::fs::remove_dir(temp)?;

    std::fs::write(version_file_path, VERSION)?;

    Ok(())
}

async fn download_emsdk() -> Result<()> {
    const VERSION: &str = "3.1.61";

    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let dist = root.join("emscripten");
    let version_file_path = dist.join("VERSION");

    if dist.exists() {
        if let std::io::Result::Ok(version_file) = std::fs::read_to_string(&version_file_path) {
            println!("EMSDK {version_file} Installed");

            if version_file == VERSION {
                return Ok(());
            }
        }

        std::fs::remove_dir_all(&dist)?;
    }

    println!("DOWNLOADING EMSCRIPTEN");

    assert!(
        Command::new("git")
            .current_dir(&root)
            .args([
                "clone",
                "--filter=blob:none",
                "--no-checkout",
                "https://github.com/emscripten-core/emscripten",
            ])
            .output()
            .await?
            .status
            .success()
    );

    assert!(
        Command::new("git")
            .current_dir(&dist)
            .args(["sparse-checkout", "set", "--cone"])
            .output()
            .await?
            .status
            .success()
    );

    assert!(
        Command::new("git")
            .current_dir(&dist)
            .args(["checkout", VERSION])
            .output()
            .await?
            .status
            .success()
    );

    assert!(
        Command::new("git")
            .current_dir(&dist)
            .args(["sparse-checkout", "set", "system/include"])
            .output()
            .await?
            .status
            .success()
    );

    // NOTE: This is a temporary solution to avoid the error.
    tokio::fs::remove_file(dist.join("system/include/emscripten/version.h")).await?;
    let no_version_emscripten_h =
        tokio::fs::read_to_string(dist.join("system/include/emscripten/emscripten.h"))
            .await?
            .replace("#include \"version.h\"", "// #include \"version.h\"");
    tokio::fs::write(
        dist.join("system/include/emscripten/emscripten.h"),
        no_version_emscripten_h,
    )
    .await?;

    std::fs::write(version_file_path, VERSION)?;

    Ok(())
}

async fn download_binaryen() -> Result<()> {
    const VERSION: &str = "119";

    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let dist = root.join("binaryen");
    let temp = root.join("binaryen-temp");

    let version_file_path = dist.join("VERSION");
    let expected_version_file_content = VERSION;

    if dist.exists() {
        if let std::io::Result::Ok(version_file) = std::fs::read_to_string(&version_file_path) {
            println!("Binaryen {version_file} Installed");

            if version_file == expected_version_file_content {
                return Ok(());
            }
        }

        std::fs::remove_dir_all(&dist)?;
    }

    println!("DOWNLOADING BINARYEN {VERSION}");
    let url = format!(
        "https://github.com/WebAssembly/binaryen/releases/download/version_{VERSION}/binaryen-version_{VERSION}-x86_64-linux.tar.gz"
    );

    let response = reqwest::get(url).await?.error_for_status()?;
    let bytes = response.bytes().await?;

    let mut d = flate2::read::GzDecoder::new(bytes.as_ref());
    let mut archive = tar::Archive::new(&mut d);
    archive.unpack(&temp)?;
    std::fs::rename(temp.join(format!("binaryen-version_{VERSION}")), dist)?;
    std::fs::remove_dir(temp)?;

    std::fs::write(version_file_path, expected_version_file_content)?;

    Ok(())
}
