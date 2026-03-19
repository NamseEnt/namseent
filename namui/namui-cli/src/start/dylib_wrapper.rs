use crate::{util::recreate_dir_all, *};
use std::path::Path;

pub fn generate_dylib_wrapper_project(app_project_path: &Path) -> Result<()> {
    let app_name = get_app_name(app_project_path);
    let namui_rel_path = get_namui_relative_path(app_project_path)?;
    let wrapper_project_path = app_project_path.join("target/namui-native");

    recreate_dir_all(
        &wrapper_project_path,
        Some(vec![wrapper_project_path.join("target")]),
    )?;

    // The wrapper project is at {project}/target/namui-native/,
    // so we need ../../ prefix to reach the project root.
    let app_dep_path = "../../";
    // namui_rel_path is relative from the user project root,
    // so from wrapper it's ../../{namui_rel_path}
    let namui_dep_path = format!("../../{namui_rel_path}");

    std::fs::write(
        wrapper_project_path.join("Cargo.toml"),
        format!(
            r#"[package]
name = "namui-native-dylib"
version = "0.0.1"
edition = "2024"

[lib]
crate-type = ["cdylib"]

[dependencies]
{app_name} = {{ path = "{app_dep_path}" }}
namui = {{ path = "{namui_dep_path}" }}

[profile.dev]
opt-level = 1
strip = "debuginfo"
debug = "line-tables-only"
"#,
        ),
    )?;

    recreate_dir_all(wrapper_project_path.join("src"), None)?;

    let project_name_underscored = app_name.replace('-', "_");
    std::fs::write(
        wrapper_project_path.join("src/lib.rs"),
        format!(
            r#"#[unsafe(no_mangle)]
pub extern "C" fn namui_main() {{
    {project_name_underscored}::asset::init_native_assets();
    {project_name_underscored}::main();
}}

/// Write image buffer info from the dylib's registry into a flat array.
/// Each entry is 3 usizes: [id, ptr, len].
/// Returns the number of images written.
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

/// Register a font into the dylib's NativeTypeface map.
/// The runner binary has its own separate static map, so it must call
/// this function to also populate the dylib's map.
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

/// Register image infos into the dylib's IMAGE_INFOS map.
/// The runner has its own separate IMAGE_INFOS, so it must forward
/// the info to the dylib so app code can call Image::info().
#[unsafe(no_mangle)]
pub extern "C" fn _dylib_set_image_infos(ptr: *const u8, count: usize) {{
    unsafe {{ namui::_set_image_infos(ptr, count) }};
}}

/// No-op audio FFI stubs.
/// These must live in the dylib (not the runner binary) because macOS
/// executables do not export their symbols to dlopen'd libraries.
/// The namui crate declares these as `extern "C"` imports and the
/// dylib was built with `-undefined dynamic_lookup`, so they resolve
/// here at link time.
mod audio_stubs {{
    #[unsafe(no_mangle)]
    pub extern "C" fn _audio_play(_audio_id: usize, _playback_id: usize, _repeat: bool) {{}}
    #[unsafe(no_mangle)]
    pub extern "C" fn _audio_play_spatial(_audio_id: usize, _playback_id: usize, _repeat: bool) {{}}
    #[unsafe(no_mangle)]
    pub extern "C" fn _audio_playback_drop(_playback_id: usize) {{}}
    #[unsafe(no_mangle)]
    pub extern "C" fn _audio_playback_set_volume(_playback_id: usize, _volume: f32) {{}}
    #[unsafe(no_mangle)]
    pub extern "C" fn _audio_playback_set_position(_playback_id: usize, _x: f32, _y: f32, _z: f32) {{}}
    #[unsafe(no_mangle)]
    pub extern "C" fn _audio_set_listener_position(_x: f32, _y: f32, _z: f32) {{}}
    #[unsafe(no_mangle)]
    pub extern "C" fn _audio_set_volume(_volume: f32) {{}}
}}
"#,
        ),
    )?;

    Ok(())
}

fn get_app_name(project_path: &Path) -> String {
    let manifest_path = project_path.join("Cargo.toml");
    let manifest_contents = std::fs::read_to_string(manifest_path).unwrap();
    manifest_contents
        .split("name = ")
        .nth(1)
        .unwrap()
        .split('\n')
        .next()
        .unwrap()
        .trim()
        .replace('"', "")
        .to_string()
}

fn get_namui_relative_path(project_path: &Path) -> Result<String> {
    let manifest_path = project_path.join("Cargo.toml");
    let manifest_contents = std::fs::read_to_string(&manifest_path)?;

    // Parse namui = { path = "..." } from Cargo.toml
    for line in manifest_contents.lines() {
        let line = line.trim();
        if line.starts_with("namui") && line.contains("path") {
            // Extract the path value
            if let Some(path_start) = line.find("path") {
                let rest = &line[path_start..];
                if let Some(quote_start) = rest.find('"') {
                    let after_quote = &rest[quote_start + 1..];
                    if let Some(quote_end) = after_quote.find('"') {
                        return Ok(after_quote[..quote_end].to_string());
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
