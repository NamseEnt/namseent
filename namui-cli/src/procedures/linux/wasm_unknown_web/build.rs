use crate::{
    services::{
        bundle_metadata_service::BundleMetadataService,
        resource_collect_service::{CollectOperation, ResourceCollectService},
        wasm_watch_build_service::WasmWatchBuildService,
    },
    util::{
        get_cli_root_path, get_namui_bundle_manifest, overwrite_hot_reload_script_with_empty_file,
    },
};
use std::path::{Path, PathBuf};

pub fn build(manifest_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let build_dist_path = manifest_path.parent().unwrap().join("pkg");
    let namui_static_path = get_cli_root_path().join("www");
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let release_path = project_root_path
        .join("target")
        .join("namui")
        .join("wasm_unknown_web");

    let resource_collect_service = ResourceCollectService::new(&project_root_path, &release_path);

    WasmWatchBuildService::just_build(project_root_path.clone())?;

    let namui_bundle_manifest = get_namui_bundle_manifest(&project_root_path)?;
    let mut ops: Vec<CollectOperation> = namui_bundle_manifest
        .query(&project_root_path, &release_path)?
        .iter()
        .map(|(src_path, dest_path)| CollectOperation::new(src_path, dest_path))
        .collect();
    ops.push(CollectOperation::new(
        &namui_static_path,
        &PathBuf::from(""),
    ));
    ops.push(CollectOperation::new(
        &build_dist_path.join("bundle.js"),
        &PathBuf::from(""),
    ));
    ops.push(CollectOperation::new(
        &build_dist_path.join("bundle_bg.wasm"),
        &PathBuf::from(""),
    ));
    resource_collect_service.collect_resources(ops)?;

    let bundle_metadata_service = BundleMetadataService::new();
    bundle_metadata_service.load_bundle_manifest(&project_root_path)?;
    bundle_metadata_service.create_bundle_metadata_file(&release_path)?;

    overwrite_hot_reload_script_with_empty_file(&release_path)?;

    Ok(())
}
