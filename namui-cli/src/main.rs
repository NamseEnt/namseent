mod procedures;
mod services;
mod util;
use procedures::{dev_wasm_electron, dev_wasm_web, release_wasm_electron, release_wasm_web};
use services::electron_package_service::{Arch, Platform};
use std::{env::current_dir, str::FromStr};
mod types;
use clap::{ArgEnum, Parser, Subcommand};

#[derive(Parser)]
#[clap(version)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    DevWasmWeb {},
    DevWasmElectron {},
    ReleaseWasmWeb {},
    ReleaseWasmElectron {
        #[clap(short, long, arg_enum, default_value = "auto")]
        platform: ElectronPackagePlatform,
        #[clap(short, long, arg_enum, default_value = "auto")]
        arch: ElectronPackageArch,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let manifest_path = current_dir()
        .expect("No current dir found")
        .join("Cargo.toml");

    let result = match &cli.command {
        Commands::DevWasmWeb {} => dev_wasm_web(&manifest_path),
        Commands::DevWasmElectron {} => dev_wasm_electron(&manifest_path),
        Commands::ReleaseWasmWeb {} => release_wasm_web(&manifest_path),
        Commands::ReleaseWasmElectron { platform, arch } => {
            release_wasm_electron(&manifest_path, platform.into(), arch.into())
        }
    };

    match result {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);
            std::process::exit(1);
        }
    }
}

#[derive(Clone, ArgEnum)]
#[clap(short, long)]
enum ElectronPackagePlatform {
    #[clap(rename_all = "camelCase")]
    Auto,
    Win32,
    Linux,
}
impl FromStr for ElectronPackagePlatform {
    type Err = String;
    fn from_str(tr: &str) -> Result<Self, Self::Err> {
        match tr {
            "auto" => Ok(Self::Auto),
            "win32" => Ok(Self::Win32),
            "linux" => Ok(Self::Linux),
            _ => Err(format!("Unknown platform")),
        }
    }
}
impl Into<Option<Platform>> for &ElectronPackagePlatform {
    fn into(self) -> Option<Platform> {
        match self {
            ElectronPackagePlatform::Auto => None,
            ElectronPackagePlatform::Win32 => Some(Platform::Win32),
            ElectronPackagePlatform::Linux => Some(Platform::Linux),
        }
    }
}

#[derive(Clone, ArgEnum)]
#[clap(short, long)]
enum ElectronPackageArch {
    #[clap(rename_all = "camelCase")]
    Auto,
    X64,
}
impl FromStr for ElectronPackageArch {
    type Err = String;
    fn from_str(tr: &str) -> Result<Self, Self::Err> {
        match tr {
            "auto" => Ok(Self::Auto),
            "win32" => Ok(Self::X64),
            _ => Err(format!("Unknown arch")),
        }
    }
}
impl Into<Option<Arch>> for &ElectronPackageArch {
    fn into(self) -> Option<Arch> {
        match self {
            ElectronPackageArch::Auto => None,
            ElectronPackageArch::X64 => Some(Arch::X64),
        }
    }
}
