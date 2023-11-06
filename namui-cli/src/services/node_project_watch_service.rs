use crate::debug_println;
use crate::*;
use notify::{Config, RecommendedWatcher, Watcher};
use std::path::PathBuf;

pub struct NodeProjectWatchService {}

const WATCHING_ITEMS_IN_PROJECT: [&str; 1] = ["src"];

impl NodeProjectWatchService {
    pub(crate) fn new() -> Self {
        Self {}
    }

    pub(crate) async fn watch(
        &self,
        node_project_root_path: PathBuf,
        callback: impl 'static + Fn() + Send + Sync,
    ) -> Result<()> {
        let (watcher_sender, mut watcher_receiver) = tokio::sync::mpsc::unbounded_channel();
        let mut watcher = RecommendedWatcher::new(
            move |res| {
                watcher_sender.send(res).unwrap();
            },
            Config::default(),
        )?;
        for watch_path in WATCHING_ITEMS_IN_PROJECT
            .iter()
            .map(|path| node_project_root_path.join(path))
        {
            watcher.watch(&watch_path, notify::RecursiveMode::Recursive)?;
        }

        loop {
            let event = watcher_receiver.recv().await.unwrap().unwrap();
            debug_println!("watch event");
            match event.kind {
                notify::EventKind::Create(_)
                | notify::EventKind::Modify(_)
                | notify::EventKind::Remove(_) => {
                    'flush: loop {
                        match watcher_receiver.try_recv() {
                            Ok(_) => (),
                            Err(error) => match error {
                                tokio::sync::mpsc::error::TryRecvError::Empty => break 'flush,
                                tokio::sync::mpsc::error::TryRecvError::Disconnected => {
                                    panic!("watcher closed {:?}", error)
                                }
                            },
                        }
                    }
                    callback();
                }
                _ => {}
            };
        }
    }
}
