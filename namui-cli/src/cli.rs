use crate::services::electron_package_service;
use clap::{ArgEnum, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Parser)]
#[clap(version)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[clap(arg_enum)]
        target: Option<Target>,
        #[clap(short, long, parse(from_os_str))]
        manifest_path: Option<PathBuf>,
    },
    Build {
        #[clap(arg_enum)]
        target: Option<Target>,
        #[clap(short, long, parse(from_os_str))]
        manifest_path: Option<PathBuf>,
        #[clap(arg_enum, default_value = "auto")]
        arch: ElectronPackageArch,
    },
    Test {
        #[clap(arg_enum)]
        target: Option<Target>,
        #[clap(short, long, parse(from_os_str))]
        manifest_path: Option<PathBuf>,
    },
    Target {
        #[clap(arg_enum)]
        target: Target,
    },
    Print {
        #[clap(arg_enum)]
        printable_object: PrintableObject,
    },
}

#[derive(Clone, Copy, Debug, ArgEnum, Serialize, Deserialize)]
pub enum Target {
    #[clap(rename_all = "kebab-case")]
    WasmUnknownWeb,
    WasmWindowsElectron,
    WasmLinuxElectron,
}
impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Target::WasmUnknownWeb => "wasm-unknown-web",
                Target::WasmWindowsElectron => "wasm-windows-electron",
                Target::WasmLinuxElectron => "wasm-linux-electron",
            }
        )
    }
}
impl From<namui_user_config::Target> for Target {
    fn from(target: namui_user_config::Target) -> Self {
        match target {
            namui_user_config::Target::WasmUnknownWeb => Target::WasmUnknownWeb,
            namui_user_config::Target::WasmWindowsElectron => Target::WasmWindowsElectron,
            namui_user_config::Target::WasmLinuxElectron => Target::WasmLinuxElectron,
        }
    }
}
impl Into<namui_user_config::Target> for Target {
    fn into(self) -> namui_user_config::Target {
        match self {
            Target::WasmUnknownWeb => namui_user_config::Target::WasmUnknownWeb,
            Target::WasmWindowsElectron => namui_user_config::Target::WasmWindowsElectron,
            Target::WasmLinuxElectron => namui_user_config::Target::WasmLinuxElectron,
        }
    }
}

#[derive(Clone, ArgEnum)]
pub enum PrintableObject {
    #[clap(rename_all = "camelCase")]
    Cfg,
    Target,
}

#[derive(Clone, ArgEnum)]
#[clap(short, long)]
pub enum ElectronPackageArch {
    #[clap(rename_all = "camelCase")]
    Auto,
    X64,
}
impl Into<Option<electron_package_service::Arch>> for &ElectronPackageArch {
    fn into(self) -> Option<electron_package_service::Arch> {
        match self {
            ElectronPackageArch::Auto => None,
            ElectronPackageArch::X64 => Some(electron_package_service::Arch::X64),
        }
    }
}
