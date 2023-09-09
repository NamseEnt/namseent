use anyhow::Result;
use futures::{future::BoxFuture, FutureExt};
use ignore::gitignore::*;
use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn find_cargo_project_dirs(dir: PathBuf) -> Result<Vec<PathBuf>> {
    let git_root = find_git_root(&dir).await?;
    find_cargo_project_dirs_internal(dir, git_root).await
}

fn find_cargo_project_dirs_internal(
    dir: PathBuf,
    mut git_root: Option<PathBuf>,
) -> BoxFuture<'static, Result<Vec<PathBuf>>> {
    async move {
        let am_i_git_root = fs::try_exists(dir.join(".git")).await?;
        if am_i_git_root {
            git_root = Some(dir.clone())
        }

        let gitignores = match &git_root {
            Some(git_root) => {
                build_gitignores(git_root, &dir, &[".gitignore", ".projectsignore"]).await?
            }
            None => Gitignores::empty(),
        };

        let sub_dirs = {
            let mut sub_dirs = vec![];

            let mut entries = fs::read_dir(&dir).await?;
            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                if entry.file_name() != "git"
                    && entry.file_type().await?.is_dir()
                    && !gitignores.ignore(&path)
                {
                    sub_dirs.push(path);
                }
            }

            sub_dirs
        };

        let mut cargo_project_dirs = vec![];

        let mut join_set = tokio::task::JoinSet::new();
        for sub_dir in sub_dirs {
            let git_root = git_root.clone();
            join_set
                .spawn(async move { find_cargo_project_dirs_internal(sub_dir, git_root).await });
        }
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

struct Gitignores {
    ignores: Vec<Gitignore>,
}

impl Gitignores {
    fn empty() -> Self {
        Self { ignores: vec![] }
    }
    fn ignore(&self, dir: &Path) -> bool {
        self.ignores
            .iter()
            .any(|ignore| ignore.matched(dir, true).is_ignore())
    }
}

async fn build_gitignores(
    git_root: &Path,
    dir: &Path,
    ignore_file_names: &[&str],
) -> Result<Gitignores> {
    let dirs = {
        let mut dir = dir;
        let mut dirs = vec![];
        while dir != git_root {
            dirs.push(dir);
            dir = dir.parent().unwrap();
        }
        dirs.push(git_root);
        dirs
    };

    let ignore_file_paths = dirs
        .into_iter()
        .flat_map(|dir| {
            ignore_file_names
                .iter()
                .map(|ignore_file_name| dir.join(ignore_file_name))
        })
        .collect::<Vec<_>>();

    let mut ignores = vec![];
    for ignore_file_path in ignore_file_paths {
        if fs::try_exists(&ignore_file_path).await? {
            ignores.push(Gitignore::new(ignore_file_path).0);
        }
    }

    Ok(Gitignores { ignores })
}

async fn find_git_root(dir: &Path) -> Result<Option<PathBuf>> {
    let mut dir_option = Some(dir);

    while let Some(dir) = dir_option {
        if fs::try_exists(dir.join(".git")).await? {
            return Ok(Some(dir.to_path_buf()));
        }
        dir_option = dir.parent();
    }

    Ok(None)
}
