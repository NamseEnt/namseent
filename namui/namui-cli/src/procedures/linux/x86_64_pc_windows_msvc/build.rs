use crate::{
    services::{
        build_status_service::{BuildStatusCategory, BuildStatusService},
        resource_collect_service,
        runtime_project::x86_64_pc_windows_msvc::generate_runtime_project,
        rust_build_service::{self, BuildOption},
    },
    *,
};

pub async fn build(manifest_path: impl AsRef<std::path::Path>, release: bool) -> Result<()> {
    let manifest_path = manifest_path.as_ref();
    let target = cli::NamuiTarget::X86_64PcWindowsMsvc;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let release_path = project_root_path
        .join("target")
        .join("namui")
        .join("x86_64-pc-windows-msvc");
    let runtime_target_dir = project_root_path.join("target/namui");

    generate_runtime_project(services::runtime_project::GenerateRuntimeProjectArgs {
        target_dir: runtime_target_dir.clone(),
        project_path: project_root_path.clone(),
        strip_debug_info: true,
    })?;

    let build_status_service = BuildStatusService::new();

    build_status_service
        .build_started(services::build_status_service::BuildStatusCategory::Namui)
        .await;

    let cargo_build_output = rust_build_service::build(BuildOption {
        target: cli::NamuiTarget::X86_64PcWindowsMsvc,
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
    )?;

    Ok(())
}
