use crate::services::build_status_service::BuildStatusService;
use crate::services::wasm_web_runtime_watch_build_service::WasmWebRuntimeWatchBuildService;
use crate::*;
use crate::{
    cli::Target,
    services::{resource_collect_service, wasm_watch_build_service::WasmWatchBuildService},
};
use std::path::Path;

pub fn build(manifest_path: &Path) -> Result<()> {
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let release_path = project_root_path
        .join("target")
        .join("namui")
        .join("wasm_unknown_web");

    let build_status_service = BuildStatusService::new();
    WasmWebRuntimeWatchBuildService::just_build(build_status_service.clone())?;
    WasmWatchBuildService::just_build(
        build_status_service,
        project_root_path.clone(),
        Target::WasmUnknownWeb,
    )?;

    let bundle_manifest =
        crate::services::bundle::NamuiBundleManifest::parse(project_root_path.clone())?;

    resource_collect_service::collect_all(
        &project_root_path,
        &release_path,
        Target::WasmUnknownWeb,
        bundle_manifest,
        None,
    )?;

    Ok(())
}
