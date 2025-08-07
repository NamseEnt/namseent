use crate::{
    services::{
        build_status_service::{BuildStatusCategory, BuildStatusService},
        resource_collect_service,
        runtime_project::x86_64_pc_windows_msvc::generate_runtime_project,
        rust_build_service::{self, BuildOption, BuildResult},
    },
    *,
};
use std::path::Path;

pub async fn build(manifest_path: impl AsRef<std::path::Path>) -> Result<()> {
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
    })?;

    let build_status_service = BuildStatusService::new();
    let rust_build_service = rust_build_service::RustBuildService::new();

    build_status_service
        .build_started(services::build_status_service::BuildStatusCategory::Namui)
        .await;

    match rust_build_service
        .cancel_and_start_build(&BuildOption {
            target: cli::NamuiTarget::X86_64PcWindowsMsvc,
            dist_path: release_path.clone(),
            project_root_path: runtime_target_dir,
            watch: false,
        })
        .await
    {
        BuildResult::Successful(cargo_build_result) => {
            build_status_service
                .build_finished(
                    BuildStatusCategory::Namui,
                    cargo_build_result.error_messages,
                    vec![],
                )
                .await;

            if !cargo_build_result.is_successful {
                return Err(anyhow!("Build failed"));
            }
        }
        BuildResult::Canceled => unreachable!(),
        BuildResult::Failed(error) => return Err(anyhow!("{}", error)),
    }

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
