use super::{GenerateRuntimeProjectArgs, get_project_name};
use crate::{util::recreate_dir_all, *};
use services::wasi_cargo_envs::wasi_cargo_envs;
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
edition = "2024"

[dependencies]
{project_name} = {{ path = "{project_path}" }}

[profile.release]
opt-level = 3
lto = true
codegen-units = 1 # Reduce number of codegen units to increase optimizations
strip = {strip}
debug = {debug}

[profile.dev]
opt-level = 1
# opt-level = 3
strip = {strip}
debug = {debug}
"#,
            project_path = project_path_in_relative
                .to_str()
                .unwrap()
                .split('\\')
                .collect::<Vec<&str>>()
                .join("/"),
            strip = if args.strip_debug_info {
                "true"
            } else {
                "false"
            },
            debug = if args.strip_debug_info {
                "false"
            } else {
                "true"
            },
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
    "-Ctarget-feature=-crt-static",
    "-L{wasi_sdk_path}/share/wasi-sysroot/lib/wasm32-wasip2",
    "-L{wasi_sdk_path}/lib/clang/20/lib/wasm32-unknown-wasip2",
    # 2MB: 2097152
    # 8MB: 8388608
    # 256MB: 268435456
    # 4GB: 4294967296
    # stack size 8MB

    "-Clink-arg=--initial-memory=8388608",
    "-Clink-arg=--max-memory=4294967296",
    "-Clink-arg=--stack-first",

    "-Clink-arg=--export=__heap_base",
    "-Clink-arg=--export=__data_end",
    "-Clink-arg=--export=malloc",
    "-Clink-arg=--export=free",

    # Supported by every main browser: https://caniuse.com/wasm-simd
    "-Ctarget-feature=+simd128",
]

[env]
{env}
"#,
                wasi_sdk_path = wasi_sdk_path.to_string_lossy(),
                env = wasi_cargo_envs()
                    .map(|(key, value)| format!("{key:?}={value:?}"))
                    .join("\n"),
            ),
        )?;
    }

    Ok(())
}
