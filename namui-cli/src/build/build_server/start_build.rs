use futures::executor::block_on;
use namui::build::types::ErrorMessage;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{fs::File, io::Read, sync::Arc, thread, time::Duration};

use crate::build::{bundle::Bundle, web_server::WebServer};

use super::{
    run_cargo_build::run_cargo_build,
    run_wasm_bindgen::{run_wasm_bindgen, RunWasmBindgenOption},
};

pub struct RebuildCallbackOption {
    pub error_messages: Vec<ErrorMessage>,
    pub result_path: Option<String>,
}

pub type RebuildCallback = fn(option: RebuildCallbackOption) -> ();

pub struct StartBuildOption {
    pub callback: RebuildCallback,
    pub watch_dir: String,
    pub bundle: Arc<Bundle>,
    pub web_server: Arc<WebServer>,
    pub manifest_path: String,
}

pub async fn start_build<'a>(option: StartBuildOption) {
    let (thread_sender, mut thread_receiver) = tokio::sync::mpsc::channel::<DebouncedEvent>(32);

    thread::spawn(move || {
        let (watcher_sender, watcher_receiver) = std::sync::mpsc::channel::<DebouncedEvent>();

        let mut watcher = watcher(watcher_sender, Duration::from_secs(1)).unwrap();
        watcher
            .watch(option.watch_dir, RecursiveMode::Recursive)
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
                    option
                        .bundle
                        .clone(),
                    option
                        .web_server
                        .clone(),
                    option
                        .manifest_path
                        .clone(),
                )
                .await;
                should_rebuild = false;
            }
            false => 'await_file_change_event: loop {
                match thread_receiver
                    .recv()
                    .await
                {
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
    bundle: Arc<Bundle>,
    web_server: Arc<WebServer>,
    manifest_path: String,
) {
    let build_result = run_cargo_build(manifest_path);

    if build_result.is_successful {
        if let Some(result_path) = &build_result.result_path {
            match run_wasm_bindgen(RunWasmBindgenOption {
                wasm_path: result_path.clone(),
            }) {
                Ok(result) => {
                    let mut buffer: Vec<u8> = Vec::new();

                    match File::open(result.result_js_path) {
                        Ok(mut js_file) => match js_file.read_to_end(&mut buffer) {
                            Ok(_) => {
                                let mut js_bundle = bundle
                                    .js
                                    .write()
                                    .await;
                                *js_bundle = buffer.clone();
                            }
                            Err(error) => {
                                eprintln!("failed to read js. try changing the source file to rebuild.\n  {:?}", error);
                            }
                        },
                        Err(error) => {
                            eprintln!("failed to open js. try changing the source file to rebuild.\n  {:?}", error);
                        }
                    }

                    buffer.clear();
                    match File::open(result.result_wasm_path) {
                        Ok(mut wasm_file) => match wasm_file.read_to_end(&mut buffer) {
                            Ok(_) => {
                                let mut wasm_bundle = bundle
                                    .wasm
                                    .write()
                                    .await;
                                *wasm_bundle = buffer.clone();
                            }
                            Err(error) => {
                                eprintln!("failed to read wasm. try changing the source file to rebuild.\n  {:?}", error);
                            }
                        },
                        Err(error) => {
                            eprintln!("failed to open wasm. try changing the source file to rebuild.\n  {:?}", error);
                        }
                    }
                }
                Err(error) => {
                    eprintln!("{:?}", error);
                }
            }
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
