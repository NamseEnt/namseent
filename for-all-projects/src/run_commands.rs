use crate::cli::Cli;
use anyhow::Result;
use std::path::PathBuf;
use tokio::{fs, process};

pub async fn run_commands_in_parallel(cli: Cli, cargo_project_dirs: Vec<PathBuf>) -> Result<()> {
    let mut join_set = tokio::task::JoinSet::new();

    for cargo_project_dir in cargo_project_dirs {
        join_set.spawn(async move { run_commands(cli, cargo_project_dir).await });
    }

    while let Some(result) = join_set.join_next().await {
        result??;
    }

    Ok(())
}

async fn run_commands(cli: Cli, cargo_project_dir: PathBuf) -> Result<()> {
    let mut commands = [
        (cli.clean, "cargo clean"),
        (cli.update, "cargo update"),
        (cli.metadata, "cargo metadata"),
        (cli.check, "cargo check"),
        (cli.fmt, "cargo fmt --allow-dirty --allow-staged"),
        (
            cli.clippy,
            "cargo clippy --fix --allow-dirty --allow-staged",
        ),
    ]
    .into_iter()
    .filter_map(|(flag, command)| if flag { Some(command) } else { None })
    .collect::<Vec<_>>();

    if cli.test {
        let command = if is_namui_project(&cargo_project_dir).await? {
            "namui test"
        } else {
            "cargo test"
        };
        commands.push(command);
    }

    for command in commands {
        let mut split = command.split_ascii_whitespace();
        let mut child = process::Command::new(split.next().unwrap())
            .args(split)
            .current_dir(&cargo_project_dir)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null()) // TODO: Show stderr and stdout when status is not success
            .spawn()?;

        println!(
            "{}",
            format!("Running `{command}` in {cargo_project_dir:?}",)
        );

        let status = child.wait().await?;

        if !status.success() {
            anyhow::bail!("Failed to run `{command}` in {cargo_project_dir:?}",);
        }

        println!(
            "{}",
            format!("Finished `{command}` in {cargo_project_dir:?}",)
        );
    }

    Ok(())
}

async fn is_namui_project(cargo_project_dir: &PathBuf) -> Result<bool> {
    let cargo_toml_path = cargo_project_dir.join("Cargo.toml");
    let cargo_toml_str = fs::read_to_string(&cargo_toml_path).await?;

    let cargo_toml = cargo_toml_str.parse::<toml::Value>()?;

    let package = cargo_toml
        .get("package")
        .ok_or_else(|| anyhow::anyhow!("No [package] table in {}", cargo_toml_path.display()))?
        .as_table()
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Failed to parse [package] table in {}",
                cargo_toml_path.display()
            )
        })?;

    let is_namui_project = package
        .get("namui")
        .and_then(|namui| namui.as_bool())
        .unwrap_or(false);

    Ok(is_namui_project)
}
