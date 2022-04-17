use crate::services::electron_package_service::{Arch, Platform};
use clap::{ArgEnum, Parser, Subcommand};
use std::{path::PathBuf, str::FromStr};

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    DevWasmWeb {},
    DevWasmElectron {},
    ReleaseWasmWeb {},
    ReleaseWasmElectron {
        #[clap(short, long, arg_enum, default_value = "auto")]
        platform: ElectronPackagePlatform,
        #[clap(short, long, arg_enum, default_value = "auto")]
        arch: ElectronPackageArch,
    },
    Test {
        #[clap(arg_enum)]
        target: Target,
        #[clap(short, long, parse(from_os_str))]
        manifest_path: Option<PathBuf>,
    },
}

#[derive(Clone, ArgEnum)]
pub enum Target {
    #[clap(rename_all = "kebab-case")]
    WasmUnknownWeb,
}

#[derive(Clone, ArgEnum)]
#[clap(short, long)]
pub enum ElectronPackagePlatform {
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
pub enum ElectronPackageArch {
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
