use anyhow::*;
use clap::CommandFactory;
use clap_complete::{generate_to, shells::Bash, shells::Zsh};
use std::env;
use std::fs::create_dir_all;
use std::path::Path;
use tokio::process::Command;

include!("src/cli.rs");

#[tokio::main]
async fn main() -> Result<()> {
    generate_completions().context("failed to generate shell completions")?;
    generate_symlink().context("failed to create namui executable symlink")?;

    tokio::try_join!(
        async {
            download_wasi_sdk()
                .await
                .context("failed to download or install WASI SDK")
        },
        async {
            download_emsdk()
                .await
                .context("failed to download or install Emscripten")
        },
        async {
            download_binaryen()
                .await
                .context("failed to download or install Binaryen")
        },
    )?;

    Ok(())
}

fn generate_symlink() -> Result<()> {
    // NOTE: This is very temporary solution.
    let Some(cargo_home) = env::var_os("CARGO_HOME") else {
        return Ok(());
    };

    let cargo_bin_dir = PathBuf::from(cargo_home).join("bin");
    create_dir_all(&cargo_bin_dir).with_context(|| {
        format!(
            "failed to create Cargo bin directory {}",
            cargo_bin_dir.display()
        )
    })?;
    let symlink_path = cargo_bin_dir.join(if cfg!(target_os = "windows") {
        "namui.exe"
    } else {
        "namui"
    });

    if std::fs::symlink_metadata(&symlink_path).is_ok() {
        std::fs::remove_file(&symlink_path).with_context(|| {
            format!(
                "failed to remove existing symlink {}",
                symlink_path.display()
            )
        })?;
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
            std::os::unix::fs::symlink(&bin_path, &symlink_path).with_context(|| {
                format!(
                    "failed to create symlink {} -> {}",
                    symlink_path.display(),
                    bin_path.display()
                )
            })?;
        }
    }

    Ok(())
}

fn generate_completions() -> Result<()> {
    let outdir = match env::var_os("CARGO_MANIFEST_DIR") {
        None => return Ok(()),
        Some(outdir) => PathBuf::from(outdir).join("target").join("completions"),
    };

    create_dir_all(&outdir)
        .with_context(|| format!("failed to create completion directory {}", outdir.display()))?;

    let mut cmd = Cli::command();
    generate_to(Bash, &mut cmd, "namui", &outdir).with_context(|| {
        format!(
            "failed to generate Bash completions in {}",
            outdir.display()
        )
    })?;
    generate_to(Zsh, &mut cmd, "namui", &outdir)
        .with_context(|| format!("failed to generate Zsh completions in {}", outdir.display()))?;

    // Install zsh completion
    if let Some(home) = env::var_os("HOME") {
        let home = PathBuf::from(home);

        // Copy completion to ~/.zfunc/
        let zfunc = home.join(".zfunc");
        let _ = create_dir_all(&zfunc);
        let _ = std::fs::copy(outdir.join("_namui"), zfunc.join("_namui"));

        // Add fpath + compinit to ~/.zshrc if not already present
        let zshrc = home.join(".zshrc");
        let content = std::fs::read_to_string(&zshrc).unwrap_or_default();
        if !content.contains(".zfunc") {
            use std::io::Write;
            if let std::result::Result::Ok(mut f) =
                std::fs::OpenOptions::new().append(true).open(&zshrc)
            {
                let _ = writeln!(
                    f,
                    "\n# namui completions\nfpath=(~/.zfunc $fpath)\nautoload -U compinit && compinit"
                );
            }
        }
    }

    Ok(())
}

