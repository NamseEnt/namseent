use std::{
    collections::HashSet,
    error::Error,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};

use cargo_metadata::MetadataCommand;
use regex::Regex;

pub fn test(manifest_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let source_root_directory_to_bind =
        find_source_root_directory_to_bind_to_docker(manifest_path)?;
    let source_bind_path = PathBuf::from_str("/namui-test")?;

    let manifest_path_in_docker = get_manifest_path_in_docker(
        manifest_path,
        &source_root_directory_to_bind,
        &source_bind_path,
    );

    let cargo_directory = get_cargo_directory();
    let cargo_directory_of_docker = PathBuf::from_str("/usr/local/cargo")?;

    let cargo_cache_bind_directory_tuples = ["/registry/index/", "/registry/cache/", "/git/db/"]
        .iter()
        .map(|cache_path_suffix| {
            (
                cargo_directory.join(cache_path_suffix),
                cargo_directory_of_docker.join(cache_path_suffix),
            )
        });

    let bind_directory_tuples = [(source_root_directory_to_bind, source_bind_path)]
        .into_iter()
        .chain(cargo_cache_bind_directory_tuples);

    let bind_args: Vec<String> = bind_directory_tuples
        .map(|(source, target)| {
            format!(
                "type=bind,source={},target={}",
                source.to_str().unwrap(),
                target.to_str().unwrap()
            )
        })
        .fold(vec![], |mut acc, bind_arg| {
            acc.push("--mount".to_string());
            acc.push(bind_arg);
            acc
        });

    let directory = manifest_path_in_docker
        .parent()
        .expect("No parent directory found");

    let args = ["run", "--rm"]
        .into_iter()
        .chain(bind_args.iter().map(|s| s.as_ref()))
        .chain([
            "ghcr.io/namseent/namui-test-host:latest",
            "wasm-pack",
            "test",
            "--headless",
            "--chrome",
            directory.to_str().unwrap(),
        ]);
    let result = Command::new("docker").args(args).status()?;

    if !result.success() {
        return Err(format!("test failed").into());
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

fn get_manifest_path_in_docker(
    original_manifest_path: &PathBuf,
    directory_to_bind: &PathBuf,
    bind_path: &PathBuf,
) -> PathBuf {
    bind_path.join(
        original_manifest_path
            .strip_prefix(directory_to_bind)
            .expect("No prefix found")
            .to_path_buf(),
    )
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::get_manifest_path_in_docker;

    #[test]
    fn test_get_manifest_path_in_docker() {
        let manifest_path = get_manifest_path_in_docker(
            &PathBuf::from_str("/a/b/c/Cargo.toml").unwrap(),
            &PathBuf::from_str("/a").unwrap(),
            &PathBuf::from_str("/namseent").unwrap(),
        );

        assert_eq!(
            manifest_path,
            PathBuf::from_str("/namseent/b/c/Cargo.toml").unwrap()
        );

        let manifest_path = get_manifest_path_in_docker(
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

fn find_source_root_directory_to_bind_to_docker(
    manifest_path: &PathBuf,
) -> Result<PathBuf, Box<dyn Error>> {
    let manifest_paths = get_all_path_dependencies_recursively(manifest_path)?;

    let source_root_directory = get_common_path(manifest_paths.iter());

    source_root_directory.ok_or_else(|| {
        format!(
            "No common path found between {}",
            manifest_paths
                .iter()
                .map(|p| p.to_str().unwrap())
                .collect::<Vec<_>>()
                .join(", ")
        )
        .into()
    })
}

fn get_all_path_dependencies_recursively(
    manifest_path: &PathBuf,
) -> Result<HashSet<PathBuf>, Box<dyn Error>> {
    let mut searching_manifest_paths = vec![manifest_path.clone()];

    let mut manifest_paths: HashSet<PathBuf> = HashSet::new();

    while searching_manifest_paths.len() > 0 {
        let manifest_path = searching_manifest_paths.pop().unwrap();
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

fn get_path_dependency_manifest_paths(
    manifest_path: &PathBuf,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let path_dependency_regex = Regex::new(r"\(path\+file://([^\)]+)\)$").unwrap();

    let metadata = MetadataCommand::new()
        .manifest_path(&manifest_path)
        .exec()?;

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
