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

    // Helper to derive sibling paths from namui parent directory
    let sibling_path = |dir_name: &str| -> Option<String> {
        namui_abs_path.as_ref().and_then(|namui_canonical| {
            let parent = namui_canonical.parent()?;
            let sibling = parent.join(dir_name);
            pathdiff::diff_paths(&sibling, &args.target_dir).map(|p| {
                p.to_str()
                    .unwrap()
                    .split('\\')
                    .collect::<Vec<&str>>()
                    .join("/")
            })
        })
    };

    let native_runner_path_in_relative = sibling_path("native-runner");
    let audio_native_path_in_relative = sibling_path("audio-native");
    let kv_store_native_path_in_relative = sibling_path("kv-store-native");

    match args.mode {
        RuntimeProjectMode::Binary => {
            generate_binary_project(
                &args.target_dir,
                &project_name,
                &project_path_str,
                namui_path_in_relative.as_deref(),
                native_runner_path_in_relative.as_deref(),
                audio_native_path_in_relative.as_deref(),
                kv_store_native_path_in_relative.as_deref(),
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
    audio_native_path: Option<&str>,
    kv_store_native_path: Option<&str>,
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

    let audio_native_dep = if let Some(path) = audio_native_path {
        format!(r#"namui-audio-native = {{ path = "{path}" }}"#)
    } else {
        String::new()
    };

    let kv_store_native_dep = if let Some(path) = kv_store_native_path {
        format!(r#"namui-kv-store-native = {{ path = "{path}" }}"#)
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
{audio_native_dep}
{kv_store_native_dep}
mimalloc = "0.1.39"
rusqlite = {{ version = "0.31.0", features = ["blob", "bundled"] }}

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
    let exe_dir = std::env::current_exe()
        .expect("Failed to get current exe path")
        .parent()
        .unwrap()
        .to_path_buf();
    let bundle_path = exe_dir.join("bundle.sqlite");
    {project_name_underscored}::asset::init_native_assets(|relative_path| {{
        use std::io::Read;
        let asset_path = format!("asset/{{}}", relative_path);
        let conn = rusqlite::Connection::open_with_flags(
            &bundle_path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        ).unwrap_or_else(|e| panic!("Failed to open bundle.sqlite: {{e}}"));
        let rowid: i64 = conn.query_row(
            "SELECT rowid FROM bundle WHERE path = ?",
            [&asset_path],
            |row| row.get(0),
        ).unwrap_or_else(|e| panic!("Asset not found in bundle '{{}}': {{e}}", asset_path));
        let mut blob = conn.blob_open(
            rusqlite::DatabaseName::Main,
            "bundle",
            "data",
            rowid,
            true,
        ).unwrap_or_else(|e| panic!("Failed to open blob for '{{}}': {{e}}", asset_path));
        let mut data = vec![0u8; blob.len()];
        blob.read_exact(&mut data)
            .unwrap_or_else(|e| panic!("Failed to read blob for '{{}}': {{e}}", asset_path));
        data
    }});
    {project_name_underscored}::main();
    native_runner::run();
}}

#[unsafe(no_mangle)]
pub extern "C" fn namui_main() {{
    // Already called from main(), this is a no-op for binary mode.
}}

#[unsafe(no_mangle)]
pub extern "C" fn _dylib_image_buffer_list(out: *mut usize, max_count: usize) -> usize {{
    let list = namui::image_buffer_list();
    let count = list.len().min(max_count);
    let out = unsafe {{ std::slice::from_raw_parts_mut(out, count * 3) }};
    for (i, [id, ptr, len]) in list.iter().enumerate().take(count) {{
        out[i * 3] = *id;
        out[i * 3 + 1] = *ptr;
        out[i * 3 + 2] = *len;
    }}
    count
}}

#[unsafe(no_mangle)]
pub extern "C" fn _dylib_register_font(
    name_ptr: *const u8,
    name_len: usize,
    buffer_ptr: *const u8,
    buffer_len: usize,
) {{
    let name = unsafe {{ std::str::from_utf8_unchecked(std::slice::from_raw_parts(name_ptr, name_len)) }};
    let bytes = unsafe {{ std::slice::from_raw_parts(buffer_ptr, buffer_len) }};
    namui::NativeTypeface::load(name, bytes)
        .unwrap_or_else(|e| panic!("Failed to load font {{name}}: {{e}}"));
}}

#[unsafe(no_mangle)]
pub extern "C" fn _dylib_set_image_infos(ptr: *const u8, count: usize) {{
    unsafe {{ namui::_set_image_infos(ptr, count) }};
}}

extern crate namui_audio_native;
extern crate namui_kv_store_native;
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
