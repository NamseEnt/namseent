use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Parser)]
#[command(version, name = "namui")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[arg(value_enum)]
        target: Option<Target>,
        #[arg(short, long)]
        manifest_path: Option<PathBuf>,
    },
    Build {
        #[arg(value_enum)]
        target: Option<Target>,
        #[arg(short, long)]
        manifest_path: Option<PathBuf>,
        #[arg(short, long, value_enum, default_value = "auto")]
        arch: ElectronPackageArch,
    },
    Test {
        #[arg(value_enum)]
        target: Option<Target>,
        #[clap(short, long)]
        manifest_path: Option<PathBuf>,
    },
    Target {
        #[arg(value_enum)]
        target: Target,
    },
    Print {
        #[arg(value_enum)]
        printable_object: PrintableObject,
    },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, ValueEnum)]
pub enum Target {
    #[value(rename_all = "kebab-case")]
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

#[derive(Clone, ValueEnum)]
pub enum PrintableObject {
    #[value(rename_all = "camelCase")]
    Cfg,
    Target,
}

#[derive(Clone, ValueEnum)]
pub enum ElectronPackageArch {
    #[value(rename_all = "camelCase")]
    Auto,
    X64,
}
