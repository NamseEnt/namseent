use crate::{
    cli::Target,
    services::wasm_watch_build_service::{WasmWatchBuildService, WatchAndBuildArgs},
};
use std::path::Path;

pub fn start(manifest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    const PORT: u16 = 8080;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();

    let wasm_bundle_web_server_url = format!("http://localhost:{}", PORT);
    println!("server is running on {}", wasm_bundle_web_server_url);

    WasmWatchBuildService::watch_and_build(WatchAndBuildArgs {
        project_root_path,
        port: PORT,
        target: Target::WasmUnknownWeb,
        after_first_build: Some(move || {
            let _ = webbrowser::open(&wasm_bundle_web_server_url);
        }),
    })
}
