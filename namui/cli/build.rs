use anyhow::*;
use std::env;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    download_wasi_sdk().await?;
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
