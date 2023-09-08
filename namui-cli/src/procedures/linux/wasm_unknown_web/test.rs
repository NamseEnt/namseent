use crate::*;
use cargo_metadata::MetadataCommand;
use regex::Regex;
use std::{
    collections::HashSet,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    str::FromStr,
};

pub fn test(manifest_path: &Path) -> Result<()> {
    let source_root_directory_to_bind =
        find_source_root_directory_to_bind_to_podman(manifest_path)?;
    let source_bind_path = PathBuf::from_str("/namui-test")?;

    let manifest_path_in_podman = get_manifest_path_in_podman(
        manifest_path,
        &source_root_directory_to_bind,
        &source_bind_path,
    );

    let cargo_directory = get_cargo_directory();
    let cargo_directory_of_podman = PathBuf::from_str("/usr/local/cargo")?;

    let cargo_cache_bind_directory_tuples = ["registry/index", "registry/cache", "git/db"]
        .iter()
        .map(|cache_path_suffix| {
            (
                cargo_directory.join(cache_path_suffix),
                cargo_directory_of_podman.join(cache_path_suffix),
            )
        })
        .filter(|(cargo_cache_path, _)| cargo_cache_path.exists());

    let mut bind_directory_tuples: Vec<_> = [(source_root_directory_to_bind, source_bind_path)]
        .into_iter()
        .chain(cargo_cache_bind_directory_tuples)
        .collect();

    let sccache_host_path = match std::env::var("SCCACHE_DIR") {
        Ok(sccache_dir) => PathBuf::from_str(&sccache_dir)?,
        Err(_) => PathBuf::from_str("/root/.cache/sccache")?,
    };
    if sccache_host_path.exists() {
        let sccache_bind_directory_tuple = (
            PathBuf::from_str(&format!("{}/.cache/sccache", std::env::var("HOME")?))?,
            sccache_host_path,
        );

        bind_directory_tuples.push(sccache_bind_directory_tuple);
    }

    let bind_args: Vec<String> = bind_directory_tuples
        .into_iter()
        .map(|(source, target)| {
            format!("{}:{}", source.to_str().unwrap(), target.to_str().unwrap())
        })
        .fold(vec![], |mut acc, bind_arg| {
            acc.push("--volume".to_string());
            acc.push(bind_arg);
            acc
        });

    let directory = manifest_path_in_podman
        .parent()
        .expect("No parent directory found");

    let rust_flags = std::env::var("RUSTFLAGS").unwrap_or_else(|_| "".to_string());
    let command_to_pass_to_podman = [
        "rustc --version".to_string(),
        format!(
            "RUSTFLAGS=\"{}\" wasm-pack test --headless --chrome {}",
            rust_flags,
            directory.to_str().unwrap()
        ),
    ]
    .into_iter()
    .collect::<Vec<String>>()
    .join("; ")
    .to_string();

    build_docker_image()?;

    let args = ["run", "--rm"]
        .into_iter()
        .chain(bind_args.iter().map(|s| s.as_ref()))
        .chain([
            "namui-test-host:latest",
            "sh",
            "-c",
            &command_to_pass_to_podman,
        ]);
    let result = Command::new("podman").args(args).status()?;

    if !result.success() {
        return Err(anyhow!("test failed"));
    }
    Ok(())
}

fn build_docker_image() -> Result<()> {
    let dockerfile = include_str!("../../../../../docker-images/namui-test-host/linux.Dockerfile");
    let args = ["build", "--tag", "namui-test-host:latest", "--quiet", "-"];
    let mut podman = Command::new("podman")
        .args(args)
        .stdin(Stdio::piped())
        .spawn()?;

    podman
        .stdin
        .as_ref()
        .unwrap()
        .write_all(dockerfile.as_bytes())?;

    let result = podman.wait()?;

    if !result.success() {
        anyhow::bail!("build docker image failed");
    }
    Ok(())
}

fn get_cargo_directory() -> PathBuf {
    let stdout = Command::new("which")
        .arg("cargo")
        .output()
        .expect("Failed to execute `which cargo`")
        .stdout;

    let path: PathBuf = String::from_utf8_lossy(&stdout).trim().into();

    path.parent()
        .expect("fail to get bin of cargo")
        .parent()
        .expect("fail to get parent of cargo bin")
        .to_path_buf()
}

