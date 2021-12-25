use cargo_metadata::{diagnostic::DiagnosticLevel, CompilerMessage, Message};
use namui::build::types::ErrorMessage;
use std::{
    process::{Command, Stdio},
    str::FromStr,
};

pub struct CargoBuildResult {
    pub warning_messages: Vec<ErrorMessage>,
    pub error_messages: Vec<ErrorMessage>,
    pub other_messages: Vec<ErrorMessage>,
    pub result_path: Option<String>,
    pub is_successful: bool,
}

pub fn run_cargo_build(manifest_path: String) -> CargoBuildResult {
    let mut warning_messages: Vec<ErrorMessage> = Vec::new();
    let mut error_messages: Vec<ErrorMessage> = Vec::new();
    let mut other_messages: Vec<ErrorMessage> = Vec::new();
    let mut result_path: Option<String> = None;
    let mut is_successful: bool = false;

    let mut command = Command::new("cargo")
        .args([
            "build",
            "--frozen",
            "--target",
            "wasm32-unknown-unknown",
            "--message-format",
            "json",
            "--manifest-path",
            manifest_path.as_str(),
        ])
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    let reader = std::io::BufReader::new(
        command
            .stdout
            .take()
            .unwrap(),
    );
    for message in cargo_metadata::Message::parse_stream(reader) {
        match message.unwrap() {
            Message::CompilerMessage(message) => match message
                .message
                .level
            {
                DiagnosticLevel::Warning => {
                    warning_messages
                        .push(convert_compiler_message_to_namui_error_message(&message));
                }
                DiagnosticLevel::Error => {
                    error_messages.push(convert_compiler_message_to_namui_error_message(&message));
                }
                _ => other_messages.push(convert_compiler_message_to_namui_error_message(&message)),
            },
            Message::CompilerArtifact(artifact) => {
                for file in artifact.filenames {
                    let file = String::from(file.to_string_lossy());
                    if file.ends_with(".wasm") {
                        result_path = Some(file);
                    }
                }
            }
            Message::BuildFinished(finished) => {
                is_successful = finished.success;
            }
            _ => (), // Unknown message
        }
    }
    CargoBuildResult {
        warning_messages,
        error_messages,
        other_messages,
        result_path,
        is_successful,
    }
}

fn convert_compiler_message_to_namui_error_message(message: &CompilerMessage) -> ErrorMessage {
    let first_span = message
        .message
        .spans
        .get(0);
    match first_span {
        Some(span) => {
            let relative_file = span
                .file_name
                .clone();
            let mut absolute_file = message
                .target
                .src_path
                .clone();
            absolute_file.pop();
            absolute_file.pop();
            absolute_file.push(&relative_file);
            let absolute_file = String::from(absolute_file.to_string_lossy());

            ErrorMessage {
                relative_file,
                absolute_file,
                line: span.line_start,
                column: span.column_start,
                text: message
                    .message
                    .message
                    .clone(),
            }
        }
        None => ErrorMessage {
            relative_file: String::new(),
            absolute_file: String::from_str(
                message
                    .target
                    .src_path
                    .to_str()
                    .unwrap(),
            )
            .unwrap(),
            line: 0,
            column: 0,
            text: String::new(),
        },
    }
}
