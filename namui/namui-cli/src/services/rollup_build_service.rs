use crate::types::ErrorMessage;
use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::path::PathBuf;
use tokio::process::Command;

#[derive(Clone, Debug)]
pub struct BuildOption {
    pub rollup_project_root_path: PathBuf,
    pub development: bool,
}

pub fn build(build_option: BuildOption) -> tokio::task::JoinHandle<Result<RollupBuildOutput>> {
    tokio::spawn(async move {
        let output = Command::new("npm")
            .args([
                "run",
                match build_option.development {
                    true => "build:dev",
                    false => "build:prod",
                },
            ])
            .current_dir(build_option.rollup_project_root_path)
            .output()
            .await?;

        let stderr = String::from_utf8(output.stderr)?;
        let stdout = String::from_utf8(output.stdout)?;

        if !output.status.success() {
            tokio::fs::write(
                "rollup_err.json",
                format!("stdout: {stdout}\n\nstderr: {stderr}"),
            )
            .await?;
            return Err(anyhow!("rollup build failed. check rollup_err.json",));
        }

        parse_rollup_build_output(stdout)
            .map_err(|err| anyhow!("Failed to parse rollup build result: {err} / {stderr}"))
    })
}

#[derive(Debug)]
pub struct RollupBuildOutput {
    pub error_messages: Vec<ErrorMessage>,
}

fn parse_rollup_build_output(stdout: String) -> Result<RollupBuildOutput> {
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

    Ok(RollupBuildOutput { error_messages })
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
