use crate::build::types::ErrorMessage;
use cargo_metadata::{diagnostic::DiagnosticLevel, CompilerMessage, Message};
use std::{
    io::Read,
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

pub struct RustBuildService {
    builder: Mutex<Option<Arc<CancelableBuilder>>>,
}
pub enum BuildResult {
    Canceled,
    Successful(CargoBuildResult),
    Failed(String),
}

#[derive(Clone)]
pub struct BuildOption {
    pub dist_path: PathBuf,
    pub project_root_path: PathBuf,
    pub platform: BuildPlatform,
}

#[derive(Clone)]
pub enum BuildPlatform {
    WasmWeb,
}

impl RustBuildService {
    pub(crate) fn new() -> Self {
        Self {
            builder: Mutex::new(None),
        }
    }

    pub(crate) fn cancel_and_start_build(&self, build_option: &BuildOption) -> BuildResult {
        let result_receiver = {
            let mut builder_lock = self.builder.lock().unwrap();
            if let Some(builder) = builder_lock.take() {
                builder.cancel();
            }

            let (builder, result_receiver) = CancelableBuilder::start(build_option);
            *builder_lock = Some(builder);
            result_receiver
        };

        result_receiver.recv().unwrap()
    }
}

struct CancelableBuilder {
    is_cancel_requested: AtomicBool,
    is_canceled: AtomicBool,
}

impl CancelableBuilder {
    pub fn cancel(&self) {
        self.is_cancel_requested.store(true, Ordering::Relaxed);
        loop {
            thread::sleep(Duration::from_secs(1));
            if self.is_canceled.load(Ordering::Relaxed) {
                break;
            }
        }
    }

    pub fn start(
        build_option: &BuildOption,
    ) -> (
        Arc<CancelableBuilder>,
        std::sync::mpsc::Receiver<BuildResult>,
    ) {
        let (result_sender, result_receiver) = std::sync::mpsc::channel();

        let builder = Arc::new(Self {
            is_cancel_requested: AtomicBool::new(false),
            is_canceled: AtomicBool::new(false),
        });
        let build_option = build_option.clone();

        thread::spawn({
            let builder = builder.clone();
            move || {
                let send_about_build_end = |result| {
                    result_sender.send(result).unwrap();
                    builder.is_canceled.store(true, Ordering::Relaxed);
                };
                let mut spawned_process = match Self::spawn_build_process(&build_option) {
                    Ok(spawned_process) => spawned_process,
                    Err(error) => {
                        send_about_build_end(BuildResult::Failed(error.to_string()));
                        return;
                    }
                };

                let mut stdout_string = String::new();
                let mut stderr_string = String::new();
                let mut stdout = spawned_process.stdout.take().unwrap();
                let mut stderr = spawned_process.stderr.take().unwrap();

                loop {
                    thread::sleep(Duration::from_secs(1));

                    if builder.is_cancel_requested.load(Ordering::Relaxed) {
                        spawned_process.kill().unwrap();
                        send_about_build_end(BuildResult::Canceled);
                        return;
                    }

                    let _ = stdout.read_to_string(&mut stdout_string);
                    let _ = stderr.read_to_string(&mut stderr_string);

                    match spawned_process.try_wait() {
                        Ok(None) => {}
                        Ok(_) => match spawned_process.wait() {
                            Ok(exit_status) => {
                                let _ = stdout.read_to_string(&mut stdout_string);
                                let _ = stderr.read_to_string(&mut stderr_string);

                                if exit_status.success() {
                                    send_about_build_end(BuildResult::Successful(
                                        parse_cargo_build_result(stdout_string.as_bytes()),
                                    ));
                                } else {
                                    send_about_build_end(BuildResult::Failed(stderr_string));
                                }
                                return;
                            }
                            Err(error) => {
                                send_about_build_end(BuildResult::Failed(error.to_string()));
                                println!("error on wait_with_output: {}", error);
                                return;
                            }
                        },
                        Err(error) => {
                            spawned_process.kill().unwrap();
                            println!("error on try_wait: {}", error);
                            send_about_build_end(BuildResult::Failed(error.to_string()));
                            return;
                        }
                    };
                }
            }
        });

        (builder, result_receiver)
    }

    fn spawn_build_process(
        build_option: &BuildOption,
    ) -> Result<Child, Box<dyn std::error::Error>> {
        match build_option.platform {
            BuildPlatform::WasmWeb => Ok(Command::new("wasm-pack")
                .args([
                    "build",
                    "--target",
                    "no-modules",
                    "--out-name",
                    "bundle",
                    "--dev",
                    build_option.project_root_path.to_str().unwrap(),
                    "--",
                    "--message-format",
                    "json",
                ])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()?),
        }
    }
}

pub struct CargoBuildResult {
    pub warning_messages: Vec<ErrorMessage>,
    pub error_messages: Vec<ErrorMessage>,
    pub other_messages: Vec<ErrorMessage>,
    pub is_successful: bool,
}

fn parse_cargo_build_result(stdout: &[u8]) -> CargoBuildResult {
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

    CargoBuildResult {
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
