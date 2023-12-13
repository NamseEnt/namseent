use super::{
    build_status_service::BuildStatusService,
    rust_build_service::{BuildOption, BuildResult, RustBuildService},
    rust_project_watch_service::RustProjectWatchService,
    wasm_bundle_web_server::WasmBundleWebServer,
};
use crate::{cli::Target, services::build_status_service::BuildStatusCategory};
use crate::{util::get_cli_root_path, *};
use std::{path::PathBuf, sync::Arc};
use tokio::try_join;

pub struct DrawerWatchBuildService {}

type AfterBuild = Option<Arc<dyn Fn() + Send + Sync + 'static>>;

pub struct WatchArgs {
    pub target: Target,
    pub after_build: AfterBuild,
    pub wasm_bundle_web_server: Arc<WasmBundleWebServer>,
    pub build_status_service: Arc<BuildStatusService>,
}
impl DrawerWatchBuildService {
    pub async fn spawn_watch(args: WatchArgs) -> Result<()> {
        args.wasm_bundle_web_server
            .add_static_dir("drawer/", build_dist_path());

        let rust_project_watch_service = Arc::new(RustProjectWatchService::new());
        let rust_build_service = Arc::new(RustBuildService::new());
        let build_status_service = args.build_status_service;

        try_join!(
            cancel_and_start_build(
                args.wasm_bundle_web_server.clone(),
                rust_build_service.clone(),
                build_status_service.clone(),
                build_dist_path(),
                project_root_path(),
                args.target,
                args.after_build.clone(),
            ),
            rust_project_watch_service.watch(project_root_path().join("Cargo.toml"), {
                let rust_build_service = rust_build_service.clone();
                let after_build = args.after_build.clone();
                move || {
                    tokio::spawn(cancel_and_start_build(
                        args.wasm_bundle_web_server.clone(),
                        rust_build_service.clone(),
                        build_status_service.clone(),
                        build_dist_path(),
                        project_root_path(),
                        args.target,
                        after_build.clone(),
                    ));
                }
            }),
        )?;

        return Ok(());

        pub async fn cancel_and_start_build(
            wasm_bundle_web_server: Arc<WasmBundleWebServer>,
            rust_build_service: Arc<RustBuildService>,
            build_status_service: Arc<BuildStatusService>,
            build_dist_path: PathBuf,
            project_root_path: PathBuf,
            target: Target,
            after_build: AfterBuild,
        ) -> Result<()> {
            debug_println!("build fn run");
            build_status_service
                .build_started(BuildStatusCategory::Drawer)
                .await;
            wasm_bundle_web_server.send_build_start_signal().await;
            match rust_build_service
                .cancel_and_start_build(&BuildOption {
                    target,
                    dist_path: build_dist_path,
                    project_root_path: project_root_path.clone(),
                    watch: true,
                })
                .await
            {
                BuildResult::Canceled => {
                    debug_println!("build canceled");
                }
                BuildResult::Successful(cargo_build_result) => {
                    if let Some(f) = after_build.as_ref() {
                        f()
                    }

                    build_status_service
                        .build_finished(
                            BuildStatusCategory::Drawer,
                            cargo_build_result.error_messages,
                            vec![],
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
                }
                BuildResult::Failed(err) => {
                    eprintln!("failed to build: {}", err);
                    bail!("failed to build: {}", err);
                }
            }

            Ok(())
        }
    }

    pub async fn just_build(
        target: Target,
        build_status_service: Arc<BuildStatusService>,
    ) -> Result<()> {
        build_status_service
            .build_started(BuildStatusCategory::Drawer)
            .await;

        let rust_build_service = Arc::new(RustBuildService::new());

        match rust_build_service
            .cancel_and_start_build(&BuildOption {
                target,
                dist_path: build_dist_path(),
                project_root_path: project_root_path(),
                watch: false,
            })
            .await
        {
            BuildResult::Successful(cargo_build_result) => {
                build_status_service
                    .build_finished(
                        BuildStatusCategory::Drawer,
                        cargo_build_result.error_messages,
                        vec![],
                    )
                    .await;
                Ok(())
            }
            BuildResult::Canceled => unreachable!(),
            BuildResult::Failed(error) => Err(anyhow!("{}", error)),
        }
    }
}

pub fn project_root_path() -> PathBuf {
    get_cli_root_path().join("../namui-drawer/wasm-runner")
}
pub fn build_dist_path() -> PathBuf {
    project_root_path().join("pkg/drawer")
}
