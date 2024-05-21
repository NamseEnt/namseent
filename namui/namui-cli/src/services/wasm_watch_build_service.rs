use super::{
    runtime_project::{self, GenerateRuntimeProjectArgs},
    rust_build_service::{self, BuildOption},
    rust_project_watch_service::{self},
    wasm_bundle_web_server::WasmBundleWebServer,
};
use crate::cli::Target;
use crate::{
    services::build_status_service::{BuildStatusCategory, BuildStatusService},
    *,
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

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
    pub build_status_service: Arc<BuildStatusService>,
    pub release: bool,
}
impl WasmWatchBuildService {
    pub async fn watch_and_build<AfterFirstBuild>(
        args: WatchAndBuildArgs<AfterFirstBuild>,
    ) -> Result<()>
    where
        AfterFirstBuild: FnOnce() + Send + 'static,
    {
        let WatchAndBuildArgs {
            project_root_path,
            bundle_web_server,
            target,
            after_first_build,
            build_status_service,
            release,
        } = args;
        let build_dist_path = project_root_path.join("pkg");
        let runtime_target_dir = project_root_path.join("target/namui");

        let after_first_build = Arc::new(Mutex::new(after_first_build));

        runtime_project::wasm::generate_runtime_project(GenerateRuntimeProjectArgs {
            target_dir: runtime_target_dir.clone(),
            project_path: project_root_path.clone(),
        })?;

        let wasm_bundle_web_server = {
            match bundle_web_server {
                BundleWebServerArgs::Port { port } => WasmBundleWebServer::start(port),
                BundleWebServerArgs::WebServer { web_server } => web_server,
            }
        };
        wasm_bundle_web_server.add_static_dir("", build_dist_path.clone());

        let mut rust_project_watch_service =
            rust_project_watch_service::RustProjectWatchService::new(
                project_root_path.join("Cargo.toml"),
            )?;

        let start_build = move || {
            let build_status_service = build_status_service.clone();
            let wasm_bundle_web_server = wasm_bundle_web_server.clone();

            async move {
                build_status_service
                    .build_started(BuildStatusCategory::Namui)
                    .await;
                wasm_bundle_web_server.send_build_start_signal().await;

                let cargo_build_output = rust_build_service::build(BuildOption {
                    target,
                    dist_path: build_dist_path,
                    project_root_path: runtime_target_dir,
                    watch: true,
                    release,
                })
                .await??;

                let mut cli_error_messages = Vec::new();
                let bundle_manifest =
                    crate::services::bundle::NamuiBundleManifest::parse(project_root_path.clone())
                        .map_err(|error| error.to_string());

                if let Err(error) = bundle_manifest.as_ref() {
                    cli_error_messages.push(format!("fail to get bundle_manifest: {}", error));
                }

                wasm_bundle_web_server
                    .update_namui_bundle_manifest(bundle_manifest.ok())
                    .await;
                build_status_service
                    .build_finished(
                        BuildStatusCategory::Namui,
                        cargo_build_output.error_messages,
                        cli_error_messages,
                    )
                    .await;
                let error_messages = build_status_service.compile_error_messages().await;
                let no_error = error_messages.is_empty();
                wasm_bundle_web_server
                    .send_error_messages(error_messages)
                    .await;
                if no_error {
                    wasm_bundle_web_server.send_reload_signal().await;
                };

                if let Some(after_first_build) = after_first_build.lock().await.take() {
                    (after_first_build)();
                }

                anyhow::Ok(())
            }
        };

        let mut handle = tokio::spawn(start_build.clone()());

        while (rust_project_watch_service.next().await?).is_some() {
            handle.abort();
            handle = tokio::spawn(start_build.clone()());
        }

        Ok(())
    }

    pub async fn just_build(
        build_status_service: Arc<BuildStatusService>,
        project_root_path: PathBuf,
        target: Target,
        release: bool,
    ) -> Result<()> {
        build_status_service
            .build_started(BuildStatusCategory::Namui)
            .await;

        let build_dist_path = project_root_path.join("pkg");
        let runtime_target_dir = project_root_path.join("target/namui");

        runtime_project::wasm::generate_runtime_project(GenerateRuntimeProjectArgs {
            target_dir: runtime_target_dir.clone(),
            project_path: project_root_path.clone(),
        })?;

        let output = rust_build_service::build(BuildOption {
            target,
            dist_path: build_dist_path,
            project_root_path: runtime_target_dir,
            watch: false,
            release,
        })
        .await??;

        build_status_service
            .build_finished(BuildStatusCategory::Namui, output.error_messages, vec![])
            .await;
        Ok(())
    }
}
