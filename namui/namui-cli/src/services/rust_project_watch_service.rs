use crate::*;
use cargo_metadata::MetadataCommand;
use notify::{Config, RecommendedWatcher, Watcher};
use regex::Regex;
use std::{collections::HashSet, path::PathBuf, str::FromStr};

pub struct RustProjectWatchService {
    manifest_path: PathBuf,
    watcher: RecommendedWatcher,
    watcher_receiver: tokio::sync::mpsc::UnboundedReceiver<notify::Result<notify::Event>>,
    watching_paths: HashSet<PathBuf>,
}

const WATCHING_ITEMS_IN_PROJECT: [&str; 3] = ["src", "Cargo.toml", ".namuibundle"];

impl RustProjectWatchService {
    pub(crate) fn new(manifest_path: impl AsRef<std::path::Path>) -> Result<Self> {
        let (watcher_sender, watcher_receiver) = tokio::sync::mpsc::unbounded_channel();
        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = watcher_sender.send(res);
            },
            Config::default(),
        )?;

        Ok(Self {
            manifest_path: manifest_path.as_ref().to_path_buf(),
            watcher,
            watcher_receiver,
            watching_paths: HashSet::new(),
        })
    }

    pub(crate) async fn next(&mut self) -> Result<Option<()>> {
        loop {
            self.update_watching_paths().await?;

            let event = self.watcher_receiver.recv().await.unwrap().unwrap();
            match event.kind {
                notify::EventKind::Create(_)
                | notify::EventKind::Modify(_)
                | notify::EventKind::Remove(_) => {
                    'flush: loop {
                        match self.watcher_receiver.try_recv() {
                            Ok(_) => (),
                            Err(error) => match error {
                                tokio::sync::mpsc::error::TryRecvError::Empty => break 'flush,
                                tokio::sync::mpsc::error::TryRecvError::Disconnected => {
                                    panic!("watcher closed {error:?}")
                                }
                            },
                        }
                    }
                    return Ok(Some(()));
                }
                _ => {}
            };
        }
    }

    async fn update_watching_paths(&mut self) -> Result<()> {
        let local_path_in_repr = Regex::new(r"path\+file://([^#]+)")?;
        let project_root_path = self.manifest_path.parent().unwrap();
        let mut local_dependencies_root_paths = HashSet::new();

        let metadata = tokio::task::spawn_blocking({
            let manifest_path = self.manifest_path.clone();
            move || MetadataCommand::new().manifest_path(manifest_path).exec()
        })
        .await??;

        if let Some(resolve) = metadata.resolve {
            for node in resolve.nodes {
                for dependency in node.deps {
                    let path = dependency.pkg.repr;
                    if let Some(captures) = local_path_in_repr.captures(&path)
                        && let Some(matched_path) = captures.get(1)
                    {
                        local_dependencies_root_paths
                            .insert(PathBuf::from_str(matched_path.as_str())?);
                    }
                }
            }
        }

        let watched_paths = self.watching_paths.clone();
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
            self.watcher.unwatch(path)?;
            self.watching_paths.remove(path);
        }

        for path in new_watch_paths {
            if path.exists() {
                self.watcher.watch(path, notify::RecursiveMode::Recursive)?;
                self.watching_paths.insert(path.clone());
            }
        }

        Ok(())
    }
}
