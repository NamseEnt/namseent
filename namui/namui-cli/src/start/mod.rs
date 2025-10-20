mod app_wrapper;
mod wasi_cargo_envs;
mod watcher;

use crate::{util::get_cli_root_path, *};
use std::{
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
    thread,
    time::Duration,
};
use wasi_cargo_envs::{WasiType, wasi_cargo_envs};

static DRAWER_CODE_CHANGED: AtomicBool = AtomicBool::new(false);
static APP_CODE_CHANGED: AtomicBool = AtomicBool::new(false);

fn drawer_project_path() -> PathBuf {
    get_cli_root_path().join("../namui-drawer")
}

pub fn start(project_path: &Path) -> Result<()> {
    watcher::start_watcher(drawer_project_path().join("Cargo.toml"), || {
        DRAWER_CODE_CHANGED.store(true, Ordering::Relaxed);
    });

    watcher::start_watcher(project_path.join("Cargo.toml"), || {
        APP_CODE_CHANGED.store(true, Ordering::Relaxed);
    });

    let mut vite = None;

    build_drawer_step()?;
    build_app_step(project_path)?;

    static VITE: std::sync::Once = std::sync::Once::new();
    VITE.call_once(|| {
        vite = Some(
            Command::new("npm")
                .args(["run", "dev"])
                .current_dir(get_cli_root_path().join("webCode"))
                .envs([("NAMUI_APP_PATH", project_path.to_str().unwrap())])
                .spawn()
                .unwrap(),
        );
    });

    loop {
        wait_changes(&[&DRAWER_CODE_CHANGED, &APP_CODE_CHANGED]);

        if DRAWER_CODE_CHANGED.load(Ordering::Relaxed) {
            build_drawer_step()?;
        }

        if APP_CODE_CHANGED.load(Ordering::Relaxed) {
            build_app_step(project_path)?;
        }
    }
}

fn build_drawer_step() -> Result<()> {
    loop {
        println!("-- Build Drawer-- ");
        DRAWER_CODE_CHANGED.store(false, Ordering::Relaxed);

        let status = Command::new("cargo")
            .args(["build", "--target", "wasm32-wasip1-threads", "--release"])
            .current_dir(drawer_project_path())
            .envs(wasi_cargo_envs(WasiType::Drawer))
            .status()?;

        println!(
            "-- Build Drawer Done. isSuccessful : {} -- ",
            status.success()
        );

        if status.success() {
            return Ok(());
        }

        wait_changes(&[&DRAWER_CODE_CHANGED]);
    }
}

fn build_app_step(project_path: &Path) -> Result<()> {
    loop {
        APP_CODE_CHANGED.store(false, Ordering::Relaxed);
        app_wrapper::generate_app_wrapper_project(project_path)?;

        let status = Command::new("cargo")
            .args(["build", "--target", "wasm32-wasip1-threads"])
            .current_dir(project_path.join("target/namui"))
            .envs(wasi_cargo_envs(WasiType::App))
            .status()?;

        println!("-- Build App Done. isSuccessful : {} -- ", status.success());

        if status.success() {
            return Ok(());
        }

        wait_changes(&[&APP_CODE_CHANGED]);
    }
}

fn wait_changes(atomics: &[&AtomicBool]) {
    loop {
        if atomics.iter().any(|atomic| atomic.load(Ordering::Relaxed)) {
            break;
        }
        thread::sleep(Duration::from_millis(30));
    }
}
