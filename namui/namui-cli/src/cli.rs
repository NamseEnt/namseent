use clap::{Parser, Subcommand, ValueEnum, ValueHint};
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
        #[arg(short, long, value_hint = ValueHint::FilePath)]
        manifest_path: Option<PathBuf>,
        #[arg(long)]
        release: bool,
    },
    Build {
        #[arg(value_enum)]
        target: Option<Target>,
        #[arg(short, long, value_hint = ValueHint::FilePath)]
        manifest_path: Option<PathBuf>,
        #[arg(short, long, value_enum, default_value = "auto")]
        arch: ElectronPackageArch,
        #[arg(long)]
        release: bool,
    },
    Test {
        #[arg(value_enum)]
        target: Option<Target>,
        #[clap(short, long, value_hint = ValueHint::FilePath)]
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
    Clippy {
        #[arg(value_enum)]
        target: Option<Target>,
        #[arg(short, long, value_hint = ValueHint::FilePath)]
        manifest_path: Option<PathBuf>,
    },
    Check {
        #[arg(value_enum)]
        target: Option<Target>,
        #[arg(short, long, value_hint = ValueHint::FilePath)]
        manifest_path: Option<PathBuf>,
    },
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, ValueEnum)]
pub enum Target {
    WasmUnknownWeb,
    WasmWindowsElectron,
    WasmLinuxElectron,
    #[value(name = "x86_64-pc-windows-msvc")]
    X86_64PcWindowsMsvc,
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
                Target::X86_64PcWindowsMsvc => "x86_64-pc-windows-msvc",
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
            namui_user_config::Target::X86_64PcWindowsMsvc => Target::X86_64PcWindowsMsvc,
        }
    }
}
impl From<Target> for namui_user_config::Target {
    fn from(val: Target) -> Self {
        match val {
            Target::WasmUnknownWeb => namui_user_config::Target::WasmUnknownWeb,
            Target::WasmWindowsElectron => namui_user_config::Target::WasmWindowsElectron,
            Target::WasmLinuxElectron => namui_user_config::Target::WasmLinuxElectron,
            Target::X86_64PcWindowsMsvc => namui_user_config::Target::X86_64PcWindowsMsvc,
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
