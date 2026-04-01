use proc_macro::TokenStream;
use quote::quote;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Default)]
struct ModuleNode {
    children: BTreeMap<String, ModuleNode>,
    images: Vec<(String, usize)>,
    audios: Vec<(String, usize, i64)>,
}

impl ModuleNode {
    fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            images: Vec::new(),
            audios: Vec::new(),
        }
    }
    fn add_image(&mut self, path_parts: &[String], file_name: String, id: usize) {
        if path_parts.is_empty() {
            self.images.push((file_name, id));
        } else {
            let child = self.children.entry(path_parts[0].clone()).or_default();
            child.add_image(&path_parts[1..], file_name, id);
        }
    }
    fn add_audio(
        &mut self,
        path_parts: &[String],
        file_name: String,
        id: usize,
        duration_millis: i64,
    ) {
        if path_parts.is_empty() {
            self.audios.push((file_name, id, duration_millis));
        } else {
            let child = self.children.entry(path_parts[0].clone()).or_default();
            child.add_audio(&path_parts[1..], file_name, id, duration_millis);
        }
    }
    fn has_images(&self) -> bool {
        !self.images.is_empty() || self.children.values().any(|c| c.has_images())
    }
    fn has_audios(&self) -> bool {
        !self.audios.is_empty() || self.children.values().any(|c| c.has_audios())
    }
    fn to_tokens(&self) -> proc_macro2::TokenStream {
        let mut modules = Vec::new();

        for (name, child) in &self.children {
            let mod_ident = quote::format_ident!("{}", name);
            let child_tokens = child.to_tokens();
            let image_use = if child.has_images() {
                quote! { use super::Image; }
            } else {
                quote! {}
            };
            let audio_use = if child.has_audios() {
                quote! { use super::AudioAsset; use super::Duration; }
            } else {
                quote! {}
            };
            modules.push(quote! {
                pub mod #mod_ident {
                    #image_use
                    #audio_use
                    #child_tokens
                }
            });
        }

        let mut images = Vec::new();
        for (name, id) in &self.images {
            let const_name = quote::format_ident!("{}", name);
            images.push(quote! {
                pub static #const_name: Image = Image::new(#id);
            });
        }

        let mut audios = Vec::new();
        for (name, id, duration_millis) in &self.audios {
            let const_name = quote::format_ident!("{}", name);
            audios.push(quote! {
                pub static #const_name: AudioAsset = AudioAsset::new(#id, Duration::from_millis(#duration_millis));
            });
        }

        quote! {
            #(#modules)*
            #(#images)*
            #(#audios)*
        }
    }
}
fn file_stem_to_const_name(file_stem: &str) -> String {
    file_stem
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

fn collect_image_files(asset_dir: &Path) -> Vec<PathBuf> {
    collect_files_by_extensions(asset_dir, &["jpg", "jpeg", "png"])
}

fn collect_audio_files(asset_dir: &Path) -> Vec<PathBuf> {
    collect_files_by_extensions(asset_dir, &["mp3", "wav", "ogg", "opus"])
}

fn collect_files_by_extensions(asset_dir: &Path, extensions: &[&str]) -> Vec<PathBuf> {
    let mut files = Vec::new();

    if !asset_dir.exists() {
        return files;
    }

    fn visit_dirs(dir: &Path, files: &mut Vec<PathBuf>, extensions: &[&str]) {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    visit_dirs(&path, files, extensions);
                } else if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if extensions.contains(&ext_str.as_str()) {
                            files.push(path);
                        }
                    }
                }
            }
        }
    }

    visit_dirs(asset_dir, &mut files, extensions);
    files
}

