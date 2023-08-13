use super::{
    runtime_project::{self, GenerateRuntimeProjectArgs},
    rust_build_service::{BuildOption, BuildResult, RustBuildService},
    rust_project_watch_service::RustProjectWatchService,
    wasm_bundle_web_server::WasmBundleWebServer,
};
use crate::*;
use crate::{cli::Target, debug_println, util::print_build_result};
use std::{path::PathBuf, sync::Arc};
use tokio::try_join;

pub struct WasmWatchBuildService {}

pub enum BundleWebServerArgs {
    Port {
        port: u16,
    },
    WebServer {
        web_server: Arc<WasmBundleWebServer>,
    },
}

pub struct WatchAndBuildArgs<AfterFirstBuild>
where
    AfterFirstBuild: FnOnce() + Send + 'static,
{
    pub project_root_path: PathBuf,
    pub bundle_web_server: BundleWebServerArgs,
    pub target: Target,
    pub after_first_build: Option<AfterFirstBuild>,
}
impl WasmWatchBuildService {
    pub async fn watch_and_build<AfterFirstBuild>(
        args: WatchAndBuildArgs<AfterFirstBuild>,
    ) -> Result<()>
    where
        AfterFirstBuild: FnOnce() + Send + 'static,
    {
        let project_root_path = args.project_root_path;
        let build_dist_path = project_root_path.join("pkg");
        let runtime_target_dir = project_root_path.join("target/namui");

        runtime_project::wasm::generate_runtime_project(GenerateRuntimeProjectArgs {
            target_dir: runtime_target_dir.clone(),
            project_path: project_root_path.clone(),
        })?;

        let rust_project_watch_service = Arc::new(RustProjectWatchService::new());
        let wasm_bundle_web_server = {
            match args.bundle_web_server {
                BundleWebServerArgs::Port { port } => WasmBundleWebServer::start(port),
                BundleWebServerArgs::WebServer { web_server } => web_server,
            }
        };
        wasm_bundle_web_server.add_static_dir("", &build_dist_path);
        let rust_build_service = Arc::new(RustBuildService::new());

        pub async fn cancel_and_start_build(
            wasm_bundle_web_server: Arc<WasmBundleWebServer>,
            rust_build_service: Arc<RustBuildService>,
            build_dist_path: PathBuf,
            project_root_path: PathBuf,
            runtime_target_dir: PathBuf,
            target: Target,
        ) {
            debug_println!("build fn run");
            match rust_build_service.cancel_and_start_build(&BuildOption {
                target,
                dist_path: build_dist_path,
                project_root_path: runtime_target_dir,
                watch: true,
            }) {
                BuildResult::Canceled => {
                    debug_println!("build canceled");
                }
                BuildResult::Successful(cargo_build_result) => {
                    let mut cli_error_messages = Vec::new();
                    let bundle_manifest = crate::services::bundle::NamuiBundleManifest::parse(
                        project_root_path.clone(),
                    )
                    .map_err(|error| error.to_string());

                    if let Err(error) = bundle_manifest.as_ref() {
                        cli_error_messages.push(format!("fail to get bundle_manifest: {}", error));
                    }
                    print_build_result(&cargo_build_result.error_messages, &cli_error_messages);

                    wasm_bundle_web_server
                        .on_build_done(&cargo_build_result, bundle_manifest.ok())
                        .await;
                }
                BuildResult::Failed(err) => {
                    eprintln!("failed to build: {}", err);
                    std::process::exit(1);
                }
            }
        }

        let first_run = {
            let wasm_bundle_web_server = wasm_bundle_web_server.clone();
            let rust_build_service = rust_build_service.clone();
            let build_dist_path = build_dist_path.clone();
            let runtime_target_dir = runtime_target_dir.clone();
            let project_root_path = project_root_path.clone();
            async move {
                cancel_and_start_build(
                    wasm_bundle_web_server.clone(),
                    rust_build_service.clone(),
                    build_dist_path.clone(),
                    project_root_path.clone(),
                    runtime_target_dir.clone(),
                    args.target,
                )
                .await;
                if let Some(after_first_build) = args.after_first_build {
                    (after_first_build)();
                }
                Ok(())
            }
        };

        let watch = rust_project_watch_service.watch(project_root_path.join("Cargo.toml"), {
            let wasm_bundle_web_server = wasm_bundle_web_server.clone();
            let rust_build_service = rust_build_service.clone();
            let build_dist_path = build_dist_path.clone();
            let runtime_target_dir = runtime_target_dir.clone();
            move || {
                tokio::spawn(cancel_and_start_build(
                    wasm_bundle_web_server.clone(),
                    rust_build_service.clone(),
                    build_dist_path.clone(),
                    project_root_path.clone(),
                    runtime_target_dir.clone(),
                    args.target,
                ));
            }
        });
        try_join!(first_run, watch)?;

        Ok(())
    }

    pub fn just_build(project_root_path: PathBuf, target: Target) -> Result<()> {
        let build_dist_path = project_root_path.join("pkg");
        let runtime_target_dir = project_root_path.join("target/namui");
        let rust_build_service = RustBuildService::new();

        runtime_project::wasm::generate_runtime_project(GenerateRuntimeProjectArgs {
            target_dir: runtime_target_dir.clone(),
            project_path: project_root_path.clone(),
        })?;

        match rust_build_service.cancel_and_start_build(&BuildOption {
            target,
            dist_path: build_dist_path,
            project_root_path: runtime_target_dir,
            watch: false,
        }) {
            BuildResult::Successful(cargo_build_result) => {
                print_build_result(&cargo_build_result.error_messages, &vec![]);
                Ok(())
            }
            BuildResult::Canceled => unreachable!(),
            BuildResult::Failed(error) => Err(anyhow!("{}", error)),
        }
    }
}
