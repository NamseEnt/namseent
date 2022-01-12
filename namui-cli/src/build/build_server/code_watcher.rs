use crate::debug_println;
use cargo_metadata::MetadataCommand;
use futures::{executor::block_on, lock::Mutex};
use notify::{DebouncedEvent, INotifyWatcher, RecursiveMode, Watcher};
use regex::Regex;
use std::{collections::HashSet, path::PathBuf, sync::Arc, thread, time::Duration};
use tokio::sync::oneshot::{self, Sender};

struct CodeWatcherContext {
    was_changed: bool,
    sender: Option<Sender<()>>,
}

pub struct CodeWatcher {
    watcher: INotifyWatcher,
    context: Arc<Mutex<CodeWatcherContext>>,
    watching_paths: HashSet<String>,
    manifest_path: String,
}

impl CodeWatcher {
    pub fn new(manifest_path: String) -> Self {
        let context = Arc::new(Mutex::new(CodeWatcherContext {
            was_changed: true,
            sender: None,
        }));
        let mut root_watching_path = PathBuf::from(&manifest_path);
        root_watching_path.pop();
        let root_watching_path = root_watching_path.join("src").to_string_lossy().to_string();

        let (watcher_sender, watcher_receiver) = std::sync::mpsc::channel::<DebouncedEvent>();
        let mut watcher = notify::watcher(watcher_sender, Duration::from_secs(1)).unwrap();
        if let Err(error) = watcher.watch(root_watching_path, RecursiveMode::Recursive) {
            panic!("watch failed {:?}", error);
        }

        let context_clone = context.clone();
        thread::spawn(move || -> ! {
            loop {
                match watcher_receiver.recv() {
                    Ok(event) => {
                        match event {
                            DebouncedEvent::Create(_)
                            | DebouncedEvent::Remove(_)
                            | DebouncedEvent::Rename(_, _)
                            | DebouncedEvent::Write(_) => {
                                debug_println!("CodeWatcher: locking code_watcher.context...");
                                let mut context = block_on(context_clone.lock());
                                debug_println!("CodeWatcher: code_watcher.context locked");

                                match context.sender.take() {
                                    Some(sender) => {
                                        if let Err(_) = sender.send(()) {
                                            panic!("send failed in watcher")
                                        }
                                    }
                                    None => {
                                        context.was_changed = true;
                                    }
                                }
                            }
                            _ => (),
                        };
                    }
                    Err(error) => eprintln!("{:?}", error),
                }
            }
        });

        Self {
            watcher,
            context,
            watching_paths: HashSet::new(),
            manifest_path,
        }
    }

    pub async fn wait_for_change(&self) {
        match self.check_changed_from_last_check().await {
            true => (),
            false => {
                let receiver = self.register_sender().await;
                match receiver.await {
                    Ok(_) => (),
                    Err(error) => panic!("code_watcher oneshot receive error: {:?}", error),
                }
            }
        }
    }

    async fn check_changed_from_last_check(&self) -> bool {
        debug_println!("check_changed: locking code_watcher.context...");
        let mut context = self.context.lock().await;
        debug_println!("check_changed: code_watcher.context locked");
        match context.was_changed {
            true => {
                context.was_changed = false;
                true
            }
            false => false,
        }
    }

    async fn register_sender(&self) -> oneshot::Receiver<()> {
        let (sender, receiver) = oneshot::channel::<()>();
        debug_println!("register_sender: locking code_watcher.context...");
        let mut context = self.context.lock().await;
        debug_println!("register_sender: code_watcher.context locked");
        match context.sender {
            Some(_) => unreachable!(),
            None => context.sender = Some(sender),
        }
        receiver
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
