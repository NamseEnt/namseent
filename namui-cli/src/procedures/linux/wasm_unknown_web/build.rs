use crate::services::build_status_service::BuildStatusService;
use crate::services::drawer_watch_build_service::DrawerWatchBuildService;
use crate::services::wasm_web_runtime_watch_build_service::WasmWebRuntimeWatchBuildService;
use crate::*;
use crate::{
    cli::Target,
    services::{resource_collect_service, wasm_watch_build_service::WasmWatchBuildService},
};
use std::path::Path;
use tokio::try_join;

pub async fn build(manifest_path: &Path) -> Result<()> {
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let release_path = project_root_path
        .join("target")
        .join("namui")
        .join("wasm_unknown_web");
    let target = Target::WasmUnknownWeb;

    let build_status_service = BuildStatusService::new();

    let web_runtime_build =
        WasmWebRuntimeWatchBuildService::just_build(build_status_service.clone());
    let wasm_build = WasmWatchBuildService::just_build(
        build_status_service.clone(),
        project_root_path.clone(),
        target,
    );
    let drawer_build = DrawerWatchBuildService::just_build(target, build_status_service.clone());

    try_join!(web_runtime_build, wasm_build, drawer_build)?;

    let bundle_manifest =
        crate::services::bundle::NamuiBundleManifest::parse(project_root_path.clone())?;

    resource_collect_service::collect_all(
        &project_root_path,
        &release_path,
        target,
        bundle_manifest,
        None,
    )?;

    Ok(())
}
