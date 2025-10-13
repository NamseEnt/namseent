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

    Ok(())
}