async fn download_wasi_sdk() -> Result<()> {
    const VERSION: &str = "25.0";

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

        std::fs::remove_dir_all(&dist)
            .with_context(|| format!("failed to remove outdated WASI SDK at {}", dist.display()))?;
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

    let response = reqwest::get(&url)
        .await
        .with_context(|| format!("failed to request WASI SDK archive from {url}"))?
        .error_for_status()
        .with_context(|| format!("WASI SDK archive request returned an error for {url}"))?;
    let bytes = response
        .bytes()
        .await
        .with_context(|| format!("failed to read WASI SDK archive from {url}"))?;

    let mut d = flate2::read::GzDecoder::new(bytes.as_ref());
    let mut archive = tar::Archive::new(&mut d);
    archive
        .unpack(&temp)
        .with_context(|| format!("failed to unpack WASI SDK archive into {}", temp.display()))?;
    let extracted = temp.join(format!("wasi-sdk-{VERSION}-{platform}"));
    std::fs::rename(&extracted, &dist).with_context(|| {
        format!(
            "failed to move extracted WASI SDK from {} to {}",
            extracted.display(),
            dist.display()
        )
    })?;
    std::fs::remove_dir(&temp).with_context(|| {
        format!(
            "failed to remove temporary WASI SDK directory {}",
            temp.display()
        )
    })?;

    std::fs::write(&version_file_path, VERSION).with_context(|| {
        format!(
            "failed to write WASI SDK version file {}",
            version_file_path.display()
        )
    })?;

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

        std::fs::remove_dir_all(&dist).with_context(|| {
            format!(
                "failed to remove outdated Emscripten checkout at {}",
                dist.display()
            )
        })?;
    }

    println!("DOWNLOADING EMSCRIPTEN");

    run_git(
        &root,
        &[
            "clone",
            "--filter=blob:none",
            "--no-checkout",
            "https://github.com/emscripten-core/emscripten",
        ],
    )
    .await?;
    run_git(&dist, &["sparse-checkout", "set", "--cone"]).await?;
    run_git(&dist, &["checkout", VERSION]).await?;
    run_git(&dist, &["sparse-checkout", "set", "system/include"]).await?;

    // NOTE: This is a temporary solution to avoid the error.
    let version_header = dist.join("system/include/emscripten/version.h");
    match tokio::fs::remove_file(&version_header).await {
        std::result::Result::Ok(()) => {}
        std::result::Result::Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        std::result::Result::Err(error) => {
            return Err(error).with_context(|| {
                format!(
                    "failed to remove Emscripten version header {}",
                    version_header.display()
                )
            });
        }
    }
    let emscripten_header = dist.join("system/include/emscripten/emscripten.h");
    let no_version_emscripten_h = tokio::fs::read_to_string(&emscripten_header)
        .await
        .with_context(|| {
            format!(
                "failed to read Emscripten header {}",
                emscripten_header.display()
            )
        })?
        .replace("#include \"version.h\"", "// #include \"version.h\"");
    tokio::fs::write(&emscripten_header, no_version_emscripten_h)
        .await
        .with_context(|| {
            format!(
                "failed to update Emscripten header {}",
                emscripten_header.display()
            )
        })?;

    std::fs::write(&version_file_path, VERSION).with_context(|| {
        format!(
            "failed to write Emscripten version file {}",
            version_file_path.display()
        )
    })?;

    Ok(())
}

async fn run_git(root: &Path, args: &[&str]) -> Result<()> {
    let command = format!("git {}", args.join(" "));
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .output()
        .await
        .with_context(|| format!("failed to start `{command}` in {}", root.display()))?;

    if !output.status.success() {
        bail!(
            "`{command}` failed in {} with status {}: {}",
            root.display(),
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }

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

        std::fs::remove_dir_all(&dist).with_context(|| {
            format!(
                "failed to remove outdated Binaryen installation at {}",
                dist.display()
            )
        })?;
    }

    println!("DOWNLOADING BINARYEN {VERSION}");
    let url = format!(
        "https://github.com/WebAssembly/binaryen/releases/download/version_{VERSION}/binaryen-version_{VERSION}-x86_64-linux.tar.gz"
    );

    let response = reqwest::get(&url)
        .await
        .with_context(|| format!("failed to request Binaryen archive from {url}"))?
        .error_for_status()
        .with_context(|| format!("Binaryen archive request returned an error for {url}"))?;
    let bytes = response
        .bytes()
        .await
        .with_context(|| format!("failed to read Binaryen archive from {url}"))?;

    let mut d = flate2::read::GzDecoder::new(bytes.as_ref());
    let mut archive = tar::Archive::new(&mut d);
    archive
        .unpack(&temp)
        .with_context(|| format!("failed to unpack Binaryen archive into {}", temp.display()))?;
    let extracted = temp.join(format!("binaryen-version_{VERSION}"));
    std::fs::rename(&extracted, &dist).with_context(|| {
        format!(
            "failed to move extracted Binaryen from {} to {}",
            extracted.display(),
            dist.display()
        )
    })?;
    std::fs::remove_dir(&temp).with_context(|| {
        format!(
            "failed to remove temporary Binaryen directory {}",
            temp.display()
        )
    })?;

    std::fs::write(&version_file_path, expected_version_file_content).with_context(|| {
        format!(
            "failed to write Binaryen version file {}",
            version_file_path.display()
        )
    })?;

    Ok(())
}
