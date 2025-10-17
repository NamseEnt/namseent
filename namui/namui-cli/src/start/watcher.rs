use crate::*;
use cargo_metadata::MetadataCommand;
use notify::{Config, RecommendedWatcher, Watcher};
use regex::Regex;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    str::FromStr,
};

const WATCHING_ITEMS_IN_PROJECT: [&str; 2] = ["src", "Cargo.toml"];

pub fn start_watcher(
    manifest_path: impl AsRef<std::path::Path>,
    call_changed: impl Fn() + Send + Sync + 'static,
) {
    let manifest_path = manifest_path.as_ref().to_path_buf();

    std::thread::spawn(move || {
        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(
            move |result: std::result::Result<notify::Event, notify::Error>| {
                let Ok(event) = result else {
                    // ignore err
                    return;
                };
                tx.send(event).unwrap();
            },
            Config::default(),
        )
        .unwrap();

        let mut watching_paths = HashSet::new();
        update_watching_paths(&manifest_path, &mut watching_paths, &mut watcher).unwrap();

        while let Ok(event) = rx.recv() {
            let cargo_toml_changed = event.paths.iter().any(|path| path.ends_with("Cargo.toml"));
            if cargo_toml_changed {
                update_watching_paths(&manifest_path, &mut watching_paths, &mut watcher).unwrap();
            }

            call_changed();
        }
    });
}

fn update_watching_paths(
    manifest_path: &Path,
    watching_paths: &mut HashSet<PathBuf>,
    watcher: &mut RecommendedWatcher,
) -> Result<()> {
    let local_path_in_repr = Regex::new(r"path\+file://([^#]+)")?;
    let project_root_path = manifest_path.parent().unwrap();
    let mut local_dependencies_root_paths = HashSet::new();

    let metadata = MetadataCommand::new().manifest_path(manifest_path).exec()?;

    if let Some(resolve) = metadata.resolve {
        for node in resolve.nodes {
            for dependency in node.deps {
                let path = dependency.pkg.repr;
                if let Some(captures) = local_path_in_repr.captures(&path)
                    && let Some(matched_path) = captures.get(1)
                {
                    local_dependencies_root_paths.insert(PathBuf::from_str(matched_path.as_str())?);
                }
            }
        }
    }

    let watched_paths = watching_paths.clone();
    let next_watching_paths = local_dependencies_root_paths
        .union(&HashSet::from_iter([project_root_path.to_path_buf()]))
        .flat_map(|root_path| {
            WATCHING_ITEMS_IN_PROJECT
                .iter()
                .map(|watching_item| root_path.join(watching_item))
        })
        .collect::<HashSet<_>>();

    let unwatch_paths = watched_paths
        .difference(&next_watching_paths)
        .collect::<Vec<_>>();

    let new_watch_paths = next_watching_paths
        .difference(&watched_paths)
        .collect::<Vec<_>>();

    for path in unwatch_paths {
        watcher.unwatch(path)?;
        watching_paths.remove(path);
    }

    for path in new_watch_paths {
        if path.exists() {
            watcher.watch(path, notify::RecursiveMode::Recursive)?;
            watching_paths.insert(path.clone());
        }
    }

    Ok(())
}
