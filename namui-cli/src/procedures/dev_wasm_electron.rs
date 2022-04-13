use crate::{
    debug_println,
    services::{
        electron_dev_service::start_electron_dev_service,
        rust_build_service::{BuildOption, BuildPlatform, BuildResult, RustBuildService},
        rust_project_watch_service::RustProjectWatchService,
        wasm_bundle_web_server::WasmBundleWebServer,
    },
    util::print_build_result,
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn dev_wasm_electron(manifest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    const PORT: u16 = 8080;
    let wasm_bundle_web_server_url = format!("http://localhost:{}", PORT);

    let _ = start_electron_dev_service(&PORT).unwrap();
    println!("server is running on {}", wasm_bundle_web_server_url);

    let build_dist_path = manifest_path.parent().unwrap().join("pkg");
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();

    let rust_project_watch_service = Arc::new(RustProjectWatchService::new());
    let wasm_bundle_web_server = WasmBundleWebServer::start(PORT, &build_dist_path);
    let rust_build_service = Arc::new(RustBuildService::new());

    tokio::spawn(build(
        wasm_bundle_web_server.clone(),
        rust_build_service.clone(),
        build_dist_path.clone(),
        project_root_path.clone(),
    ));
    rust_project_watch_service.watch(manifest_path, {
        let wasm_bundle_web_server = wasm_bundle_web_server.clone();
        let rust_build_service = rust_build_service.clone();
        let build_dist_path = build_dist_path.clone();
        let project_root_path = project_root_path.clone();
        move || {
            tokio::spawn(build(
                wasm_bundle_web_server.clone(),
                rust_build_service.clone(),
                build_dist_path.clone(),
                project_root_path.clone(),
            ));
        }
    })?;

    Ok(())
}

async fn build(
    wasm_bundle_web_server: Arc<WasmBundleWebServer>,
    rust_build_service: Arc<RustBuildService>,
    build_dist_path: PathBuf,
    project_root_path: PathBuf,
) {
    debug_println!("build fn run");
    match rust_build_service.cancel_and_start_build(&BuildOption {
        platform: BuildPlatform::DevWasmElectron,
        dist_path: build_dist_path.to_path_buf(),
        project_root_path: project_root_path.to_path_buf(),
    }) {
        BuildResult::Canceled => {
            debug_println!("build canceled");
        }
        BuildResult::Successful(cargo_build_result) => {
            print_build_result(&cargo_build_result.error_messages, &vec![]);
            wasm_bundle_web_server
                .on_build_done(&cargo_build_result)
                .await;
        }
        BuildResult::Failed(err) => {
            eprintln!("failed to build: {}", err);
        }
    }
}
