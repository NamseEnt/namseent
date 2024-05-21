use crate::*;
use notify::{Config, RecommendedWatcher, Watcher};
use std::path::Path;

pub struct NodeProjectWatchService {
    watcher: RecommendedWatcher,
    watcher_receiver: tokio::sync::mpsc::UnboundedReceiver<notify::Event>,
}

impl NodeProjectWatchService {
    pub(crate) fn new(node_project_root_path: impl AsRef<Path>) -> Result<Self> {
        let (watcher_sender, watcher_receiver) = tokio::sync::mpsc::unbounded_channel();
        let mut watcher = RecommendedWatcher::new(
            move |res: notify::Result<notify::Event>| {
                let _ = watcher_sender.send(res.unwrap());
            },
            Config::default(),
        )?;

        watcher.watch(
            node_project_root_path.as_ref().join("src").as_ref(),
            notify::RecursiveMode::Recursive,
        )?;

        Ok(Self {
            watcher,
            watcher_receiver,
        })
    }

    pub(crate) async fn next(&mut self) -> Option<()> {
        while let Some(event) = self.watcher_receiver.recv().await {
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

        None
    }
}
