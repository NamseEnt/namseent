use futures::{executor::block_on, lock::Mutex};
use notify::{DebouncedEvent, INotifyWatcher, RecursiveMode, Watcher};
use std::{sync::Arc, thread, time::Duration};
use tokio::sync::oneshot::{self, Sender};

use crate::debug_println;

struct CodeWatcherContext {
    was_changed: bool,
    sender: Option<Sender<()>>,
}

pub struct CodeWatcher {
    // TODO: it will needed for updating paths
    #[allow(dead_code)]
    watcher: INotifyWatcher,
    context: Arc<Mutex<CodeWatcherContext>>,
}

impl CodeWatcher {
    pub fn new(path: String) -> Self {
        let context = Arc::new(Mutex::new(CodeWatcherContext {
            was_changed: true,
            sender: None,
        }));

        let (watcher_sender, watcher_receiver) = std::sync::mpsc::channel::<DebouncedEvent>();
        let mut watcher = notify::watcher(watcher_sender, Duration::from_secs(1)).unwrap();
        watcher.watch(path, RecursiveMode::Recursive).unwrap();

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

        Self { watcher, context }
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
}
