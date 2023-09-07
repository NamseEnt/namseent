mod cli;

use anyhow::Result;
use clap::Parser;
use cli::*;
use futures::{future::BoxFuture, FutureExt};
use ignore::gitignore::*;
use std::{
    env::current_dir,
    path::{Path, PathBuf},
};
use tokio::{fs, process};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let cargo_project_dirs =
        find_cargo_project_dirs(current_dir().unwrap(), HasParentGitRoot::Unknown).await?;

    run_commands_in_parallel(cli, cargo_project_dirs).await?;

    Ok(())
}

#[derive(Clone)]
enum HasParentGitRoot {
    Yes { git_root: PathBuf },
    No,
    Unknown,
}

unsafe impl Send for HasParentGitRoot {}

fn find_cargo_project_dirs(
    dir: PathBuf,
    mut has_parent_git_root: HasParentGitRoot,
) -> BoxFuture<'static, Result<Vec<PathBuf>>> {
    async move {
        let am_i_git_root = fs::try_exists(dir.join(".git")).await?;

        if am_i_git_root {
            has_parent_git_root = HasParentGitRoot::Yes {
                git_root: dir.clone(),
            };
        } else if let HasParentGitRoot::Unknown = has_parent_git_root {
            has_parent_git_root = match find_git_root_ancesstor(&dir).await? {
                Some(git_root) => HasParentGitRoot::Yes { git_root },
                None => HasParentGitRoot::No,
            }
        }

        let gitignore = match &has_parent_git_root {
            HasParentGitRoot::Yes { git_root } => build_gitignore(git_root, &dir)?,
            HasParentGitRoot::No => Gitignore::empty(),
            HasParentGitRoot::Unknown => unreachable!(),
        };

        let sub_dirs = {
            let mut sub_dirs = vec![];

            let mut entries = fs::read_dir(&dir).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if entry.file_type().await?.is_dir() && !gitignore.matched(&path, true).is_ignore()
                {
                    sub_dirs.push(path);
                }
            }

            sub_dirs
        };

        let mut join_set = tokio::task::JoinSet::new();
        for sub_dir in sub_dirs {
            let has_parent_git_root = has_parent_git_root.clone();
            join_set
                .spawn(async move { find_cargo_project_dirs(sub_dir, has_parent_git_root).await });
        }

        let mut cargo_project_dirs = vec![];

        while let Some(result) = join_set.join_next().await {
            cargo_project_dirs.extend(result??);
        }

        let am_i_cargo_porject = fs::try_exists(dir.join("Cargo.toml")).await?;
        if am_i_cargo_porject {
            cargo_project_dirs.push(dir);
        }

        Ok(cargo_project_dirs)
    }
    .boxed()
}

/// git_root should be a ancestor of the current_dir.
fn build_gitignore(git_root: &Path, current_dir: &Path) -> Result<Gitignore> {
    let mut gitignore_builder = GitignoreBuilder::new(current_dir.join(".gitignore"));

    let mut parent = current_dir.parent().unwrap();
    while parent != git_root {
        gitignore_builder.add(parent.join(".gitignore"));
        parent = parent.parent().unwrap();
    }

    let gitignore = gitignore_builder.build()?;

    Ok(gitignore)
}

async fn find_git_root_ancesstor(dir: &Path) -> Result<Option<PathBuf>> {
    let mut dir_option = Some(dir);

    while let Some(dir) = dir_option {
        if fs::try_exists(dir.join(".git")).await? {
            return Ok(Some(dir.to_path_buf()));
        }
        dir_option = dir.parent();
    }

    Ok(None)
}

async fn run_commands_in_parallel(cli: Cli, cargo_project_dirs: Vec<PathBuf>) -> Result<()> {
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
    macro_rules! run_command {
        (
            $($command:ident),*
        ) => {
            $(
                if cli.$command {
                    let mut command = process::Command::new("cargo")
                        .arg(stringify!($command))
                        .current_dir(&cargo_project_dir)
                        .spawn()?;

                    command.wait().await?;
                }
            )*
        };
    }
    run_command!(check, clean, metadata, update, clippy, fmt);

    Ok(())
}
