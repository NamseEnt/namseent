use clap::{Parser, Subcommand, ValueEnum, ValueHint};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Parser)]
#[command(version, name = "namui")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

pub struct StartOption {
    pub release: bool,
    pub host: Option<String>,
    pub strip_debug_info: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Start {
        #[arg(value_enum)]
        target: Option<NamuiTarget>,
        #[arg(short, long, value_hint = ValueHint::FilePath)]
        manifest_path: Option<PathBuf>,
        #[arg(long)]
        release: bool,
        #[arg(long)]
        host: Option<String>,
        #[arg(long)]
        strip_debug_info: bool,
    },
    Build {
        #[arg(value_enum)]
        target: Option<NamuiTarget>,
        #[arg(short, long, value_hint = ValueHint::FilePath)]
        manifest_path: Option<PathBuf>,
        #[arg(long)]
        release: bool,
    },
    Test {
        #[arg(value_enum)]
        target: Option<NamuiTarget>,
        #[clap(short, long, value_hint = ValueHint::FilePath)]
        manifest_path: Option<PathBuf>,
    },
    Target {
        #[arg(value_enum)]
        target: NamuiTarget,
    },
    Print {
        #[arg(value_enum)]
        printable_object: PrintableObject,
    },
    Clippy {
        #[arg(value_enum)]
        target: Option<NamuiTarget>,
        #[arg(short, long, value_hint = ValueHint::FilePath)]
        manifest_path: Option<PathBuf>,
    },
    Check {
        #[arg(value_enum)]
        target: Option<NamuiTarget>,
        #[arg(short, long, value_hint = ValueHint::FilePath)]
        manifest_path: Option<PathBuf>,
    },
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize, ValueEnum)]
pub enum NamuiTarget {
    Wasm32WasiWeb,
    #[value(name = "x86_64-pc-windows-msvc")]
    X86_64PcWindowsMsvc,
    #[value(name = "x86_64-unknown-linux-gnu")]
    X86_64UnknownLinuxGnu,
    #[value(name = "aarch64-apple-darwin")]
    Aarch64AppleDarwin,
}
impl Display for NamuiTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NamuiTarget::Wasm32WasiWeb => "wasm32-wasi-web",
                NamuiTarget::X86_64PcWindowsMsvc => "x86_64-pc-windows-msvc",
                NamuiTarget::X86_64UnknownLinuxGnu => "x86_64-unknown-linux-gnu",
                NamuiTarget::Aarch64AppleDarwin => "aarch64-apple-darwin",
            }
        )
    }
}
impl From<namui_user_config::Target> for NamuiTarget {
    fn from(target: namui_user_config::Target) -> Self {
        match target {
            namui_user_config::Target::Wasm32WasiWeb => NamuiTarget::Wasm32WasiWeb,
            namui_user_config::Target::X86_64PcWindowsMsvc => NamuiTarget::X86_64PcWindowsMsvc,
            namui_user_config::Target::X86_64UnknownLinuxGnu => NamuiTarget::X86_64UnknownLinuxGnu,
            namui_user_config::Target::Aarch64AppleDarwin => NamuiTarget::Aarch64AppleDarwin,
        }
    }
}
impl From<NamuiTarget> for namui_user_config::Target {
    fn from(val: NamuiTarget) -> Self {
        match val {
            NamuiTarget::Wasm32WasiWeb => namui_user_config::Target::Wasm32WasiWeb,
            NamuiTarget::X86_64PcWindowsMsvc => namui_user_config::Target::X86_64PcWindowsMsvc,
            NamuiTarget::X86_64UnknownLinuxGnu => namui_user_config::Target::X86_64UnknownLinuxGnu,
            NamuiTarget::Aarch64AppleDarwin => namui_user_config::Target::Aarch64AppleDarwin,
        }
    }
}

#[derive(Clone, ValueEnum)]
pub enum PrintableObject {
    #[value(rename_all = "camelCase")]
    Cfg,
    Target,
}
