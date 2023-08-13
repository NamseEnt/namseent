use crate::services::wasm_bundle_web_server::WasmBundleWebServer;
use crate::services::{drawer_build_service, wasm_web_runtime_prepare_service};
use crate::*;
use crate::{
    cli::Target,
    services::{
        drawer_build_service::DrawerBuildService,
        wasm_watch_build_service::{WasmWatchBuildService, WatchAndBuildArgs},
    },
};
use std::path::Path;
use tokio::try_join;

pub async fn start(manifest_path: &Path) -> Result<()> {
    const PORT: u16 = 8080;
    let target = Target::WasmUnknownWeb;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();

    let wasm_bundle_web_server_url = format!("http://localhost:{}", PORT);
    println!("server is running on {}", wasm_bundle_web_server_url);

    wasm_web_runtime_prepare_service::watch_browser_runtime()?;
    let wasm_bundle_web_server = WasmBundleWebServer::start(PORT);

    let drawer = DrawerBuildService::spawn_watch(drawer_build_service::WatchArgs {
        target,
        wasm_bundle_web_server: wasm_bundle_web_server.clone(),
        after_build: None,
    });

    let main_watch = WasmWatchBuildService::watch_and_build(WatchAndBuildArgs {
        project_root_path,
        bundle_web_server: services::wasm_watch_build_service::BundleWebServerArgs::WebServer {
            web_server: wasm_bundle_web_server.clone(),
        },
        target,
        after_first_build: Some(move || {
            let _ = webbrowser::open(&wasm_bundle_web_server_url);
        }),
    });

    try_join!(drawer, main_watch)?;
    Ok(())
}
