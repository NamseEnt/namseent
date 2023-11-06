use crate::*;
use crate::{cli::Target, debug_println, types::ErrorMessage};
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
    is_build_just_started: AtomicBool,
}
#[derive(Debug)]
pub enum BuildResult {
    Canceled,
    Successful(CargoBuildResult),
    Failed(String), // It's not failed if cargo build result has error messages.
}

#[derive(Clone, Debug)]
pub struct BuildOption {
    pub dist_path: PathBuf,
    pub project_root_path: PathBuf,
    pub target: Target,
    pub watch: bool,
}

impl RustBuildService {
    pub(crate) fn new() -> Self {
        Self {
            builder: Mutex::new(None),
            is_build_just_started: AtomicBool::new(false),
        }
    }

    pub(crate) async fn cancel_and_start_build(&self, build_option: &BuildOption) -> BuildResult {
        if self.is_build_just_started.swap(true, Ordering::Relaxed) {
            return BuildResult::Canceled;
        }

        let mut result_receiver = {
            let mut builder_lock = self.builder.lock().unwrap();
            if let Some(builder) = builder_lock.take() {
                builder.cancel();
            }

            self.is_build_just_started.store(false, Ordering::Relaxed);
            let (builder, result_receiver) = CancelableBuilder::start(build_option);
            *builder_lock = Some(builder);
            result_receiver
        };

        result_receiver.recv().await.unwrap()
    }
}

struct CancelableBuilder {
    is_cancel_requested: AtomicBool,
    is_canceled: AtomicBool,
}

impl CancelableBuilder {
    pub fn cancel(&self) {
        if self.is_canceled.load(Ordering::Relaxed) {
            return;
        }

        self.is_cancel_requested.store(true, Ordering::Relaxed);
        debug_println!("build cancel requested");

        loop {
            thread::sleep(Duration::from_millis(100));

            if self.is_canceled.load(Ordering::Relaxed) {
                break;
            }
        }
    }

    pub fn start(
        build_option: &BuildOption,
    ) -> (
        Arc<CancelableBuilder>,
        tokio::sync::mpsc::Receiver<BuildResult>,
    ) {
        let builder = Arc::new(Self {
            is_cancel_requested: AtomicBool::new(false),
            is_canceled: AtomicBool::new(false),
        });
        let build_option = build_option.clone();

        let builder_thread_fn = {
            move |builder: Arc<Self>| -> Result<BuildResult> {
                let mut spawned_process = Self::spawn_build_process(&build_option)?;

                let mut stdout = spawned_process.stdout.take().unwrap();
                let mut stderr = spawned_process.stderr.take().unwrap();
                let stdout_reading_thread = thread::spawn(move || {
                    let mut string = String::new();
                    stdout.read_to_string(&mut string).unwrap();
                    string
                });
                let stderr_reading_thread = thread::spawn(move || {
                    let mut string = String::new();
                    stderr.read_to_string(&mut string).unwrap();
                    string
                });

                loop {
                    thread::sleep(Duration::from_millis(100));

                    if builder.is_cancel_requested.load(Ordering::Relaxed) {
                        debug_println!("cancel requested received");
                        spawned_process.kill()?;
                        return Ok(BuildResult::Canceled);
                    }

                    match spawned_process.try_wait()? {
                        None => {}
                        Some(exit_status) => {
                            let cargo_outputs = stdout_reading_thread
                                .join()
                                .expect("fail to get stdout from thread");

                            if cargo_outputs.is_empty() {
                                return Err(anyhow!(
                                    "cargo build failed {stderr}",
                                    stderr = stderr_reading_thread.join().unwrap()
                                ));
                            }
                            match parse_cargo_build_result(cargo_outputs.as_bytes()) {
                                Ok(result) => {
                                    if result.is_successful && !exit_status.success() {
                                        return Err(anyhow!(
                                            "build process exited with code {exit_status}\nstderr: {stderr}",
                                            stderr = stderr_reading_thread.join().unwrap()
                                        ));
                                    }
                                    return Ok(BuildResult::Successful(result));
                                }
                                Err(_) => {
                                    let error = stderr_reading_thread
                                        .join()
                                        .expect("fail to get stderr from thread");

                                    return Ok(BuildResult::Failed(error));
                                }
                            }
                        }
                    };
                }
            }
        };

        let (result_sender, result_receiver) = tokio::sync::mpsc::channel(1048576);

        tokio::spawn({
            let builder = builder.clone();
            async move {
                let build_result = match builder_thread_fn(builder.clone()) {
                    Ok(result) => result,
                    Err(error) => BuildResult::Failed(error.to_string()),
                };
                result_sender.send(build_result).await.unwrap();
                builder.is_canceled.store(true, Ordering::Relaxed);
            }
        });

        (builder, result_receiver)
    }

    fn spawn_build_process(build_option: &BuildOption) -> Result<Child> {
        Ok(Command::new("wasm-pack")
            .args([
                "build",
                "--target",
                "no-modules",
                "--out-name",
                "bundle",
                "--dev",
                "--out-dir",
                build_option.dist_path.to_str().unwrap(),
                build_option.project_root_path.to_str().unwrap(),
                "--",
                "--message-format",
                "json",
            ])
            .envs(get_envs(build_option))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?)
    }
}

fn get_envs(build_option: &BuildOption) -> Vec<(&str, &str)> {
    let mut envs = match build_option.target {
        Target::WasmUnknownWeb => vec![
            ("NAMUI_CFG_TARGET_OS", "unknown"),
            ("NAMUI_CFG_TARGET_ENV", "web"),
            ("NAMUI_CFG_TARGET_ARCH", "wasm"),
        ],
        Target::WasmWindowsElectron => vec![
            ("NAMUI_CFG_TARGET_OS", "windows"),
            ("NAMUI_CFG_TARGET_ENV", "electron"),
            ("NAMUI_CFG_TARGET_ARCH", "wasm"),
        ],
        Target::WasmLinuxElectron => vec![
            ("NAMUI_CFG_TARGET_OS", "linux"),
            ("NAMUI_CFG_TARGET_ENV", "electron"),
            ("NAMUI_CFG_TARGET_ARCH", "wasm"),
        ],
    };

    if build_option.watch {
        envs.push(("NAMUI_CFG_WATCH_RELOAD", ""));
    }

    // NOTE: This may break build when user's platform doesn't support simd128.
    envs.push(("-C", "target-feature=+simd128"));

    envs
}

#[derive(Debug)]
pub struct CargoBuildResult {
    pub warning_messages: Vec<ErrorMessage>,
    pub error_messages: Vec<ErrorMessage>,
    pub other_messages: Vec<ErrorMessage>,
    pub is_successful: bool,
}

fn parse_cargo_build_result(stdout: &[u8]) -> Result<CargoBuildResult> {
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

    Ok(CargoBuildResult {
        warning_messages,
        error_messages,
        other_messages,
        is_successful,
    })
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
