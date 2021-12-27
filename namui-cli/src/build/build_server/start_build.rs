use futures::executor::block_on;
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use std::{fs::File, io::Read, sync::Arc, thread, time::Duration};
use tokio::sync::RwLock;

use crate::build::{bundle::Bundle, types::ErrorMessage, web_server::WebServer};

use super::{
    run_cargo_check::run_cargo_check,
    run_wasm_pack::{run_wasm_pack, RunWasmPackOption},
};

pub struct RebuildCallbackOption {
    pub cli_error_messages: Vec<String>,
    pub error_messages: Vec<ErrorMessage>,
}

pub type RebuildCallback = fn(option: RebuildCallbackOption) -> ();

pub struct StartBuildOption {
    pub callback: RebuildCallback,
    pub watch_dir: String,
    pub bundle: Arc<RwLock<Bundle>>,
    pub web_server: Arc<WebServer>,
    pub manifest_path: String,
    pub root_dir: String,
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
                    option.bundle.clone(),
                    option.web_server.clone(),
                    option.manifest_path.clone(),
                    option.root_dir.clone(),
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
    bundle: Arc<RwLock<Bundle>>,
    web_server: Arc<WebServer>,
    manifest_path: String,
    root_dir: String,
) {
    let mut bundle = bundle.write().await;
    let build_result = run_cargo_check(manifest_path);
    let mut cli_error_messages: Vec<String> = Vec::new();

    if build_result.is_successful {
        match run_wasm_pack(RunWasmPackOption { root_dir }) {
            Ok(result) => {
                let mut js_buffer: Vec<u8> = Vec::new();
                let is_js_successful = match File::open(result.result_js_path) {
                    Ok(mut js_file) => match js_file.read_to_end(&mut js_buffer) {
                        Ok(_) => true,
                        Err(error) => {
                            let cli_error_message = format!("failed to read js. try changing the source file to rebuild.\n  {:?}", error);
                            eprintln!("{}", cli_error_message);
                            cli_error_messages.push(cli_error_message);
                            false
                        }
                    },
                    Err(error) => {
                        let cli_error_message = format!(
                            "failed to open js. try changing the source file to rebuild.\n  {:?}",
                            error
                        );
                        eprintln!("{}", cli_error_message);
                        cli_error_messages.push(cli_error_message);
                        false
                    }
                };

                let mut wasm_buffer: Vec<u8> = Vec::new();
                let is_wasm_successful = match File::open(result.result_wasm_path) {
                    Ok(mut wasm_file) => match wasm_file.read_to_end(&mut wasm_buffer) {
                        Ok(_) => true,
                        Err(error) => {
                            let cli_error_message = format!("failed to read wasm. try changing the source file to rebuild.\n  {:?}", error);
                            eprintln!("{}", cli_error_message);
                            cli_error_messages.push(cli_error_message);
                            false
                        }
                    },
                    Err(error) => {
                        let cli_error_message = format!(
                            "failed to open wasm. try changing the source file to rebuild.\n  {:?}",
                            error
                        );
                        eprintln!("{}", cli_error_message);
                        cli_error_messages.push(cli_error_message);
                        false
                    }
                };

                let should_reload = is_js_successful && is_wasm_successful;
                if should_reload {
                    *bundle = Bundle {
                        js: js_buffer,
                        wasm: wasm_buffer,
                    };
                    web_server.request_reload().await;
                }
            }
            Err(error) => {
                let cli_error_message = format!("{:?}", error);
                eprintln!("{}", cli_error_message);
                cli_error_messages.push(cli_error_message);
            }
        }
    }

    web_server
        .send_error_messages(&build_result.error_messages)
        .await;

    callback(RebuildCallbackOption {
        cli_error_messages,
        error_messages: build_result.error_messages,
    })
}
