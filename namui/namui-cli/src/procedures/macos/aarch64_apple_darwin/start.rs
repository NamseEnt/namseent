use crate::cli::Target;
use crate::*;
use services::build_status_service::{BuildStatusCategory, BuildStatusService};
use services::runtime_project::{
    GenerateRuntimeProjectArgs, RuntimeProjectMode,
    aarch64_apple_darwin::generate_runtime_project,
};
use services::rust_build_service::{self, BuildOption};
use services::rust_project_watch_service::RustProjectWatchService;

pub async fn start(
    manifest_path: impl AsRef<std::path::Path>,
    start_option: StartOption,
) -> Result<()> {
    let manifest_path = manifest_path.as_ref();
    let target = Target::Aarch64AppleDarwin;
    let project_root_path = manifest_path.parent().unwrap().to_path_buf();
    let release_path = project_root_path
        .join("target")
        .join("namui")
        .join("aarch64-apple-darwin");
    let runtime_target_dir = project_root_path.join("target/namui");

    let opt_level = if start_option.release {
        "release"
    } else {
        "debug"
    };

    // 1. Generate cdylib wrapper project
    generate_runtime_project(GenerateRuntimeProjectArgs {
        target_dir: runtime_target_dir.clone(),
        project_path: project_root_path.clone(),
        strip_debug_info: start_option.strip_debug_info,
        mode: RuntimeProjectMode::Cdylib,
    })?;

    let build_status_service = BuildStatusService::new();

    // 2. Build the runner binary (native-runner)
    println!("Building native-runner...");
    let runner_dir = find_native_runner_dir(&project_root_path)?;
    let runner_output = tokio::process::Command::new("cargo")
        .args([
            "build",
            "-p",
            "native-runner",
            "--target",
            "aarch64-apple-darwin",
            "--message-format",
            "json",
        ])
        .current_dir(&runner_dir)
        .output()
        .await?;

    if !runner_output.status.success() {
        let stderr = String::from_utf8_lossy(&runner_output.stderr);
        return Err(anyhow!("Failed to build native-runner: {stderr}"));
    }

    let runner_exe_path = runner_dir
        .join("target/aarch64-apple-darwin/debug/native-runner");

    // 3. Initial dylib build
    build_status_service
        .build_started(BuildStatusCategory::Namui)
        .await;

    let result = rust_build_service::build(BuildOption {
        target,
        project_root_path: runtime_target_dir.clone(),
        release: start_option.release,
        watch: false,
    })
    .await??;

    build_status_service
        .build_finished(BuildStatusCategory::Namui, result.error_messages, vec![])
        .await;

    if !result.is_successful {
        return Err(anyhow!("Initial dylib build failed"));
    }

    // 4. Collect resources
    let bundle_manifest =
        crate::services::bundle::NamuiBundleManifest::parse(project_root_path.clone())?;

    services::resource_collect_service::collect_all(
        &project_root_path,
        &release_path,
        target,
        bundle_manifest,
        None,
        start_option.release,
    )?;

    // 5. Find the dylib path
    let dylib_path = runtime_target_dir
        .join("target")
        .join("aarch64-apple-darwin")
        .join(opt_level)
        .join("libnamui_runtime_aarch64_apple_darwin.dylib");

    if !dylib_path.exists() {
        return Err(anyhow!(
            "Dylib not found at: {}",
            dylib_path.display()
        ));
    }

    // 6. Start the runner binary with dylib path as argument
    println!("Starting native-runner with dylib hot reload...");
    let mut runner_process = tokio::process::Command::new(&runner_exe_path)
        .arg(&dylib_path)
        .current_dir(&release_path)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .spawn()?;

    // 7. Watch for source changes and rebuild dylib
    let mut rust_project_watch = RustProjectWatchService::new(manifest_path)?;

    loop {
        tokio::select! {
            watch_result = rust_project_watch.next() => {
                match watch_result {
                    Ok(Some(())) => {
                        println!("Source changed, rebuilding dylib...");
                        build_status_service
                            .build_started(BuildStatusCategory::Namui)
                            .await;

                        let result = rust_build_service::build(BuildOption {
                            target,
                            project_root_path: runtime_target_dir.clone(),
                            release: start_option.release,
                            watch: false,
                        })
                        .await??;

                        build_status_service
                            .build_finished(BuildStatusCategory::Namui, result.error_messages, vec![])
                            .await;

                        if result.is_successful {
                            println!("Dylib rebuild successful, runner will hot reload.");
                        } else {
                            println!("Dylib rebuild failed.");
                        }
                    }
                    Ok(None) => break,
                    Err(e) => {
                        eprintln!("Watch error: {e}");
                        break;
                    }
                }
            }
            status = runner_process.wait() => {
                match status {
                    Ok(status) => println!("Runner exited with status: {status}"),
                    Err(e) => eprintln!("Runner error: {e}"),
                }
                break;
            }
        }
    }

    Ok(())
}

/// Find the native-runner crate directory.
/// It should be a sibling of the namui crate that the user's project depends on.
fn find_native_runner_dir(project_root_path: &std::path::Path) -> Result<std::path::PathBuf> {
    // Parse the user's Cargo.toml to find the namui dependency path
    let manifest_path = project_root_path.join("Cargo.toml");
    let manifest_contents = std::fs::read_to_string(&manifest_path)?;

    for line in manifest_contents.lines() {
        let line = line.trim();
        if line.starts_with("namui") && line.contains("path") {
            if let Some(path_start) = line.find("path") {
                let rest = &line[path_start..];
                if let Some(quote_start) = rest.find('"') {
                    let after_quote = &rest[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find('"') {
                        let path_str = &after_quote[..quote_end];
                        let namui_path = project_root_path.join(path_str);
                        let namui_path = std::fs::canonicalize(&namui_path)?;
                        // native-runner is a sibling of the namui crate
                        let parent = namui_path.parent().ok_or_else(|| {
                            anyhow!("Cannot find parent of namui crate")
                        })?;
                        let runner_dir = parent.join("native-runner");
                        if runner_dir.exists() {
                            return Ok(runner_dir);
                        }
                        return Err(anyhow!(
                            "native-runner directory not found at: {}",
                            runner_dir.display()
                        ));
                    }
                }
            }
        }
    }

    Err(anyhow!(
        "Could not find namui dependency path in {}",
        manifest_path.display()
    ))
}
