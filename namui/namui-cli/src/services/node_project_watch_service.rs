use crate::debug_println;
use crate::*;
use notify::{Config, RecommendedWatcher, Watcher};
use std::path::{Path, PathBuf};

pub struct NodeProjectWatchService {
    node_project_root_path: PathBuf,
    watcher: RecommendedWatcher,
    watcher_receiver: tokio::sync::mpsc::UnboundedReceiver<notify::Result<notify::Event>>,
}

const WATCHING_ITEMS_IN_PROJECT: [&str; 1] = ["src"];

impl NodeProjectWatchService {
    pub(crate) fn new(node_project_root_path: impl AsRef<Path>) -> Result<Self> {
        let (watcher_sender, watcher_receiver) = tokio::sync::mpsc::unbounded_channel();
        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = watcher_sender.send(res);
            },
            Config::default(),
        )?;

        Ok(Self {
            node_project_root_path: node_project_root_path.as_ref().to_path_buf(),
            watcher,
            watcher_receiver,
        })
    }

    pub(crate) async fn next(&mut self) -> Option<()> {
        loop {
            let event = self.watcher_receiver.recv().await.unwrap().unwrap();
            debug_println!("watch event");
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
                                    panic!("watcher closed {:?}", error)
                                }
                            },
                        }
                    }
                    return Some(());
                }
                _ => {}
            };
        }
    }
}
