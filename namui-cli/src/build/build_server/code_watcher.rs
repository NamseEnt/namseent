use crate::debug_println;
use cargo_metadata::MetadataCommand;
use notify::{DebouncedEvent, INotifyWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::{collections::HashSet, path::PathBuf, sync::mpsc::Receiver, time::Duration};

pub struct CodeWatcher {
    watcher: INotifyWatcher,
    watching_paths: HashSet<String>,
    manifest_path: String,
    watcher_receiver: Receiver<DebouncedEvent>,
}

impl CodeWatcher {
    pub fn new(manifest_path: String) -> Self {
        let mut root_watching_path = PathBuf::from(&manifest_path);
        root_watching_path.pop();
        let root_watching_path = root_watching_path.join("src").to_string_lossy().to_string();

        let (watcher_sender, watcher_receiver) = std::sync::mpsc::channel::<DebouncedEvent>();
        let mut watcher = notify::watcher(watcher_sender, Duration::from_secs(1)).unwrap();
        if let Err(error) = watcher.watch(root_watching_path, RecursiveMode::Recursive) {
            panic!("watch failed {:?}", error);
        }

        Self {
            watcher,
            watching_paths: HashSet::new(),
            manifest_path,
            watcher_receiver,
        }
    }

    pub fn wait_for_change(&mut self) {
        loop {
            match self.watcher_receiver.recv() {
                Ok(event) => {
                    match event {
                        DebouncedEvent::Create(_)
                        | DebouncedEvent::Remove(_)
                        | DebouncedEvent::Rename(_, _)
                        | DebouncedEvent::Write(_) => {
                            'flush: loop {
                                match self.watcher_receiver.try_recv() {
                                    Ok(_) => (),
                                    Err(error) => match error {
                                        std::sync::mpsc::TryRecvError::Empty => break 'flush,
                                        std::sync::mpsc::TryRecvError::Disconnected => {
                                            panic!("watcher closed {:?}", error)
                                        }
                                    },
                                }
                            }
                            return;
                        }
                        _ => (),
                    };
                }
                Err(error) => eprintln!("{:?}", error),
            }
        }
    }

    pub fn update_watching_paths(&mut self) {
        let local_path_in_repr = Regex::new(r"\(path\+file://([^\)]+)\)$").unwrap();
        let mut new_paths: HashSet<String> = HashSet::new();
        if let Ok(metadata) = MetadataCommand::new()
            .manifest_path(&self.manifest_path)
            .exec()
        {
            if let Some(resolve) = metadata.resolve {
                for node in resolve.nodes {
                    for dependency in node.deps {
                        let path = dependency.pkg.repr;
                        if let Some(captures) = local_path_in_repr.captures(&path) {
                            if let Some(matched_path) = captures.get(1) {
                                new_paths.insert(format!("{}/src", matched_path.as_str()));
                            }
                        }
                    }
                }
            }
        }

        let mut new_watching_paths = self.watching_paths.clone();

        for watching_path in &self.watching_paths {
            match new_paths.contains(watching_path) {
                true => (),
                false => {
                    debug_println!("update_paths: unwatching {}", watching_path);
                    if let Err(error) = self.watcher.unwatch(watching_path) {
                        panic!("unwatch failed {:?}", error);
                    }
                    new_watching_paths.remove(watching_path);
                }
            }
        }

        for new_path in new_paths {
            match self.watching_paths.contains(&new_path) {
                true => (),
                false => {
                    debug_println!("update_paths: watching {}", new_path);
                    if let Err(error) = self.watcher.watch(&new_path, RecursiveMode::Recursive) {
                        panic!("watch failed {:?}", error);
                    }
                    new_watching_paths.insert(new_path);
                }
            }
        }

        self.watching_paths = new_watching_paths;
    }
}
