use super::build_status_service::BuildStatusService;
use super::rollup_build_service::BuildResult;
use super::wasm_bundle_web_server::WasmBundleWebServer;
use crate::services::build_status_service::BuildStatusCategory;
use crate::services::node_project_watch_service::NodeProjectWatchService;
use crate::services::rollup_build_service;
use crate::services::rollup_build_service::{BuildOption, RollupBuildService};
use crate::util::get_cli_root_path;
use crate::*;
use futures::executor::block_on;
use futures::try_join;
use std::path::PathBuf;
use std::sync::Arc;

type AfterBuild = Option<Arc<dyn Fn() + Send + Sync + 'static>>;
pub struct WasmWebRuntimeWatchBuildService {}
pub struct WatchAndBuildArgs {
    pub wasm_bundle_web_server: Arc<WasmBundleWebServer>,
    pub after_first_build: AfterBuild,
    pub build_status_service: Arc<BuildStatusService>,
}

impl WasmWebRuntimeWatchBuildService {
    pub async fn watch_and_build(args: WatchAndBuildArgs) -> Result<()> {
        let WatchAndBuildArgs {
            wasm_bundle_web_server,
            after_first_build,
            build_status_service,
        } = args;
        let project_root_path = get_cli_root_path().join("webCode");

        let node_project_watch_service = Arc::new(NodeProjectWatchService::new());
        let rollup_build_service = Arc::new(RollupBuildService::new());

        pub async fn cancel_and_start_build(
            wasm_bundle_web_server: Arc<WasmBundleWebServer>,
            rollup_build_service: Arc<RollupBuildService>,
            build_status_service: Arc<BuildStatusService>,
            rollup_project_root_path: PathBuf,
        ) {
            debug_println!("build fn run");
            build_status_service
                .build_started(BuildStatusCategory::WebRuntime)
                .await;

            let cli_error_messages = install_deps()
                .err()
                .map_or(vec![], |error| vec![error.to_string()]);
            match rollup_build_service.cancel_and_start_build(&BuildOption {
                rollup_project_root_path,
                development: true,
            }) {
                rollup_build_service::BuildResult::Canceled => {
                    debug_println!("build canceled");
                }
                rollup_build_service::BuildResult::Successful(rollup_build_result) => {
                    build_status_service
                        .build_finished(
                            BuildStatusCategory::WebRuntime,
                            rollup_build_result.error_messages,
                            cli_error_messages,
                        )
                        .await;
                    let error_messages = build_status_service.compile_error_messages().await;
                    let no_error = error_messages.len() == 0;
                    wasm_bundle_web_server
                        .send_error_messages(error_messages)
                        .await;
                    if no_error {
                        wasm_bundle_web_server.send_reload_signal().await;
                    };
                }
                rollup_build_service::BuildResult::Failed(err) => {
                    eprintln!("failed to build: {}", err);
                    std::process::exit(1);
                }
            }
        }

        let first_run = {
            let wasm_bundle_web_server = wasm_bundle_web_server.clone();
            let rollup_build_service = rollup_build_service.clone();
            let build_status_service = build_status_service.clone();
            let project_root_path = project_root_path.clone();
            async move {
                cancel_and_start_build(
                    wasm_bundle_web_server,
                    rollup_build_service,
                    build_status_service,
                    project_root_path,
                )
                .await;
                if let Some(after_first_build) = after_first_build {
                    (after_first_build)();
                }
                Ok(())
            }
        };

        let watch = node_project_watch_service.watch(project_root_path.clone(), {
            move || {
                let wasm_bundle_web_server = wasm_bundle_web_server.clone();
                let rollup_build_service = rollup_build_service.clone();
                let build_status_service = build_status_service.clone();
                let project_root_path = project_root_path.clone();
                tokio::spawn(cancel_and_start_build(
                    wasm_bundle_web_server,
                    rollup_build_service,
                    build_status_service,
                    project_root_path,
                ));
            }
        });
        try_join!(first_run, watch)?;

        Ok(())
    }

    pub fn just_build(build_status_service: Arc<BuildStatusService>) -> Result<()> {
        let rollup_project_root_path = get_cli_root_path().join("webCode");
        let rollup_build_service = Arc::new(RollupBuildService::new());

        block_on(build_status_service.build_started(BuildStatusCategory::WebRuntime));

        let cli_error_messages = install_deps()
            .err()
            .map_or(vec![], |error| vec![error.to_string()]);

        match rollup_build_service.cancel_and_start_build(&BuildOption {
            rollup_project_root_path,
            development: false,
        }) {
            BuildResult::Successful(rollup_build_result) => {
                block_on(build_status_service.build_finished(
                    BuildStatusCategory::WebRuntime,
                    rollup_build_result.error_messages,
                    cli_error_messages,
                ));
                Ok(())
            }
            BuildResult::Canceled => unreachable!(),
            BuildResult::Failed(error) => Err(anyhow!("{}", error)),
        }
    }
}

fn install_deps() -> Result<()> {
    let mut cmd = std::process::Command::new("npm");
    cmd.arg("i");
    cmd.current_dir(get_cli_root_path().join("webCode"));

    let output = cmd.output().unwrap();

    if !output.status.success() {
        return Err(anyhow!(
            "Failed to install dependencies: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }

    Ok(())
}
