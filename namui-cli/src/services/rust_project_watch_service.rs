use cargo_metadata::MetadataCommand;
use notify::{DebouncedEvent, PollWatcher, Watcher};
use regex::Regex;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    str::FromStr,
    thread,
    time::Duration,
};

use crate::debug_println;

pub struct RustProjectWatchService {}

impl RustProjectWatchService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) fn watch(
        &self,
        manifest_path: &Path,
        callback: impl Fn() + std::marker::Send + 'static + Clone,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut watching_paths = HashSet::new();

        let (watcher_sender, watcher_receiver) = std::sync::mpsc::channel::<_>();
        let mut watcher = PollWatcher::new(watcher_sender, Duration::from_millis(1000))?;

        loop {
            RustProjectWatchService::update_watching_paths(
                manifest_path,
                &mut watching_paths,
                &mut watcher,
            )?;

            let event = watcher_receiver.recv()?;
            debug_println!("watch event");
            match event {
                DebouncedEvent::Create(_)
                | DebouncedEvent::Remove(_)
                | DebouncedEvent::Rename(_, _)
                | DebouncedEvent::Write(_) => {
                    'flush: loop {
                        match watcher_receiver.try_recv() {
                            Ok(_) => (),
                            Err(error) => match error {
                                std::sync::mpsc::TryRecvError::Empty => break 'flush,
                                std::sync::mpsc::TryRecvError::Disconnected => {
                                    panic!("watcher closed {:?}", error)
                                }
                            },
                        }
                    }
                    thread::spawn(callback.clone());
                }
                _ => (),
            };
        }
    }

    fn update_watching_paths(
        manifest_path: &Path,
        watching_paths: &mut HashSet<PathBuf>,
        watcher: &mut impl Watcher,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let local_path_in_repr = Regex::new(r"\(path\+file://([^\)]+)\)$").unwrap();
        let project_src_path = manifest_path.parent().unwrap().join("src");
        let mut local_dependencies_src_paths = HashSet::new();

        let metadata = MetadataCommand::new()
            .manifest_path(&manifest_path)
            .exec()?;

        if let Some(resolve) = metadata.resolve {
            for node in resolve.nodes {
                for dependency in node.deps {
                    let path = dependency.pkg.repr;
                    if let Some(captures) = local_path_in_repr.captures(&path) {
                        if let Some(matched_path) = captures.get(1) {
                            local_dependencies_src_paths
                                .insert(PathBuf::from_str(matched_path.as_str())?.join("src"));
                        }
                    }
                }
            }
        }

        let watched_paths = watching_paths.clone();
        let next_watching_paths = local_dependencies_src_paths
            .union(&HashSet::from_iter([project_src_path].iter().cloned()))
            .cloned()
            .collect::<HashSet<_>>();

        let unwatch_paths = watched_paths
            .difference(&next_watching_paths)
            .collect::<Vec<_>>();

        let new_watch_paths = next_watching_paths
            .difference(&watched_paths)
            .collect::<Vec<_>>();

        for path in unwatch_paths {
            debug_println!("update_paths: unwatching {:?}", path);
            watcher.unwatch(path)?;
            watching_paths.remove(path);
        }

        for path in new_watch_paths {
            debug_println!("update_paths: watching {:?}", path);
            watcher.watch(path, notify::RecursiveMode::Recursive)?;
            watching_paths.insert(path.clone());
        }

        Ok(())
    }
}
