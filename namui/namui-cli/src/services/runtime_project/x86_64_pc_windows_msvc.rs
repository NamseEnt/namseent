use super::{GenerateRuntimeProjectArgs, get_project_name};
use crate::{util::recreate_dir_all, *};

pub fn generate_runtime_project(args: GenerateRuntimeProjectArgs) -> Result<()> {
    let project_name = get_project_name(args.project_path.clone());

    let project_path_in_relative =
        pathdiff::diff_paths(&args.project_path, &args.target_dir).unwrap();

    recreate_dir_all(&args.target_dir, Some(vec![args.target_dir.join("target")]))?;

    std::fs::write(
        args.target_dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "namui-runtime-x86_64-pc-windows-msvc"
version = "0.0.1"
edition = "2024"

[dependencies]
{project_name} = {{ path = "{project_path}" }}
mimalloc = "0.1.39"

[profile.release]
opt-level = 3

[profile.dev]
opt-level = 2
    "#,
            project_path = project_path_in_relative
                .to_str()
                .unwrap()
                .split('\\')
                .collect::<Vec<&str>>()
                .join("/"),
        ),
    )?;

    // src
    {
        recreate_dir_all(args.target_dir.join("src"), None)?;

        std::fs::write(
            args.target_dir.join("src/main.rs"),
            format!(
                r#"#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
                
fn main() {{
    {project_name_underscored}::main()
}}
"#,
                project_name_underscored = project_name.replace('-', "_"),
            ),
        )?;
    }

    // .cargo
    {
        recreate_dir_all(args.target_dir.join(".cargo"), None)?;

        std::fs::write(
            args.target_dir.join(".cargo/config.toml"),
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
