use super::{GenerateRuntimeProjectArgs, RuntimeProjectMode, get_namui_dep_path, get_project_name};
use crate::{util::recreate_dir_all, *};

pub fn generate_runtime_project(args: GenerateRuntimeProjectArgs) -> Result<()> {
    let project_name = get_project_name(args.project_path.clone());

    let project_path_in_relative =
        pathdiff::diff_paths(&args.project_path, &args.target_dir).unwrap();

    let project_path_str = project_path_in_relative
        .to_str()
        .unwrap()
        .split('\\')
        .collect::<Vec<&str>>()
        .join("/");

    recreate_dir_all(&args.target_dir, Some(vec![args.target_dir.join("target")]))?;

    // Resolve namui dependency path relative to target_dir
    let namui_abs_path = get_namui_dep_path(&args.project_path)
        .and_then(|abs_path| std::fs::canonicalize(&abs_path).ok());

    let namui_path_in_relative = namui_abs_path.as_ref().and_then(|canonical| {
        pathdiff::diff_paths(canonical, &args.target_dir).map(|p| {
            p.to_str()
                .unwrap()
                .split('\\')
                .collect::<Vec<&str>>()
                .join("/")
        })
    });

    // Derive native-runner path from namui path (sibling directory)
    let native_runner_path_in_relative = namui_abs_path.as_ref().and_then(|namui_canonical| {
        let parent = namui_canonical.parent()?;
        let runner_path = parent.join("native-runner");
        pathdiff::diff_paths(&runner_path, &args.target_dir).map(|p| {
            p.to_str()
                .unwrap()
                .split('\\')
                .collect::<Vec<&str>>()
                .join("/")
        })
    });

    match args.mode {
        RuntimeProjectMode::Binary => {
            generate_binary_project(
                &args.target_dir,
                &project_name,
                &project_path_str,
                namui_path_in_relative.as_deref(),
                native_runner_path_in_relative.as_deref(),
            )?;
        }
        RuntimeProjectMode::Cdylib => {
            generate_cdylib_project(
                &args.target_dir,
                &project_name,
                &project_path_str,
                namui_path_in_relative.as_deref(),
            )?;
        }
    }

    Ok(())
}

fn generate_binary_project(
    target_dir: &std::path::Path,
    project_name: &str,
    project_path: &str,
    namui_path: Option<&str>,
    native_runner_path: Option<&str>,
) -> Result<()> {
    let namui_dep = if let Some(path) = namui_path {
        format!(r#"namui = {{ path = "{path}" }}"#)
    } else {
        String::new()
    };

    let native_runner_dep = if let Some(path) = native_runner_path {
        format!(r#"native-runner = {{ path = "{path}" }}"#)
    } else {
        String::new()
    };

    std::fs::write(
        target_dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "namui-runtime-x86_64-pc-windows-msvc"
version = "0.0.1"
edition = "2024"

[dependencies]
{project_name} = {{ path = "{project_path}" }}
{namui_dep}
{native_runner_dep}
mimalloc = "0.1.39"

[profile.release]
opt-level = 3

[profile.dev]
opt-level = 2
"#
        ),
    )?;

    // src
    {
        recreate_dir_all(target_dir.join("src"), None)?;

        let project_name_underscored = project_name.replace('-', "_");
        std::fs::write(
            target_dir.join("src/main.rs"),
            format!(
                r#"#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() {{
    {project_name_underscored}::main();
    native_runner::run();
}}
"#
            ),
        )?;
    }

    // .cargo
    {
        recreate_dir_all(target_dir.join(".cargo"), None)?;

        std::fs::write(
            target_dir.join(".cargo/config.toml"),
            r#"
[build]
# https://store.steampowered.com/hwsurvey/Steam-Hardware-Software-Survey-Welcome-to-Steam
# support 99%, 2024-01-04
rustflags = ["-C", "target-feature=+sse,+sse2,+sse3,+cmpxchg16b,+ssse3,+sse4.1,+sse4.2"]
"#,
        )?;
    }

    Ok(())
}

fn generate_cdylib_project(
    target_dir: &std::path::Path,
    project_name: &str,
    project_path: &str,
    namui_path: Option<&str>,
) -> Result<()> {
    let namui_dep = if let Some(path) = namui_path {
        format!(r#"namui = {{ path = "{path}" }}"#)
    } else {
        String::new()
    };

    std::fs::write(
        target_dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "namui-runtime-x86_64-pc-windows-msvc"
version = "0.0.1"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
{project_name} = {{ path = "{project_path}" }}
{namui_dep}

[profile.release]
opt-level = 3

[profile.dev]
opt-level = 2
"#
        ),
    )?;

    recreate_dir_all(target_dir.join("src"), None)?;

    let project_name_underscored = project_name.replace('-', "_");
    std::fs::write(
        target_dir.join("src/lib.rs"),
        format!(
            r#"
#[unsafe(no_mangle)]
pub extern "C" fn namui_main() {{
    {project_name_underscored}::main();
}}
"#
        ),
    )?;

    // .cargo
    {
        recreate_dir_all(target_dir.join(".cargo"), None)?;

        std::fs::write(
            target_dir.join(".cargo/config.toml"),
            r#"
[build]
# https://store.steampowered.com/hwsurvey/Steam-Hardware-Software-Survey-Welcome-to-Steam
# support 99%, 2024-01-04
rustflags = ["-C", "target-feature=+sse,+sse2,+sse3,+cmpxchg16b,+ssse3,+sse4.1,+sse4.2"]
"#,
        )?;
    }

    Ok(())
}
