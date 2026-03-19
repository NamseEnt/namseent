use crate::*;
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
};

use super::{dylib_wrapper, watcher, wait_changes};

static APP_CODE_CHANGED: AtomicBool = AtomicBool::new(false);
static RUNNER_EXITED: AtomicBool = AtomicBool::new(false);

pub fn start(project_path: &Path) -> Result<()> {
    // 1. Set up file watcher for the user project
    watcher::start_watcher(project_path.join("Cargo.toml"), || {
        APP_CODE_CHANGED.store(true, Ordering::Relaxed);
    });

    // 2. Find and build the native-runner binary
    let native_runner_dir = find_native_runner_dir(project_path)?;
    println!("-- Build Native Runner --");
    let status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir(&native_runner_dir)
        .status()?;
    if !status.success() {
        anyhow::bail!("Failed to build native-runner");
    }
    println!("-- Build Native Runner Done --");

    let runner_binary = native_runner_dir.join("target/release/native-runner");

    // 3. Generate cdylib wrapper and do initial dylib build
    build_dylib_step(project_path)?;

    let dylib_path = find_dylib_path(project_path);

    // 4. Find the system font directory
    let font_dir = find_font_dir(project_path)?;

    // 5. Launch the runner process with dylib path, project path, and font dir
    let mut runner = Command::new(&runner_binary)
        .arg(&dylib_path)
        .arg(project_path)
        .arg(&font_dir)
        .spawn()?;

    // 5. Monitor runner exit in a background thread
    let runner_id = runner.id();
    std::thread::spawn(move || {
        let _ = runner.wait();
        RUNNER_EXITED.store(true, Ordering::Relaxed);
    });

    println!("-- Native runner started (pid: {runner_id}) --");

    // 6. Watch loop: rebuild dylib on source changes, runner auto hot-reloads
    loop {
        wait_changes(&[&APP_CODE_CHANGED, &RUNNER_EXITED]);

        if RUNNER_EXITED.load(Ordering::Relaxed) {
            println!("-- Native runner exited --");
            return Ok(());
        }

        if APP_CODE_CHANGED.load(Ordering::Relaxed) {
            build_dylib_step(project_path)?;
            // The runner watches the dylib file and hot-reloads automatically
        }
    }
}

fn build_dylib_step(project_path: &Path) -> Result<()> {
    loop {
        APP_CODE_CHANGED.store(false, Ordering::Relaxed);
        dylib_wrapper::generate_dylib_wrapper_project(project_path)?;

        println!("-- Build Dylib --");
        let status = Command::new("cargo")
            .args(["build"])
            .current_dir(project_path.join("target/namui-native"))
            .env(
                "RUSTFLAGS",
                "-Clink-arg=-Wl,-undefined,dynamic_lookup",
            )
            .status()?;

        println!(
            "-- Build Dylib Done. isSuccessful : {} --",
            status.success()
        );

        if status.success() {
            return Ok(());
        }

        wait_changes(&[&APP_CODE_CHANGED, &RUNNER_EXITED]);

        if RUNNER_EXITED.load(Ordering::Relaxed) {
            anyhow::bail!("Native runner exited during dylib build retry");
        }
    }
}

fn find_dylib_path(project_path: &Path) -> PathBuf {
    project_path
        .join("target/namui-native/target/debug/libnamui_native_dylib.dylib")
}

/// Find the namui parent directory (e.g. `../namui/`) from the project's Cargo.toml
/// namui dependency path.
fn find_namui_parent_dir(project_path: &Path) -> Result<PathBuf> {
    let manifest_path = project_path.join("Cargo.toml");
    let manifest_contents = std::fs::read_to_string(&manifest_path)?;

    for line in manifest_contents.lines() {
        let line = line.trim();
        if line.starts_with("namui") && line.contains("path") {
            if let Some(path_start) = line.find("path") {
                let rest = &line[path_start..];
                if let Some(quote_start) = rest.find('"') {
                    let after_quote = &rest[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find('"') {
                        let namui_rel = &after_quote[..quote_end];
                        let namui_path = project_path.join(namui_rel);
                        let namui_parent = namui_path
                            .parent()
                            .ok_or_else(|| anyhow::anyhow!("namui path has no parent"))?;
                        return Ok(namui_parent.to_path_buf());
                    }
                }
            }
        }
    }

    anyhow::bail!(
        "Could not find namui path dependency in {}",
        manifest_path.display()
    )
}

fn find_native_runner_dir(project_path: &Path) -> Result<PathBuf> {
    let namui_parent = find_namui_parent_dir(project_path)?;
    let runner_dir = namui_parent.join("native-runner");
    if runner_dir.exists() {
        return Ok(runner_dir.canonicalize()?);
    }
    anyhow::bail!(
        "native-runner directory not found at {}",
        runner_dir.display()
    )
}

fn find_font_dir(project_path: &Path) -> Result<PathBuf> {
    let namui_parent = find_namui_parent_dir(project_path)?;
    let font_dir = namui_parent.join("namui-cli/system_bundle/font");
    if font_dir.exists() {
        return Ok(font_dir.canonicalize()?);
    }
    anyhow::bail!(
        "Font directory not found at {}",
        font_dir.display()
    )
}
