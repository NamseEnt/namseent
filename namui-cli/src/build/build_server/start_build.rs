use crate::{
    build::{bundle::Bundle, types::ErrorMessage, web_server::WebServer},
    debug_println,
};
use std::{fs::File, io::Read, sync::Arc};
use tokio::sync::RwLock;

use super::{
    code_watcher::CodeWatcher,
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
    pub bundle: Arc<RwLock<Bundle>>,
    pub web_server: Arc<WebServer>,
    pub manifest_path: String,
    pub root_dir: String,
}

pub async fn start_build<'a>(option: StartBuildOption) {
    let mut watcher = CodeWatcher::new(option.manifest_path.clone());

    loop {
        watcher.update_watching_paths();
        watcher.wait_for_change().await;
        println!("File changed. starting rebuild...");
        rebuild(
            option.callback,
            option.bundle.clone(),
            option.web_server.clone(),
            option.manifest_path.clone(),
            option.root_dir.clone(),
        )
        .await;
    }
}

async fn rebuild(
    callback: RebuildCallback,
    bundle: Arc<RwLock<Bundle>>,
    web_server: Arc<WebServer>,
    manifest_path: String,
    root_dir: String,
) {
    debug_println!("rebuild: locking web_server.bundle...");
    let mut bundle = bundle.write().await;
    debug_println!("rebuild: web_server.bundle locked");
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
