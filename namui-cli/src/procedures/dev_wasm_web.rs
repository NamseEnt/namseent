use crate::services::{
    rust_build_service::{BuildOption, BuildPlatform, BuildResult, RustBuildService},
    rust_project_watch_service::RustProjectWatchService,
    wasm_bundle_web_server::WasmBundleWebServer,
};
use std::{path::Path, sync::Arc, thread};

pub fn dev_wasm_web(manifest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let rust_project_watch_service = Arc::new(RustProjectWatchService::new());
    let wasm_bundle_web_server = Arc::new(WasmBundleWebServer::new());
    let rust_build_service = Arc::new(RustBuildService::new());
    let dist_path = manifest_path.parent().unwrap().join("pkg");
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();

    let build_fn = move || {
        println!("start building");
        match rust_build_service.cancel_and_start_build(&BuildOption {
            platform: BuildPlatform::WasmWeb,
            dist_path: dist_path.clone(),
            project_root_path: project_root_path.clone(),
        }) {
            BuildResult::Canceled => {}
            BuildResult::Successful(cargo_build_result) => {
                println!("build successfully done");
            }
            BuildResult::Failed(err) => {
                eprintln!("failed to build: {}", err);
            }
        }
    };

    thread::spawn(build_fn.clone());
    rust_project_watch_service.watch(manifest_path, build_fn)?;

    Ok(())
}
