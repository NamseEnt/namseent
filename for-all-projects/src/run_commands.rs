use crate::cli::Run;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::{fs, io::AsyncReadExt, process};

pub async fn run_commands_in_parallel(run: Run, cargo_project_dirs: Vec<PathBuf>) -> Result<()> {
    let mut join_set = tokio::task::JoinSet::new();

    let throttle = std::thread::available_parallelism()?.get();

    let commands = filter_requested_commands(run);

    if commands.is_empty() {
        anyhow::bail!("No command is requested");
    }

    for command in commands {
        for cargo_project_dir in &cargo_project_dirs {
            join_set.spawn({
                let cargo_project_dir = cargo_project_dir.clone();
                let command = command.clone();
                async move {
                    let command_str = command.as_str(&cargo_project_dir).await?;
                    run_command(cargo_project_dir, &command_str).await
                }
            });
            if join_set.len() >= throttle {
                join_set.join_next().await.unwrap()??;
            }
        }

        while let Some(result) = join_set.join_next().await {
            result??;
        }
    }

    assert!(join_set.join_next().await.is_none());

    Ok(())
}

fn filter_requested_commands(run: Run) -> Vec<Command> {
    let mut commands = vec![];
    if run.command.clean {
        commands.push(Command::Clean);
    }
    if run.command.update {
        commands.push(Command::Update);
    }
    if run.command.metadata {
        commands.push(Command::Metadata);
    }
    if run.command.check {
        commands.push(Command::Check);
    }
    if run.command.fmt {
        commands.push(Command::Fmt);
    }
    if run.command.fix {
        commands.push(Command::Fix);
    }
    if run.command.clippy {
        commands.push(Command::Clippy);
    }
    if run.command.clippy_fix {
        commands.push(Command::ClippyFix);
    }
    if run.command.test {
        commands.push(Command::Test);
    }
    if let Some(command) = run.command.custom {
        commands.push(Command::Custom { command });
    }
    commands
}

#[derive(Clone)]
enum Command {
    Clean,
    Update,
    Metadata,
    Check,
    Fmt,
    Fix,
    Clippy,
    ClippyFix,
    Test,
    Custom { command: String },
}

impl Command {
    async fn as_str(&self, cargo_project_dir: &Path) -> Result<String> {
        Ok(match self {
            Command::Clean => "cargo clean".to_string(),
            Command::Update => "cargo update".to_string(),
            Command::Metadata => "cargo metadata".to_string(),
            Command::Check => "cargo check".to_string(),
            Command::Fmt => "cargo fmt".to_string(),
            Command::Fix => "cargo fix --allow-dirty --allow-staged".to_string(),
            Command::Clippy => "cargo clippy".to_string(),
            Command::ClippyFix => "cargo clippy --fix --allow-dirty --allow-staged".to_string(),
            Command::Test => get_test_command(cargo_project_dir).await?,
            Command::Custom { command } => command.clone(),
        })
    }
}

async fn run_command(cargo_project_dir: PathBuf, command: &str) -> Result<()> {
    let mut split = command.split_ascii_whitespace();

    let mut child = process::Command::new(split.next().unwrap())
        .args(split)
        .current_dir(&cargo_project_dir)
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;

    // NOTE: This is for multi-thread locked-printing.
    let print_output = format!("Running `{command}` in {cargo_project_dir:?}");
    println!("{}", print_output);

    let mut stdout_pipe = child.stdout.take().unwrap();
    let mut stderr_pipe = child.stderr.take().unwrap();

    let mut stdout_buf = vec![];
    let mut stdout_eof = false;
    let mut stderr_buf = vec![];
    let mut stderr_eof = false;

    let mut lines = vec![];

    fn push_lines(lines: &mut Vec<Vec<u8>>, buf: &mut Vec<u8>) {
        while let Some(linebreak_index) = buf.iter().position(|x| *x == b'\n') {
            let line = buf.drain(..=linebreak_index).collect::<Vec<_>>();
            lines.push(line);
        }
    }

    while !stdout_eof || !stderr_eof {
        if stdout_eof {
            let count = stderr_pipe.read_buf(&mut stderr_buf).await?;
            if count == 0 {
                stderr_eof = true;
                lines.push(std::mem::take(&mut stderr_buf));
            } else {
                push_lines(&mut lines, &mut stderr_buf)
            }
        } else if stderr_eof {
            let count = stdout_pipe.read_buf(&mut stdout_buf).await?;
            if count == 0 {
                stdout_eof = true;
                lines.push(std::mem::take(&mut stdout_buf));
            } else {
                push_lines(&mut lines, &mut stdout_buf)
            }
        } else {
            tokio::select! {
                count = stdout_pipe.read_buf(&mut stdout_buf) => {
                    if count? == 0 {
                        stdout_eof = true;
                        lines.push(std::mem::take(&mut stdout_buf));
                    } else {
                        push_lines(&mut lines, &mut stdout_buf)
                    }
                },
                count = stderr_pipe.read_buf(&mut stderr_buf) => {
                    if count? == 0 {
                        stderr_eof = true;
                        lines.push(std::mem::take(&mut stderr_buf));
                    } else {
                        push_lines(&mut lines, &mut stderr_buf)
                    }
                }
            };
        }
    }

    child.wait().await?;

    let output = child.wait_with_output().await?;

    if !output.status.success() {
        let print_output = format!(
            "Failed to run `{command}` in {cargo_project_dir:?}\n{}",
            lines
                .iter()
                .map(|line| String::from_utf8_lossy(line))
                .collect::<Vec<_>>()
                .join("\n")
        );
        eprintln!("{}", print_output);
        anyhow::bail!("Failed to run `{command}` in {cargo_project_dir:?}");
    }

    let print_output = format!("Finished `{command}` in {cargo_project_dir:?}");
    println!("{print_output}");

    Ok(())
}

async fn get_test_command(cargo_project_dir: &Path) -> Result<String> {
    let cargo_toml_str = fs::read_to_string(cargo_project_dir.join("Cargo.toml")).await?;

    let cargo_toml = cargo_toml_str.parse::<toml::Value>()?;

    let package = cargo_toml
        .get("package")
        .ok_or_else(|| anyhow::anyhow!("No [package] table in {cargo_toml_str}"))?
        .as_table()
        .ok_or_else(|| anyhow::anyhow!("Failed to parse [package] table in {cargo_toml_str}"))?;

    let custom_test_script = (|| package.get("metadata")?.get("test")?.as_str())();
    if let Some(custom_test_script) = custom_test_script {
        let (command, _args) = custom_test_script
            .split_once(' ')
            .unwrap_or((custom_test_script, ""));
        if ["/", "."].iter().any(|x| command.contains(x)) {
            return Ok(format!("bash {custom_test_script}"));
        } else {
            return Ok(custom_test_script.to_string());
        }
    }

    let is_namui_project = (|| package.get("metadata")?.get("namui")?.as_bool())().unwrap_or(false);
    if is_namui_project {
        return Ok("namui test".to_string());
    }

    Ok("cargo test".to_string())
}
