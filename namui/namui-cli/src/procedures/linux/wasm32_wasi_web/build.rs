use super::vite_config::{ViteConfig, update_vite_config};
use crate::cli::Target;
use crate::*;
use services::build_status_service::{BuildStatusCategory, BuildStatusService};
use services::runtime_project::{GenerateRuntimeProjectArgs, wasm::generate_runtime_project};
use services::rust_build_service::{self, BuildOption};
use util::get_cli_root_path;

pub async fn build(manifest_path: impl AsRef<std::path::Path>, release: bool) -> Result<()> {
    let manifest_path = manifest_path.as_ref();
    let target = Target::Wasm32WasiWeb;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let build_status_service = BuildStatusService::new();
    let runtime_target_dir = project_root_path.join("target/namui");
    let target_project_path = project_root_path.join(format!(
        "target/namui/target/wasm32-wasip1-threads/{}",
        if release { "release" } else { "debug" }
    ));

    generate_runtime_project(GenerateRuntimeProjectArgs {
        target_dir: runtime_target_dir.clone(),
        project_path: project_root_path.clone(),
        strip_debug_info: true,
    })?;

    build_status_service
        .build_started(BuildStatusCategory::Namui)
        .await;

    let result = rust_build_service::build(BuildOption {
        target,
        project_root_path: runtime_target_dir.clone(),
        release,
        watch: false,
    })
    .await??;

    build_status_service
        .build_finished(BuildStatusCategory::Namui, result.error_messages, vec![])
        .await;

    run_wasm_opt(&target_project_path, release).await?;

    let vite_config = ViteConfig {
        project_root_path: &project_root_path,
        release,
        host: None,
    };

    build_status_service
        .build_started(BuildStatusCategory::WebRuntime)
        .await;

    update_vite_config(&vite_config).await?;
    build_web_code().await?;

    build_status_service
        .build_finished(BuildStatusCategory::WebRuntime, vec![], vec![])
        .await;

    if std::fs::exists(target_project_path.join("dist"))? {
        std::fs::remove_dir_all(target_project_path.join("dist"))?;
    }

    if !tokio::process::Command::new("cp")
        .args([
            "-r",
            get_cli_root_path()
                .join("webCode/dist")
                .as_os_str()
                .to_str()
                .unwrap(),
            target_project_path.as_os_str().to_str().unwrap(),
        ])
        .output()
        .await?
        .status
        .success()
    {
        return Err(anyhow::anyhow!("cp failed"));
    };

    println!(
        "Build finished successfully. Output: {}",
        target_project_path.join("dist").display()
    );

    Ok(())
}

async fn run_wasm_opt(target_project_path: &std::path::Path, release: bool) -> Result<()> {
    let namui_runtime_wasm_path = target_project_path.join("namui-runtime-wasm.wasm");

    println!("Optimizing wasm file...");
    if !tokio::process::Command::new(get_cli_root_path().join("binaryen/bin").join("wasm-opt"))
        .args([
            namui_runtime_wasm_path.as_os_str().to_str().unwrap(),
            "--enable-bulk-memory",
            "--enable-threads",
            "--enable-exception-handling",
            "--enable-tail-call",
            "-O",
            "-o",
            target_project_path
                .join("namui-runtime-wasm.o.wasm")
                .as_os_str()
                .to_str()
                .unwrap(),
        ])
        .spawn()?
        .wait()
        .await?
        .success()
    {
        return Err(anyhow::anyhow!("wasm-opt failed"));
    }

    Ok(())
}

async fn build_web_code() -> Result<()> {
    let output = tokio::process::Command::new("npm")
        .current_dir(get_cli_root_path().join("webCode"))
        .args(["ci"])
        .spawn()?
        .wait()
        .await?;

    if !output.success() {
        return Err(anyhow::anyhow!("npm ci failed"));
    }

    let output = tokio::process::Command::new("npm")
        .current_dir(get_cli_root_path().join("webCode"))
        .args(["run", "build"])
        .spawn()?
        .wait()
        .await?;

    if !output.success() {
        return Err(anyhow::anyhow!("npm run build failed"));
    }

    Ok(())
}
