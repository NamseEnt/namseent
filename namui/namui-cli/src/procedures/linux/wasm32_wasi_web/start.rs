use crate::services::build_status_service::BuildStatusService;
use crate::services::wasm_bundle_web_server::WasmBundleWebServer;
use crate::services::wasm_web_runtime_watch_build_service::{
    self, WasmWebRuntimeWatchBuildService,
};
use crate::*;
use crate::{
    cli::Target,
    services::wasm_watch_build_service::{WasmWatchBuildService, WatchAndBuildArgs},
};
use std::path::Path;
use tokio::try_join;

pub async fn start(manifest_path: &Path, release: bool) -> Result<()> {
    const PORT: u16 = 8080;
    let target = Target::Wasm32WasiWeb;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let build_status_service = BuildStatusService::new();

    let wasm_bundle_web_server_url = format!("http://localhost:{}", PORT);
    println!("server is running on {}", wasm_bundle_web_server_url);

    let wasm_bundle_web_server = WasmBundleWebServer::start(PORT);
    let web_runtime_watch = WasmWebRuntimeWatchBuildService::watch_and_build(
        wasm_web_runtime_watch_build_service::WatchAndBuildArgs {
            wasm_bundle_web_server: wasm_bundle_web_server.clone(),
            after_first_build: None,
            build_status_service: build_status_service.clone(),
        },
    );

    let main_watch = WasmWatchBuildService::watch_and_build(WatchAndBuildArgs {
        project_root_path,
        bundle_web_server: services::wasm_watch_build_service::BundleWebServerArgs::WebServer {
            web_server: wasm_bundle_web_server.clone(),
        },
        build_status_service: build_status_service.clone(),
        target,
        after_first_build: Some(move || {
            let _ = webbrowser::open(&wasm_bundle_web_server_url);
        }),
        release,
    });

    try_join!(web_runtime_watch, main_watch)?;
    Ok(())
}
