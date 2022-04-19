use crate::{
    cli::Target,
    debug_println,
    services::{
        electron_package_service::{Arch, ElectronPackageService, Platform},
        resource_collect_service::{CollectOperation, ResourceCollectService},
        rust_build_service::{BuildOption, BuildResult, RustBuildService},
    },
    util::{
        get_cli_root_path, get_namui_bundle_manifest, overwrite_hot_reload_script_with_empty_file,
        print_build_result,
    },
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub fn build(manifest_path: &Path, arch: Option<Arch>) -> Result<(), Box<dyn std::error::Error>> {
    let build_dist_path = manifest_path.parent().unwrap().join("pkg");
    let namui_static_path = get_cli_root_path().join("www");
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();

    let electron_package_service = ElectronPackageService::new();
    let package_result = electron_package_service.package_electron_app(arch, Platform::Win32)?;

    let release_path = project_root_path
        .join("target")
        .join("namui")
        .join(format!("wasm_windows_{}_electron", &package_result.arch));

    let rust_build_service = Arc::new(RustBuildService::new());
    let resource_collect_service = ResourceCollectService::new(&project_root_path, &release_path);

    build_(
        rust_build_service.clone(),
        build_dist_path.clone(),
        project_root_path.clone(),
    )?;

    let namui_bundle_manifest = get_namui_bundle_manifest(&project_root_path)?;
    let mut ops: Vec<CollectOperation> = namui_bundle_manifest
        .query(&project_root_path, &release_path)?
        .iter()
        .map(|(src_path, dest_path)| CollectOperation::new(src_path, dest_path))
        .collect();
    ops.push(CollectOperation::new(
        &namui_static_path,
        &PathBuf::from("resources"),
    ));
    ops.push(CollectOperation::new(
        &build_dist_path.join("bundle.js"),
        &PathBuf::from("resources"),
    ));
    ops.push(CollectOperation::new(
        &build_dist_path.join("bundle_bg.wasm"),
        &PathBuf::from("resources"),
    ));
    ops.push(CollectOperation::new(
        &PathBuf::from(&package_result.output_path),
        &PathBuf::from(""),
    ));
    resource_collect_service.collect_resources(ops)?;

    overwrite_hot_reload_script_with_empty_file(&release_path)?;

    Ok(())
}

fn build_(
    rust_build_service: Arc<RustBuildService>,
    build_dist_path: PathBuf,
    project_root_path: PathBuf,
) -> Result<(), String> {
    debug_println!("build fn run");
    match rust_build_service.cancel_and_start_build(&BuildOption {
        target: Target::WasmWindowsElectron,
        dist_path: build_dist_path.to_path_buf(),
        project_root_path: project_root_path.to_path_buf(),
        watch: false,
    }) {
        BuildResult::Successful(cargo_build_result) => {
            print_build_result(&cargo_build_result.error_messages, &vec![]);
            Ok(())
        }
        BuildResult::Canceled => unreachable!(),
        BuildResult::Failed(error) => Err(error),
    }
}