fn get_manifest_path_in_podman(
    original_manifest_path: &Path,
    directory_to_bind: &PathBuf,
    bind_path: &Path,
) -> PathBuf {
    bind_path.join(
        original_manifest_path
            .strip_prefix(directory_to_bind)
            .expect("No prefix found"),
    )
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::get_manifest_path_in_podman;

    #[test]
    fn test_get_manifest_path_in_podman() {
        let manifest_path = get_manifest_path_in_podman(
            &PathBuf::from_str("/a/b/c/Cargo.toml").unwrap(),
            &PathBuf::from_str("/a").unwrap(),
            &PathBuf::from_str("/namseent").unwrap(),
        );

        assert_eq!(
            manifest_path,
            PathBuf::from_str("/namseent/b/c/Cargo.toml").unwrap()
        );

        let manifest_path = get_manifest_path_in_podman(
            &PathBuf::from_str("/a/b/c/Cargo.toml").unwrap(),
            &PathBuf::from_str("/a/b/c").unwrap(),
            &PathBuf::from_str("/namseent").unwrap(),
        );

        assert_eq!(
            manifest_path,
            PathBuf::from_str("/namseent/Cargo.toml").unwrap()
        );
    }
}

fn find_source_root_directory_to_bind_to_podman(manifest_path: &Path) -> Result<PathBuf> {
    let manifest_paths = get_all_path_dependencies_recursively(manifest_path)?;

    let source_root_directory = get_common_path(manifest_paths.iter());

    source_root_directory.ok_or_else(|| {
        anyhow!(
            "No common path found between {}",
            manifest_paths
                .iter()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>()
                .join(", ")
        )
    })
}

fn get_all_path_dependencies_recursively(manifest_path: &Path) -> Result<HashSet<PathBuf>> {
    let mut searching_manifest_paths = vec![manifest_path.to_path_buf()];

    let mut manifest_paths: HashSet<PathBuf> = HashSet::new();

    while let Some(manifest_path) = searching_manifest_paths.pop() {
        manifest_paths.insert(manifest_path.clone());

        let new_path_dependency_manifest_paths =
            get_path_dependency_manifest_paths(&manifest_path)?
                .into_iter()
                .filter(|path| !manifest_paths.contains(path));

        searching_manifest_paths.extend(new_path_dependency_manifest_paths);
    }

    Ok(manifest_paths)
}

fn get_common_path<'a>(iter: impl Iterator<Item = &'a PathBuf>) -> Option<PathBuf> {
    iter.fold(None, |last_common_path, manifest_path| {
        let parent = manifest_path.parent()?;
        if last_common_path.is_none() {
            return Some(parent.to_path_buf());
        }
        let last_common_path = last_common_path.unwrap();

        let new_common_path = get_common_path_of_two(&last_common_path, parent)?;

        let is_new_more_common =
            new_common_path.as_os_str().len() < last_common_path.as_os_str().len();

        let most_common_path = if is_new_more_common {
            new_common_path
        } else {
            last_common_path
        };

        Some(most_common_path)
    })
}

fn get_common_path_of_two(path_a: &Path, path_b: &Path) -> Option<PathBuf> {
    path_a
        .ancestors()
        .find(|ancestor| path_b.starts_with(ancestor))
        .map(|ancestor| ancestor.to_path_buf())
}

fn get_path_dependency_manifest_paths(manifest_path: &PathBuf) -> Result<Vec<PathBuf>> {
    let path_dependency_regex = Regex::new(r"\(path\+file://([^\)]+)\)$").unwrap();

    let metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;

    let mut manifest_paths = HashSet::new();

    if let Some(resolve) = metadata.resolve {
        for node in resolve.nodes {
            for dependency in node.deps {
                let path = dependency.pkg.repr;
                if let Some(captures) = path_dependency_regex.captures(&path) {
                    if let Some(matched_path) = captures.get(1) {
                        let manifest_path = format!("{}/Cargo.toml", matched_path.as_str());
                        let manifest_path = PathBuf::from_str(&manifest_path)?;
                        manifest_paths.insert(manifest_path);
                    }
                }
            }
        }
    }

    Ok(manifest_paths
        .iter()
        .map(|path| path.to_path_buf())
        .collect())
}
