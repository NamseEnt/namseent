use crate::services::build_status_service::BuildStatusService;
use crate::services::wasm_bundle_web_server::WasmBundleWebServer;
use crate::services::wasm_web_runtime_watch_build_service::WasmWebRuntimeWatchBuildService;
use crate::services::{drawer_watch_build_service, wasm_web_runtime_watch_build_service};
use crate::*;
use crate::{
    cli::Target,
    services::{
        drawer_watch_build_service::DrawerWatchBuildService,
        wasm_watch_build_service::{WasmWatchBuildService, WatchAndBuildArgs},
    },
};
use std::path::Path;
use tokio::try_join;

pub async fn start(manifest_path: &Path) -> Result<()> {
    const PORT: u16 = 8080;
    let target = Target::WasmUnknownWeb;
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

    let drawer = DrawerWatchBuildService::spawn_watch(drawer_watch_build_service::WatchArgs {
        target,
        wasm_bundle_web_server: wasm_bundle_web_server.clone(),
        build_status_service: build_status_service.clone(),
        after_build: None,
    });

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
    });

    try_join!(web_runtime_watch, drawer, main_watch)?;
    Ok(())
}
