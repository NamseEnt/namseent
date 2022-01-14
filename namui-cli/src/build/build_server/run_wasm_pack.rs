use crate::build::types::ErrorMessage;
use cargo_metadata::{diagnostic::DiagnosticLevel, CompilerMessage, Message};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

pub struct RunWasmPackOption {
    pub root_dir: String,
}

pub struct RunWasmPackResult {
    pub result_js_path: String,
    pub result_wasm_path: String,
    pub warning_messages: Vec<ErrorMessage>,
    pub error_messages: Vec<ErrorMessage>,
    pub other_messages: Vec<ErrorMessage>,
    pub is_successful: bool,
}

#[derive(Debug)]
pub enum RunWasmPackError {
    IoError(std::io::Error),
}

pub struct WasmPackMessage {
    pub warning_messages: Vec<ErrorMessage>,
    pub error_messages: Vec<ErrorMessage>,
    pub other_messages: Vec<ErrorMessage>,
    pub is_successful: bool,
}

pub fn run_wasm_pack(option: RunWasmPackOption) -> Result<RunWasmPackResult, RunWasmPackError> {
    let mut out_dir = PathBuf::from(&option.root_dir);
    out_dir.push("pkg");
    let output = Command::new("wasm-pack")
        .args([
            "build",
            "--target",
            "no-modules",
            "--out-name",
            "bundle",
            "--dev",
            option.root_dir.as_str(),
            "--",
            "--message-format",
            "json",
        ])
        .stdout(Stdio::piped())
        .output();

    match output {
        Ok(output) => {
            let wasm_pack_message = parse_wasm_pack_message(&output.stdout);
            let result_wasm_path = out_dir.join("bundle_bg.wasm").to_string_lossy().to_string();
            let result_js_path = out_dir.join("bundle.js").to_string_lossy().to_string();

            Ok(RunWasmPackResult {
                result_js_path,
                result_wasm_path,
                warning_messages: wasm_pack_message.warning_messages,
                error_messages: wasm_pack_message.error_messages,
                other_messages: wasm_pack_message.other_messages,
                is_successful: wasm_pack_message.is_successful,
            })
        }
        Err(error) => Err(RunWasmPackError::IoError(error)),
    }
}

fn parse_wasm_pack_message(stdout: &[u8]) -> WasmPackMessage {
    let mut warning_messages: Vec<ErrorMessage> = Vec::new();
    let mut error_messages: Vec<ErrorMessage> = Vec::new();
    let mut other_messages: Vec<ErrorMessage> = Vec::new();
    let mut is_successful: bool = false;

    let reader = std::io::BufReader::new(stdout);
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.unwrap() {
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

    WasmPackMessage {
        warning_messages,
        error_messages,
        other_messages,
        is_successful,
    }
}

fn convert_compiler_message_to_namui_error_message(
    message: &CompilerMessage,
) -> Result<ErrorMessage, ()> {
    let first_span = message.message.spans.get(0);
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
