use super::{get_project_name, GenerateRuntimeProjectArgs};
use crate::*;

pub fn generate_runtime_project(args: GenerateRuntimeProjectArgs) -> Result<()> {
    let project_name = get_project_name(args.project_path.clone());

    let project_path_in_relative =
        pathdiff::diff_paths(&args.project_path, &args.target_dir).unwrap();

    std::fs::create_dir_all(args.target_dir.join("src"))?;

    let cargo_toml = format!(
        r#"[package]
name = "namui-runtime-x86_64-pc-windows-msvc"
version = "0.0.1"
edition = "2021"

[dependencies]
{project_name} = {{ path = "{project_path}" }}
tokio = {{ version = "1.12.0", features = ["rt"] }}

[profile.release]
lto = true
opt-level = 3

# [profile.dev]
# lto = true
# opt-level = 2
    "#,
        project_path = project_path_in_relative
            .to_str()
            .unwrap()
            .split('\\')
            .collect::<Vec<&str>>()
            .join("/"),
    );
    std::fs::write(args.target_dir.join("Cargo.toml"), cargo_toml)?;

    let lib_rs = format!(
        r#"#[tokio::main]
async fn main() {{
    let local_set = tokio::task::LocalSet::new();
    local_set.run_until(async {{
        {project_name_underscored}::main().await;
    }}).await;
    local_set.await;
}}
"#,
        project_name_underscored = project_name.replace('-', "_"),
    );
    std::fs::write(args.target_dir.join("src/main.rs"), lib_rs)?;

    Ok(())
}
