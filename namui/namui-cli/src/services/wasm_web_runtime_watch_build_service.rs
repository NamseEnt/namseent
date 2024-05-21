use super::build_status_service::BuildStatusService;
use super::rollup_build_service::RollupBuildOutput;
use super::wasm_bundle_web_server::WasmBundleWebServer;
use crate::services::build_status_service::BuildStatusCategory;
use crate::services::node_project_watch_service::NodeProjectWatchService;
use crate::services::rollup_build_service;
use crate::*;
use crate::{services::rollup_build_service::BuildOption, util::get_cli_root_path};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

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

        let mut node_project_watch_service = NodeProjectWatchService::new(&project_root_path)?;
        let after_first_build = Arc::new(Mutex::new(after_first_build));

        let after_build = move |rollup_build_result: RollupBuildOutput| {
            let wasm_bundle_web_server = wasm_bundle_web_server.clone();
            async move {
                let error_messages = rollup_build_result.error_messages;
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

        let mut handle = start_build(
            &project_root_path,
            after_build.clone(),
            build_status_service.clone(),
        );

        while node_project_watch_service.next().await.is_some() {
            handle.abort();

            handle = start_build(
                &project_root_path,
                after_build.clone(),
                build_status_service.clone(),
            );
        }

        Ok(())
    }

    pub async fn just_build(build_status_service: Arc<BuildStatusService>) -> Result<()> {
        let project_root_path = get_cli_root_path().join("webCode");

        start_build(
            &project_root_path,
            |rollup_build_result: RollupBuildOutput| async move {},
            build_status_service,
        )
        .await??;

        Ok(())
    }
}

async fn install_deps() -> Result<()> {
    let output = tokio::process::Command::new("npm")
        .arg("i")
        .current_dir(get_cli_root_path().join("webCode"))
        .output()
        .await?;

    if !output.status.success() {
        let stdout = String::from_utf8(output.stdout)?;
        let stderr = String::from_utf8(output.stderr)?;
        tokio::fs::write(
            "npm_install_err.json",
            format!("stdout: {stdout}\n\nstderr: {stderr}"),
        )
        .await?;
        return Err(anyhow!(
            "Failed to install dependencies. check ./npm_install_err.json",
        ));
    }

    Ok(())
}

fn start_build<Fut: std::future::Future + Send>(
    rollup_project_root_path: &Path,
    after_build: impl 'static + Send + FnOnce(RollupBuildOutput) -> Fut,
    build_status_service: Arc<BuildStatusService>,
) -> tokio::task::JoinHandle<Result<()>> {
    let rollup_project_root_path = rollup_project_root_path.to_path_buf();
    tokio::spawn(async move {
        build_status_service
            .build_started(BuildStatusCategory::WebRuntime)
            .await;

        // TODO: Handle error. show error message to user
        install_deps().await?;

        let rollup_build_result = rollup_build_service::build(BuildOption {
            rollup_project_root_path,
            development: true,
        })
        .await??;

        build_status_service
            .build_finished(
                BuildStatusCategory::WebRuntime,
                rollup_build_result.error_messages.clone(),
                vec![],
            )
            .await;

        after_build(rollup_build_result).await;

        Ok(())
    })
}
