use crate::services::electron_package_service;
use clap::{ArgEnum, Parser, Subcommand};
use std::path::PathBuf;

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
        target: Target,
        #[clap(short, long, parse(from_os_str))]
        manifest_path: Option<PathBuf>,
    },
    Build {
        #[clap(arg_enum)]
        target: Target,
        #[clap(short, long, parse(from_os_str))]
        manifest_path: Option<PathBuf>,
        #[clap(arg_enum, default_value = "auto")]
        arch: ElectronPackageArch,
    },
    Test {
        #[clap(arg_enum)]
        target: Target,
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

#[derive(Clone, ArgEnum)]
pub enum Target {
    #[clap(rename_all = "kebab-case")]
    WasmUnknownWeb,
    WasmWindowsElectron,
    WasmLinuxElectron,
}

#[derive(Clone, ArgEnum)]
pub enum PrintableObject {
    #[clap(rename_all = "camelCase")]
    Cfg,
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
