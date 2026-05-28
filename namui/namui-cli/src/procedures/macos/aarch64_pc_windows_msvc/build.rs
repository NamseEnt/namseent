use crate::{
    services::{
        build_status_service::{BuildStatusCategory, BuildStatusService},
        icon_service, resource_collect_service,
        runtime_project::{
            RuntimeProjectMode, aarch64_pc_windows_msvc::generate_runtime_project,
        },
        rust_build_service::{self, BuildOption},
    },
    *,
};

pub async fn build(manifest_path: impl AsRef<std::path::Path>, release: bool) -> Result<()> {
    let manifest_path = manifest_path.as_ref();
    let target = cli::Target::Aarch64PcWindowsMsvc;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let release_path = project_root_path
        .join("target")
        .join("namui")
        .join("aarch64-pc-windows-msvc");
    let runtime_target_dir = project_root_path.join("target/namui");
    let icon_path = icon_service::read_icon_path(manifest_path)?;

    generate_runtime_project(services::runtime_project::GenerateRuntimeProjectArgs {
        target_dir: runtime_target_dir.clone(),
        project_path: project_root_path.clone(),
        strip_debug_info: true,
        mode: RuntimeProjectMode::Binary,
        icon_path: icon_path.clone(),
    })?;

    let build_status_service = BuildStatusService::new();

    build_status_service
        .build_started(services::build_status_service::BuildStatusCategory::Namui)
        .await;

    let cargo_build_output = rust_build_service::build(BuildOption {
        target: cli::Target::Aarch64PcWindowsMsvc,
        project_root_path: runtime_target_dir,
        watch: false,
        release,
    })
    .await??;
    build_status_service
        .build_finished(
            BuildStatusCategory::Namui,
            cargo_build_output.error_messages,
            vec![],
        )
        .await;

    let bundle_manifest =
        crate::services::bundle::NamuiBundleManifest::parse(project_root_path.clone())?;

    resource_collect_service::collect_all(
        &project_root_path,
        &release_path,
        target,
        bundle_manifest,
        None,
        release,
        icon_path.as_deref(),
    )?;

    Ok(())
}
