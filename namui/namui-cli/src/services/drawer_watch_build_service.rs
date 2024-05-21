use super::{
    build_status_service::BuildStatusService,
    rust_build_service::{self, BuildOption},
    rust_project_watch_service::RustProjectWatchService,
    wasm_bundle_web_server::WasmBundleWebServer,
};
use crate::{cli::Target, services::build_status_service::BuildStatusCategory};
use crate::{util::get_cli_root_path, *};
use std::{path::PathBuf, sync::Arc};

pub struct DrawerWatchBuildService {}

type AfterBuild = Option<Arc<dyn Fn() + Send + Sync + 'static>>;

pub struct WatchArgs {
    pub target: Target,
    pub after_build: AfterBuild,
    pub wasm_bundle_web_server: Arc<WasmBundleWebServer>,
    pub build_status_service: Arc<BuildStatusService>,
    pub release: bool,
}
impl DrawerWatchBuildService {
    pub async fn spawn_watch(
        WatchArgs {
            target,
            after_build,
            wasm_bundle_web_server,
            build_status_service,
            release,
        }: WatchArgs,
    ) -> Result<()> {
        wasm_bundle_web_server.add_static_dir("drawer/", build_dist_path());

        let start_build = move || {
            let build_status_service = build_status_service.clone();
            let wasm_bundle_web_server = wasm_bundle_web_server.clone();
            let after_build = after_build.clone();

            async move {
                build_status_service
                    .build_started(BuildStatusCategory::Drawer)
                    .await;
                wasm_bundle_web_server.send_build_start_signal().await;

                let output = rust_build_service::build(BuildOption {
                    target,
                    dist_path: build_dist_path(),
                    project_root_path: project_root_path(),
                    watch: true,
                    release,
                })
                .await??;

                build_status_service
                    .build_finished(BuildStatusCategory::Drawer, output.error_messages, vec![])
                    .await;
                let error_messages = build_status_service.compile_error_messages().await;
                let no_error = error_messages.is_empty();
                wasm_bundle_web_server
                    .send_error_messages(error_messages)
                    .await;
                if no_error {
                    wasm_bundle_web_server.send_reload_signal().await;
                };

                if let Some(f) = after_build.as_ref() {
                    f()
                }

                anyhow::Ok(())
            }
        };

        let mut rust_project_watch_service =
            RustProjectWatchService::new(project_root_path().join("Cargo.toml"))?;

        let mut handle = tokio::spawn(start_build.clone()());

        while (rust_project_watch_service.next().await?).is_some() {
            handle.abort();
            handle = tokio::spawn(start_build.clone()());
        }

        Ok(())
    }

    pub async fn just_build(
        target: Target,
        build_status_service: Arc<BuildStatusService>,
        release: bool,
    ) -> Result<()> {
        build_status_service
            .build_started(BuildStatusCategory::Drawer)
            .await;

        let output = rust_build_service::build(BuildOption {
            target,
            dist_path: build_dist_path(),
            project_root_path: project_root_path(),
            watch: false,
            release,
        })
        .await??;

        build_status_service
            .build_finished(BuildStatusCategory::Drawer, output.error_messages, vec![])
            .await;

        Ok(())
    }
}

pub fn project_root_path() -> PathBuf {
    get_cli_root_path().join("../namui-drawer/wasm-runner")
}
pub fn build_dist_path() -> PathBuf {
    project_root_path().join("pkg/drawer")
}
