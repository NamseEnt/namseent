use futures::executor::block_on;
use namui::build::types::ErrorMessage;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{
    fs::File,
    io::Read,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

use crate::build::web_server::WebServer;

use super::run_cargo_build::run_cargo_build;

pub struct RebuildCallbackOption {
    pub error_messages: Vec<ErrorMessage>,
    pub result_path: Option<String>,
}

pub type RebuildCallback = fn(option: RebuildCallbackOption) -> ();

pub struct StartBuildOption {
    pub callback: RebuildCallback,
    pub root_dir: String,
    pub bundle: Arc<RwLock<Vec<u8>>>,
    pub web_server: Arc<WebServer>,
}

pub async fn start_build<'a>(option: StartBuildOption) {
    let (thread_sender, mut thread_receiver) = tokio::sync::mpsc::channel::<DebouncedEvent>(32);

    thread::spawn(move || {
        let (watcher_sender, watcher_receiver) = std::sync::mpsc::channel::<DebouncedEvent>();

        let mut watcher = watcher(watcher_sender, Duration::from_secs(1)).unwrap();
        watcher
            .watch(option.root_dir, RecursiveMode::Recursive)
            .unwrap();

        loop {
            match watcher_receiver.recv() {
                Ok(event) => {
                    let _ = block_on(thread_sender.send(event));
                }
                Err(error) => eprintln!("{:?}", error),
            }
        }
    });

    let mut should_rebuild = true;
    loop {
        match should_rebuild {
            true => {
                rebuild(
                    option.callback,
                    option.bundle.clone(),
                    option.web_server.clone(),
                )
                .await;
                should_rebuild = false;
            }
            false => 'await_file_change_event: loop {
                match thread_receiver.recv().await {
                    Some(event) => match event {
                        DebouncedEvent::Create(_)
                        | DebouncedEvent::Remove(_)
                        | DebouncedEvent::Rename(_, _)
                        | DebouncedEvent::Write(_) => {
                            should_rebuild = true;
                            break 'await_file_change_event;
                        }
                        _ => (),
                    },
                    _ => (),
                };
            },
        }

        'clear_file_change_events: loop {
            match thread_receiver.try_recv() {
                Ok(event) => match event {
                    DebouncedEvent::Create(_)
                    | DebouncedEvent::Remove(_)
                    | DebouncedEvent::Rename(_, _)
                    | DebouncedEvent::Write(_) => should_rebuild = true,
                    _ => (),
                },
                Err(_) => break 'clear_file_change_events,
            }
        }
    }
}

async fn rebuild(
    callback: RebuildCallback,
    bundle: Arc<RwLock<Vec<u8>>>,
    web_server: Arc<WebServer>,
) {
    let build_result = run_cargo_build();

    if let Some(result_path) = &build_result.result_path {
        if let Ok(mut file) = File::open(result_path) {
            let mut buffer: Vec<u8> = Vec::new();
            if let Ok(_) = file.read_to_end(&mut buffer) {
                match bundle.write() {
                    Ok(mut bundle) => *bundle = buffer,
                    Err(error) => eprintln!(
                        "failed to update bundle.wasm. try changing the source file to rebuild.\n  {:?}", error
                    ),
                }
            }
        } else {
            eprintln!("failed to open bundle file");
        }
    }

    web_server
        .send_error_messages(&build_result.error_messages)
        .await;

    callback(RebuildCallbackOption {
        error_messages: build_result.error_messages,
        result_path: build_result.result_path,
    })
}
