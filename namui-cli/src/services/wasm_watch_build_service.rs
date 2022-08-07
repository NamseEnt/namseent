use super::{
    bundle_metadata_service::BundleMetadataService,
    runtime_project::{self, GenerateRuntimeProjectArgs},
    rust_build_service::{BuildOption, BuildResult, RustBuildService},
    rust_project_watch_service::RustProjectWatchService,
    wasm_bundle_web_server::WasmBundleWebServer,
};
use crate::{cli::Target, debug_println, util::print_build_result};
use std::{error::Error, path::PathBuf, sync::Arc};

pub struct WasmWatchBuildService {}

pub struct StartArgs {
    pub project_root_path: PathBuf,
    pub port: u16,
}
impl WasmWatchBuildService {
    pub fn start(args: StartArgs) -> Result<(), Box<dyn Error>> {
        let build_dist_path = args.project_root_path.join("pkg");

        let runtime_target_dir = args.project_root_path.join("target/namui");

        runtime_project::wasm::generate_runtime_project(GenerateRuntimeProjectArgs {
            target_dir: runtime_target_dir.clone(),
            project_path: args.project_root_path.clone(),
        })?;

        let bundle_metadata_service = Arc::new(BundleMetadataService::new());
        let rust_project_watch_service = Arc::new(RustProjectWatchService::new());
        let wasm_bundle_web_server = WasmBundleWebServer::start(
            args.port,
            &build_dist_path,
            bundle_metadata_service.clone(),
        );
        let rust_build_service = Arc::new(RustBuildService::new());

        tokio::spawn(build(
            wasm_bundle_web_server.clone(),
            rust_build_service.clone(),
            build_dist_path.clone(),
            args.project_root_path.clone(),
            runtime_target_dir.clone(),
            bundle_metadata_service.clone(),
        ));
        rust_project_watch_service.watch(&args.project_root_path.join("Cargo.toml"), {
            let wasm_bundle_web_server = wasm_bundle_web_server.clone();
            let rust_build_service = rust_build_service.clone();
            let build_dist_path = build_dist_path.clone();
            move || {
                tokio::spawn(build(
                    wasm_bundle_web_server.clone(),
                    rust_build_service.clone(),
                    build_dist_path.clone(),
                    args.project_root_path.clone(),
                    runtime_target_dir.clone(),
                    bundle_metadata_service.clone(),
                ));
            }
        })
    }
}

pub async fn build(
    wasm_bundle_web_server: Arc<WasmBundleWebServer>,
    rust_build_service: Arc<RustBuildService>,
    build_dist_path: PathBuf,
    project_root_path: PathBuf,
    runtime_project_root_path: PathBuf,
    bundle_metadata_service: Arc<BundleMetadataService>,
) {
    debug_println!("build fn run");
    match rust_build_service.cancel_and_start_build(&BuildOption {
        target: Target::WasmUnknownWeb,
        dist_path: build_dist_path,
        project_root_path: runtime_project_root_path,
        watch: true,
    }) {
        BuildResult::Canceled => {
            debug_println!("build canceled");
        }
        BuildResult::Successful(cargo_build_result) => {
            let mut cli_error_messages = Vec::new();
            if let Err(error) = bundle_metadata_service.load_bundle_manifest(&project_root_path) {
                cli_error_messages.push(format!(
                    "could not load bundle manifest for bundle metadata service: {}",
                    error
                ));
            }
            print_build_result(&cargo_build_result.error_messages, &cli_error_messages);
            wasm_bundle_web_server
                .on_build_done(&cargo_build_result)
                .await;
        }
        BuildResult::Failed(err) => {
            eprintln!("failed to build: {}", err);
            std::process::exit(1);
        }
    }
}
