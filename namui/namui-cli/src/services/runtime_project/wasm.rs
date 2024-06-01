use super::{get_project_name, GenerateRuntimeProjectArgs};
use crate::{util::recreate_dir_all, *};
use util::get_cli_root_path;

pub fn generate_runtime_project(args: GenerateRuntimeProjectArgs) -> Result<()> {
    let project_name = get_project_name(args.project_path.clone());

    let project_path_in_relative =
        pathdiff::diff_paths(&args.project_path, &args.target_dir).unwrap();

    recreate_dir_all(&args.target_dir, Some(vec![args.target_dir.join("target")]))?;

    std::fs::write(
        args.target_dir.join("Cargo.toml"),
        format!(
            r#"[package]
name = "namui-runtime-wasm"
version = "0.0.1"
edition = "2021"

[dependencies]
{project_name} = {{ path = "{project_path}" }}

[profile.release]
lto = true
opt-level = 3

[profile.dev]
lto = true
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
                r#"
fn main() {{
    {project_name_underscored}::main();
}}
"#,
                project_name_underscored = project_name.replace('-', "_"),
            ),
        )?;
    }

    // .cargo
    {
        recreate_dir_all(args.target_dir.join(".cargo"), None)?;

        let wasi_sdk_path = get_cli_root_path().join("wasi-sdk");

        std::fs::write(
            args.target_dir.join(".cargo/config.toml"),
            format!(
                r#"
[build]
rustflags = [
    "--cfg",
    "tokio_unstable",
    "-Ctarget-feature=-crt-static",
    "-L{wasi_sdk_path}/share/wasi-sysroot/lib/wasm32-wasip1-threads",
    "-L{wasi_sdk_path}/lib/clang/18/lib/wasip1",
    # wasm-ld --initial-memory 20MB
    "-Clink-arg=--initial-memory=20971520",
    # wasm-ld --max-memory 1G
    "-Clink-arg=--max-memory=1073741824",
]
"#,
                wasi_sdk_path = wasi_sdk_path.to_string_lossy(),
            ),
        )?;
    }

    Ok(())
}
