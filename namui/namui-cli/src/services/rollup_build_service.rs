use crate::types::ErrorMessage;
use anyhow::{anyhow, Result};
use itertools::Itertools;
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

pub struct RollupBuildService {
    builder: Mutex<Option<Arc<CancelableBuilder>>>,
    is_build_just_started: AtomicBool,
}

#[derive(Debug)]
pub enum BuildResult {
    Canceled,
    Successful(RollupBuildResult),
    Failed(String),
}

#[derive(Clone, Debug)]
pub struct BuildOption {
    pub rollup_project_root_path: PathBuf,
    pub development: bool,
}

impl RollupBuildService {
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
                        spawned_process.kill()?;
                        return Ok(BuildResult::Canceled);
                    }

                    match spawned_process.try_wait()? {
                        None => {}
                        Some(exit_status) => {
                            let rollup_outputs = stdout_reading_thread
                                .join()
                                .expect("fail to get stdout from thread");

                            if !exit_status.success() {
                                return Err(anyhow!(
                                    "rollup build failed {stderr}",
                                    stderr = stderr_reading_thread.join().unwrap()
                                ));
                            }
                            match parse_rollup_build_result(rollup_outputs) {
                                Ok(result) => {
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
        Ok(Command::new("npm")
            .args([
                "run",
                match build_option.development {
                    true => "build:dev",
                    false => "build:prod",
                },
            ])
            .current_dir(build_option.rollup_project_root_path.clone())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?)
    }
}

#[derive(Debug)]
pub struct RollupBuildResult {
    pub error_messages: Vec<ErrorMessage>,
}

fn parse_rollup_build_result(stdout: String) -> Result<RollupBuildResult> {
    const ROLLUP_BUILD_MESSAGE_PREFIX: &str = "//ROLLUP_BUILD_MESSAGE//:";
    let mut error_messages = Vec::new();

    let lines = stdout.lines();
    for line in lines
        .filter_map(|line| match line.starts_with(ROLLUP_BUILD_MESSAGE_PREFIX) {
            true => Some(line.trim_start_matches(ROLLUP_BUILD_MESSAGE_PREFIX)),
            false => None,
        })
        .unique()
    {
        let error_message = serde_json::from_str::<RollupBuildMessage>(line)?;
        match error_message.level {
            RollupBuildMessageLevel::Warn => {}
            _ => continue,
        }
        let absolute_file = error_message
            .loc
            .as_ref()
            .map(|loc| loc.file.clone().unwrap_or_default())
            .unwrap_or_default();
        let error_message = ErrorMessage {
            relative_file: absolute_file.clone(),
            absolute_file,
            line: error_message.loc.as_ref().map_or(0, |loc| loc.line),
            column: error_message.loc.as_ref().map_or(0, |loc| loc.column),
            text: error_message.message,
        };
        error_messages.push(error_message);
    }

    Ok(RollupBuildResult { error_messages })
}

#[allow(dead_code)]
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct RollupBuildMessage {
    level: RollupBuildMessageLevel,
    loc: Option<RollupBuildMessageLocation>,
    message: String,

    #[serde(skip)]
    binding: Option<String>,
    #[serde(skip)]
    cause: Option<()>,
    #[serde(skip)]
    code: Option<String>,
    #[serde(skip)]
    exporter: Option<String>,
    #[serde(skip)]
    frame: Option<String>,
    #[serde(skip)]
    hook: Option<String>,
    #[serde(skip)]
    id: Option<String>,
    #[serde(skip)]
    ids: Option<Vec<String>>,
    #[serde(skip)]
    meta: Option<()>,
    #[serde(skip)]
    names: Option<Vec<String>>,
    #[serde(skip)]
    plugin: Option<String>,
    #[serde(skip, rename = "pluginCode")]
    plugin_code: Option<()>,
    #[serde(skip)]
    pos: Option<i64>,
    #[serde(skip)]
    reexporter: Option<String>,
    #[serde(skip)]
    stack: Option<String>,
    #[serde(skip)]
    url: Option<String>,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum RollupBuildMessageLevel {
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "debug")]
    Debug,
}
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct RollupBuildMessageLocation {
    column: usize,
    file: Option<String>,
    line: usize,
}