fn path_to_parts(asset_dir: &Path, file_path: &Path) -> (Vec<String>, String) {
    let relative_path = file_path
        .strip_prefix(asset_dir)
        .expect("Failed to strip asset directory prefix");

    let components: Vec<String> = relative_path
        .parent()
        .map(|p| {
            p.components()
                .filter_map(|c| c.as_os_str().to_str())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();

    let file_stem = relative_path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("Failed to get file stem");
    let const_name = file_stem_to_const_name(file_stem);

    (components, const_name)
}

fn get_audio_duration_millis(path: &Path) -> i64 {
    use symphonia::core::io::MediaSourceStream;
    use symphonia::core::probe::Hint;

    let file = std::fs::File::open(path)
        .unwrap_or_else(|e| panic!("Failed to open audio file {:?}: {}", path, e));
    let mss = MediaSourceStream::new(Box::new(file), Default::default());

    let mut hint = Hint::new();
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        hint.with_extension(ext);
    }

    let probed = symphonia::default::get_probe()
        .format(&hint, mss, &Default::default(), &Default::default())
        .unwrap_or_else(|e| panic!("Failed to probe audio file {:?}: {}", path, e));

    let track = probed
        .format
        .default_track()
        .unwrap_or_else(|| panic!("No default track found in {:?}", path));

    let params = &track.codec_params;

    if let (Some(n_frames), Some(time_base)) = (params.n_frames, params.time_base) {
        let time = time_base.calc_time(n_frames);
        (time.seconds as f64 * 1000.0 + time.frac * 1000.0) as i64
    } else if let (Some(n_frames), Some(sample_rate)) = (params.n_frames, params.sample_rate) {
        (n_frames as f64 / sample_rate as f64 * 1000.0) as i64
    } else {
        panic!(
            "Cannot determine duration for {:?}: n_frames={:?}, time_base={:?}, sample_rate={:?}",
            path, params.n_frames, params.time_base, params.sample_rate
        );
    }
}

#[proc_macro]
pub fn register_assets(_input: TokenStream) -> TokenStream {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .expect("CARGO_MANIFEST_DIR environment variable not set");
    let asset_dir = PathBuf::from(&manifest_dir).join("asset");

    let mut root = ModuleNode::new();

    let mut image_files = collect_image_files(&asset_dir);
    image_files.sort();
    for (id, file_path) in image_files.iter().enumerate() {
        let (components, const_name) = path_to_parts(&asset_dir, file_path);
        root.add_image(&components, const_name, id);
    }

    let mut audio_files = collect_audio_files(&asset_dir);
    audio_files.sort();
    for (id, file_path) in audio_files.iter().enumerate() {
        let (components, const_name) = path_to_parts(&asset_dir, file_path);
        let duration_millis = get_audio_duration_millis(file_path);
        root.add_audio(&components, const_name, id, duration_millis);
    }

    let module_tree = root.to_tokens();

    let image_use = if root.has_images() {
        quote! { use super::Image; }
    } else {
        quote! {}
    };
    let audio_use = if root.has_audios() {
        quote! { use super::AudioAsset; use super::Duration; }
    } else {
        quote! {}
    };

    // Generate native asset initialization function
    let native_init = {
        let mut image_init_calls = Vec::new();
        for (id, file_path) in image_files.iter().enumerate() {
            let relative_path = file_path
                .strip_prefix(&asset_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            image_init_calls.push(quote! {
                register_image(#id, #relative_path);
            });
        }

        let mut audio_init_calls = Vec::new();
        for (id, file_path) in audio_files.iter().enumerate() {
            let relative_path = file_path
                .strip_prefix(&asset_dir)
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            audio_init_calls.push(quote! {
                register_audio(#id, #relative_path);
            });
        }

        quote! {
            pub fn init_native_assets() {
                unsafe extern "C" {
                    fn _register_image(image_id: usize, buffer_ptr: *const u8, buffer_len: usize);
                    fn _register_audio(audio_id: usize, buffer_ptr: *const u8, buffer_len: usize);
                }
                fn asset_base_dir() -> std::path::PathBuf {
                    std::env::current_exe()
                        .expect("Failed to get current exe path")
                        .parent()
                        .unwrap()
                        .join("asset")
                }
                fn register_image(id: usize, relative_path: &str) {
                    let path = asset_base_dir().join(relative_path);
                    let data = std::fs::read(&path)
                        .unwrap_or_else(|e| panic!("Failed to read asset {}: {e}", path.display()));
                    let leaked = Box::leak(data.into_boxed_slice());
                    unsafe { _register_image(id, leaked.as_ptr(), leaked.len()); }
                }
                fn register_audio(id: usize, relative_path: &str) {
                    let path = asset_base_dir().join(relative_path);
                    let data = std::fs::read(&path)
                        .unwrap_or_else(|e| panic!("Failed to read audio asset {}: {e}", path.display()));
                    let leaked = Box::leak(data.into_boxed_slice());
                    unsafe { _register_audio(id, leaked.as_ptr(), leaked.len()); }
                }
                #(#image_init_calls)*
                #(#audio_init_calls)*
            }
        }
    };

    let expanded = quote! {
        pub mod asset {
            #image_use
            #audio_use
            #module_tree
            #native_init
        }
    };

    TokenStream::from(expanded)
}
