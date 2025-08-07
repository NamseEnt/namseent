use crate::*;
use crate::{cli::NamuiTarget, types::ErrorMessage};
use cargo_metadata::{CompilerMessage, Message, diagnostic::DiagnosticLevel};
use std::path::PathBuf;
use std::process::Output;
use tokio::process::Command;

#[derive(Clone, Debug)]
pub struct BuildOption {
    pub project_root_path: PathBuf,
    pub target: NamuiTarget,
    pub watch: bool,
    pub release: bool,
}

pub fn build(build_option: BuildOption) -> tokio::task::JoinHandle<Result<CargoBuildOutput>> {
    tokio::spawn(async move {
        let output = run_build_process(&build_option).await?;

        let stderr = String::from_utf8(output.stderr)?
            // last 256 lines
            .lines()
            .rev()
            .take(256)
            .collect::<Vec<&str>>()
            .iter()
            .rev()
            .fold(String::new(), |acc, line| acc + line + "\n");

        parse_cargo_build_result(&output.stdout).map_err(|err| {
            anyhow!("Failed to parse build result: stderr: {stderr} \n cargo err:  {err}")
        })
    })
}

async fn run_build_process(build_option: &BuildOption) -> Result<Output> {
    match build_option.target {
        NamuiTarget::Wasm32WasiWeb => {
            let mut args = vec![];

            args.extend([
                "build",
                "--target",
                "wasm32-wasip2",
                "--message-format",
                "json",
                "-vv",
            ]);

            if build_option.release {
                args.push("--release");
            }

            Ok(Command::new("cargo")
                .args(args)
                .current_dir(&build_option.project_root_path)
                .envs(get_envs(build_option))
                .output()
                .await?)
        }
        NamuiTarget::X86_64PcWindowsMsvc => {
            let mut args = vec![];
            if cfg!(target_os = "linux") {
                args.push("xwin");
            }

            args.extend([
                "build",
                "--target",
                "x86_64-pc-windows-msvc",
                "--message-format",
                "json",
            ]);

            if build_option.release {
                args.push("--release");
            }

            if cfg!(target_os = "linux") {
                args.extend([
                    "--xwin-arch",
                    "x86_64",
                    "--xwin-version",
                    "17",
                    "--cross-compiler",
                    "clang",
                ]);
            }

            Ok(Command::new("cargo")
                .args(args)
                .current_dir(&build_option.project_root_path)
                .envs(get_envs(build_option))
                .output()
                .await?)
        }
        NamuiTarget::X86_64UnknownLinuxGnu => todo!(),
        NamuiTarget::Aarch64AppleDarwin => todo!(),
    }
}

fn get_envs(build_option: &BuildOption) -> Vec<(&str, &str)> {
    let mut envs = match build_option.target {
        NamuiTarget::Wasm32WasiWeb => vec![
            ("NAMUI_CFG_TARGET_ARCH", "wasm32"),
            ("NAMUI_CFG_TARGET_OS", "wasip2"),
            ("NAMUI_CFG_TARGET_ENV", ""),
        ],
        NamuiTarget::X86_64PcWindowsMsvc => vec![
            ("NAMUI_CFG_TARGET_ARCH", "x86_64"),
            ("NAMUI_CFG_TARGET_OS", "windows"),
            ("NAMUI_CFG_TARGET_ENV", "msvc"),
        ],
        NamuiTarget::X86_64UnknownLinuxGnu => vec![
            ("NAMUI_CFG_TARGET_ARCH", "x86_64"),
            ("NAMUI_CFG_TARGET_OS", "linux"),
            ("NAMUI_CFG_TARGET_ENV", "gnu"),
        ],
        NamuiTarget::Aarch64AppleDarwin => vec![
            ("NAMUI_CFG_TARGET_ARCH", "aarch64"),
            ("NAMUI_CFG_TARGET_OS", "macos"),
            ("NAMUI_CFG_TARGET_ENV", "darwin"),
        ],
    };

    if build_option.watch {
        envs.push(("NAMUI_CFG_WATCH_RELOAD", ""));
    }

    if !build_option.release {
        envs.push(("RUST_BACKTRACE", "1"));
    }

    envs
}

#[derive(Debug)]
pub struct CargoBuildOutput {
    pub warning_messages: Vec<ErrorMessage>,
    pub error_messages: Vec<ErrorMessage>,
    pub other_messages: Vec<ErrorMessage>,
    pub is_successful: bool,
}

fn parse_cargo_build_result(stdout: &[u8]) -> Result<CargoBuildOutput> {
    let mut warning_messages: Vec<ErrorMessage> = Vec::new();
    let mut error_messages: Vec<ErrorMessage> = Vec::new();
    let mut other_messages: Vec<ErrorMessage> = Vec::new();
    let mut is_successful: bool = false;

    let reader = std::io::BufReader::new(stdout);
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message? {
            Message::CompilerMessage(message) => match message.message.level {
                DiagnosticLevel::Warning => {
                    if let Ok(message) = convert_compiler_message_to_namui_error_message(&message) {
                        warning_messages.push(message);
                    }
                }
                DiagnosticLevel::Error => {
                    if let Ok(message) = convert_compiler_message_to_namui_error_message(&message) {
                        error_messages.push(message);
                    }
                }
                _ => {
                    if let Ok(message) = convert_compiler_message_to_namui_error_message(&message) {
                        other_messages.push(message);
                    }
                }
            },
            Message::BuildFinished(finished) => {
                is_successful = finished.success;
            }
            _ => (), // Unknown message
        }
    }

    Ok(CargoBuildOutput {
        warning_messages,
        error_messages,
        other_messages,
        is_successful,
    })
}

fn convert_compiler_message_to_namui_error_message(
    message: &CompilerMessage,
) -> Result<ErrorMessage, ()> {
    let first_span = message.message.spans.first();
    match first_span {
        Some(span) => {
            let relative_file = span.file_name.clone();
            let mut absolute_file = message.target.src_path.clone();
            absolute_file.pop();
            absolute_file.pop();
            absolute_file.push(&relative_file);
            let absolute_file = String::from(absolute_file.to_string_lossy());

            Ok(ErrorMessage {
                relative_file,
                absolute_file,
                line: span.line_start,
                column: span.column_start,
                text: message.message.message.clone(),
            })
        }
        None => Err(()),
    }
}
